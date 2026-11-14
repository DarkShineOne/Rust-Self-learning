#![allow(dead_code)]

use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use chrono::Utc;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use uuid::Uuid;
use web_14_final_feedback::api::AppState;
use web_14_final_feedback::auth::{AuthService, JwtConfig};
use web_14_final_feedback::domain::*;
use web_14_final_feedback::external::Notifier;
use web_14_final_feedback::repository::{AuthRepository, FeedbackRepository, RotationResult};
use web_14_final_feedback::service::FeedbackService;
use web_14_final_feedback::storage::ObjectStorage;

#[derive(Default)]
pub struct MockRepository {
    pub users: Mutex<Vec<User>>,
    pub refresh_tokens: Mutex<Vec<RefreshToken>>,
    pub feedback: Mutex<Vec<Feedback>>,
    pub attachments: Mutex<Vec<Attachment>>,
    pub fail_attachment_save: bool,
}

impl MockRepository {
    pub fn seed_user(&self, role: Role, password_hash: String) -> User {
        let user = User {
            id: Uuid::new_v4(),
            email: format!("{}@example.com", Uuid::new_v4()),
            password_hash,
            role,
        };
        self.users.lock().unwrap().push(user.clone());
        user
    }

    pub fn seed_feedback(&self, author_id: Uuid, status: FeedbackStatus) -> Feedback {
        let feedback = Feedback {
            id: Uuid::new_v4(),
            author_id,
            title: "Seeded feedback".into(),
            body: "Seeded body".into(),
            rating: 5,
            category: Category::Bug,
            status,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.feedback.lock().unwrap().push(feedback.clone());
        feedback
    }
}

#[async_trait]
impl AuthRepository for MockRepository {
    async fn user_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        Ok(self
            .users
            .lock()
            .unwrap()
            .iter()
            .find(|user| user.email == email)
            .cloned())
    }

    async fn user_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        Ok(self
            .users
            .lock()
            .unwrap()
            .iter()
            .find(|user| user.id == id)
            .cloned())
    }

    async fn create_user(&self, email: &str, password_hash: &str) -> Result<User, AppError> {
        let mut users = self.users.lock().unwrap();
        if users.iter().any(|user| user.email == email) {
            return Err(AppError::Conflict("email is already registered".into()));
        }
        let user = User {
            id: Uuid::new_v4(),
            email: email.into(),
            password_hash: password_hash.into(),
            role: Role::User,
        };
        users.push(user.clone());
        Ok(user)
    }

    async fn refresh_by_hash(&self, hash: &str) -> Result<Option<RefreshToken>, AppError> {
        Ok(self
            .refresh_tokens
            .lock()
            .unwrap()
            .iter()
            .find(|token| token.token_hash == hash)
            .cloned())
    }

    async fn insert_refresh(&self, token: NewRefreshToken) -> Result<(), AppError> {
        self.refresh_tokens.lock().unwrap().push(RefreshToken {
            token_hash: token.token_hash,
            user_id: token.user_id,
            expires_at: token.expires_at,
            revoked: false,
        });
        Ok(())
    }

    async fn revoke_refresh(&self, hash: &str) -> Result<(), AppError> {
        if let Some(token) = self
            .refresh_tokens
            .lock()
            .unwrap()
            .iter_mut()
            .find(|token| token.token_hash == hash)
        {
            token.revoked = true;
        }
        Ok(())
    }

    async fn rotate_refresh(
        &self,
        old_hash: &str,
        new_token: NewRefreshToken,
        now: i64,
    ) -> Result<RotationResult, AppError> {
        let mut tokens = self.refresh_tokens.lock().unwrap();
        let Some(index) = tokens.iter().position(|token| token.token_hash == old_hash) else {
            return Ok(RotationResult::NotFound);
        };
        if tokens[index].revoked {
            return Ok(RotationResult::Reused);
        }
        if now >= tokens[index].expires_at {
            return Ok(RotationResult::Expired);
        }
        tokens[index].revoked = true;
        tokens.push(RefreshToken {
            token_hash: new_token.token_hash,
            user_id: new_token.user_id,
            expires_at: new_token.expires_at,
            revoked: false,
        });
        Ok(RotationResult::Success)
    }
}

#[async_trait]
impl FeedbackRepository for MockRepository {
    async fn create_feedback(
        &self,
        author_id: Uuid,
        draft: NewFeedback,
    ) -> Result<Feedback, AppError> {
        let feedback = Feedback {
            id: Uuid::new_v4(),
            author_id,
            title: draft.title,
            body: draft.body,
            rating: draft.rating,
            category: draft.category,
            status: FeedbackStatus::New,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.feedback.lock().unwrap().push(feedback.clone());
        Ok(feedback)
    }

    async fn feedback_by_id(&self, id: Uuid) -> Result<Option<Feedback>, AppError> {
        Ok(self
            .feedback
            .lock()
            .unwrap()
            .iter()
            .find(|feedback| feedback.id == id)
            .cloned())
    }

    async fn list_feedback(
        &self,
        author_id: Option<Uuid>,
        filter: FeedbackFilter,
    ) -> Result<Vec<Feedback>, AppError> {
        if !(1..=100).contains(&filter.limit) || filter.offset < 0 {
            return Err(AppError::Validation("invalid pagination".into()));
        }
        let mut items: Vec<Feedback> = self
            .feedback
            .lock()
            .unwrap()
            .iter()
            .filter(|feedback| author_id.is_none_or(|id| feedback.author_id == id))
            .filter(|feedback| filter.status.is_none_or(|status| feedback.status == status))
            .filter(|feedback| {
                filter
                    .category
                    .is_none_or(|category| feedback.category == category)
            })
            .cloned()
            .collect();
        items.sort_by(|left, right| {
            right
                .created_at
                .cmp(&left.created_at)
                .then_with(|| right.id.cmp(&left.id))
        });
        Ok(items
            .into_iter()
            .skip(filter.offset as usize)
            .take(filter.limit as usize)
            .collect())
    }

    async fn update_owned_feedback(
        &self,
        id: Uuid,
        author_id: Uuid,
        update: UpdateFeedback,
    ) -> Result<Option<Feedback>, AppError> {
        let mut items = self.feedback.lock().unwrap();
        let Some(feedback) = items.iter_mut().find(|feedback| {
            feedback.id == id
                && feedback.author_id == author_id
                && feedback.status == FeedbackStatus::New
        }) else {
            return Ok(None);
        };
        if let Some(title) = update.title {
            feedback.title = title;
        }
        if let Some(body) = update.body {
            feedback.body = body;
        }
        if let Some(rating) = update.rating {
            feedback.rating = rating;
        }
        if let Some(category) = update.category {
            feedback.category = category;
        }
        feedback.updated_at = Utc::now();
        Ok(Some(feedback.clone()))
    }

    async fn update_feedback_status(
        &self,
        id: Uuid,
        status: FeedbackStatus,
    ) -> Result<Option<Feedback>, AppError> {
        let mut items = self.feedback.lock().unwrap();
        let Some(feedback) = items.iter_mut().find(|feedback| feedback.id == id) else {
            return Ok(None);
        };
        if !feedback.status.can_transition_to(status) {
            return Err(AppError::Conflict("invalid status transition".into()));
        }
        feedback.status = status;
        feedback.updated_at = Utc::now();
        Ok(Some(feedback.clone()))
    }

    async fn delete_owned_feedback(
        &self,
        id: Uuid,
        author_id: Uuid,
    ) -> Result<Option<Feedback>, AppError> {
        let mut items = self.feedback.lock().unwrap();
        let Some(index) = items.iter().position(|feedback| {
            feedback.id == id
                && feedback.author_id == author_id
                && feedback.status == FeedbackStatus::New
        }) else {
            return Ok(None);
        };
        Ok(Some(items.remove(index)))
    }

    async fn delete_feedback(&self, id: Uuid) -> Result<Option<Feedback>, AppError> {
        let mut items = self.feedback.lock().unwrap();
        let Some(index) = items.iter().position(|feedback| feedback.id == id) else {
            return Ok(None);
        };
        Ok(Some(items.remove(index)))
    }

    async fn save_attachment(&self, attachment: Attachment) -> Result<(), AppError> {
        if self.fail_attachment_save {
            return Err(AppError::ServiceUnavailable);
        }
        self.attachments.lock().unwrap().push(attachment);
        Ok(())
    }

    async fn attachment_by_id(&self, id: Uuid) -> Result<Option<Attachment>, AppError> {
        Ok(self
            .attachments
            .lock()
            .unwrap()
            .iter()
            .find(|attachment| attachment.id == id)
            .cloned())
    }

    async fn list_attachments(&self, feedback_id: Uuid) -> Result<Vec<Attachment>, AppError> {
        Ok(self
            .attachments
            .lock()
            .unwrap()
            .iter()
            .filter(|attachment| attachment.feedback_id == feedback_id)
            .cloned()
            .collect())
    }

    async fn delete_attachment(&self, id: Uuid) -> Result<Option<Attachment>, AppError> {
        let mut attachments = self.attachments.lock().unwrap();
        let Some(index) = attachments
            .iter()
            .position(|attachment| attachment.id == id)
        else {
            return Ok(None);
        };
        Ok(Some(attachments.remove(index)))
    }
}

#[derive(Default)]
pub struct MockStorage {
    pub fail_put: bool,
    pub fail_delete: bool,
    pub deletes: Mutex<Vec<String>>,
}

#[async_trait]
impl ObjectStorage for MockStorage {
    async fn put(
        &self,
        feedback_id: Uuid,
        attachment_id: Uuid,
        upload: AttachmentUpload,
    ) -> Result<Attachment, AppError> {
        if self.fail_put {
            return Err(AppError::ServiceUnavailable);
        }
        Ok(Attachment {
            id: attachment_id,
            feedback_id,
            key: format!("feedback/{feedback_id}/{attachment_id}/note.txt"),
            original_name: upload.file_name.trim().into(),
            content_type: upload.content_type,
            size_bytes: upload.bytes.len() as i64,
        })
    }

    async fn download_url(&self, key: &str, _ttl: Duration) -> Result<String, AppError> {
        Ok(format!("https://s3.test/{key}"))
    }

    async fn delete(&self, key: &str) -> Result<(), AppError> {
        self.deletes.lock().unwrap().push(key.into());
        if self.fail_delete {
            Err(AppError::ServiceUnavailable)
        } else {
            Ok(())
        }
    }
}

#[derive(Default)]
pub struct MockNotifier {
    pub fail: bool,
    pub calls: Mutex<Vec<Notification>>,
}

#[async_trait]
impl Notifier for MockNotifier {
    async fn notify(&self, notification: Notification) -> Result<(), AppError> {
        self.calls.lock().unwrap().push(notification);
        if self.fail {
            Err(AppError::ServiceUnavailable)
        } else {
            Ok(())
        }
    }
}

pub struct TestContext {
    pub state: AppState,
    pub repository: Arc<MockRepository>,
    pub storage: Arc<MockStorage>,
    pub notifier: Arc<MockNotifier>,
}

pub fn jwt_config() -> JwtConfig {
    JwtConfig {
        secret: "s".repeat(32),
        issuer: "feedback-service".into(),
        audience: "feedback-api".into(),
        access_ttl_seconds: 900,
        refresh_ttl_seconds: 3600,
    }
}

pub fn context() -> TestContext {
    context_with(
        Arc::new(MockRepository::default()),
        Arc::new(MockStorage::default()),
        Arc::new(MockNotifier::default()),
    )
}

pub fn context_with(
    repository: Arc<MockRepository>,
    storage: Arc<MockStorage>,
    notifier: Arc<MockNotifier>,
) -> TestContext {
    let auth = Arc::new(AuthService::new(repository.clone(), jwt_config()).unwrap());
    let feedback = Arc::new(FeedbackService::new(
        repository.clone(),
        storage.clone(),
        notifier.clone(),
    ));
    let state = AppState::new(auth, feedback);
    TestContext {
        state,
        repository,
        storage,
        notifier,
    }
}

pub fn actor(role: Role) -> CurrentUser {
    CurrentUser {
        id: Uuid::new_v4(),
        role,
    }
}

pub fn draft() -> NewFeedback {
    NewFeedback {
        title: "  Title  ".into(),
        body: "  Body  ".into(),
        rating: 5,
        category: Category::Idea,
    }
}

pub fn upload() -> AttachmentUpload {
    AttachmentUpload {
        file_name: "note.txt".into(),
        content_type: "text/plain".into(),
        bytes: bytes::Bytes::from_static(b"attachment"),
        max_bytes: 1024,
    }
}

pub fn access_token(user: &User) -> String {
    let now = Utc::now().timestamp();
    let claims = Claims {
        sub: user.id.to_string(),
        role: user.role,
        iss: "feedback-service".into(),
        aud: "feedback-api".into(),
        exp: (now + 900) as usize,
        iat: (now - 1) as usize,
        jti: Uuid::new_v4().to_string(),
        token_type: "access".into(),
    };
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(jwt_config().secret.as_bytes()),
    )
    .unwrap()
}
