use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationResult {
    Success,
    NotFound,
    Reused,
    Expired,
}

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn user_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn user_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
    async fn create_user(&self, email: &str, password_hash: &str) -> Result<User, AppError>;
    async fn refresh_by_hash(&self, hash: &str) -> Result<Option<RefreshToken>, AppError>;
    async fn insert_refresh(&self, token: NewRefreshToken) -> Result<(), AppError>;
    async fn revoke_refresh(&self, hash: &str) -> Result<(), AppError>;
    async fn rotate_refresh(
        &self,
        old_hash: &str,
        new_token: NewRefreshToken,
        now: i64,
    ) -> Result<RotationResult, AppError>;
}

#[async_trait]
pub trait FeedbackRepository: Send + Sync {
    async fn create_feedback(
        &self,
        author_id: Uuid,
        draft: NewFeedback,
    ) -> Result<Feedback, AppError>;
    async fn feedback_by_id(&self, id: Uuid) -> Result<Option<Feedback>, AppError>;
    async fn list_feedback(
        &self,
        author_id: Option<Uuid>,
        filter: FeedbackFilter,
    ) -> Result<Vec<Feedback>, AppError>;
    async fn update_owned_feedback(
        &self,
        id: Uuid,
        author_id: Uuid,
        update: UpdateFeedback,
    ) -> Result<Option<Feedback>, AppError>;
    async fn update_feedback_status(
        &self,
        id: Uuid,
        status: FeedbackStatus,
    ) -> Result<Option<Feedback>, AppError>;
    async fn delete_owned_feedback(
        &self,
        id: Uuid,
        author_id: Uuid,
    ) -> Result<Option<Feedback>, AppError>;
    async fn delete_feedback(&self, id: Uuid) -> Result<Option<Feedback>, AppError>;
    async fn save_attachment(&self, attachment: Attachment) -> Result<(), AppError>;
    async fn attachment_by_id(&self, id: Uuid) -> Result<Option<Attachment>, AppError>;
    async fn list_attachments(&self, feedback_id: Uuid) -> Result<Vec<Attachment>, AppError>;
    async fn delete_attachment(&self, id: Uuid) -> Result<Option<Attachment>, AppError>;
}

#[derive(Debug, Clone)]
pub struct PgRepository {
    pool: PgPool,
}

impl PgRepository {
    pub fn new(pool: PgPool) -> Self {
        todo!("сохранить PgPool")
    }
}

#[async_trait]
impl AuthRepository for PgRepository {
    async fn user_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        todo!("SELECT users по нормализованному email")
    }
    async fn user_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        todo!("SELECT users по UUID")
    }
    async fn create_user(&self, email: &str, password_hash: &str) -> Result<User, AppError> {
        todo!("INSERT role=user, unique email → Conflict")
    }
    async fn refresh_by_hash(&self, hash: &str) -> Result<Option<RefreshToken>, AppError> {
        todo!("SELECT refresh_tokens по hash")
    }
    async fn insert_refresh(&self, token: NewRefreshToken) -> Result<(), AppError> {
        todo!("INSERT refresh token hash")
    }
    async fn revoke_refresh(&self, hash: &str) -> Result<(), AppError> {
        todo!("идемпотентный UPDATE revoked=true")
    }
    async fn rotate_refresh(
        &self,
        old_hash: &str,
        new_token: NewRefreshToken,
        now: i64,
    ) -> Result<RotationResult, AppError> {
        todo!("транзакция FOR UPDATE: проверить, revoke старого, insert нового")
    }
}

#[async_trait]
impl FeedbackRepository for PgRepository {
    async fn create_feedback(
        &self,
        author_id: Uuid,
        draft: NewFeedback,
    ) -> Result<Feedback, AppError> {
        todo!("INSERT feedback со статусом new и timestamps")
    }
    async fn feedback_by_id(&self, id: Uuid) -> Result<Option<Feedback>, AppError> {
        todo!("SELECT feedback по UUID")
    }
    async fn list_feedback(
        &self,
        author_id: Option<Uuid>,
        filter: FeedbackFilter,
    ) -> Result<Vec<Feedback>, AppError> {
        todo!("filters + ORDER BY created_at,id + LIMIT/OFFSET")
    }
    async fn update_owned_feedback(
        &self,
        id: Uuid,
        author_id: Uuid,
        update: UpdateFeedback,
    ) -> Result<Option<Feedback>, AppError> {
        todo!("UPDATE WHERE id/author_id/status=new")
    }
    async fn update_feedback_status(
        &self,
        id: Uuid,
        status: FeedbackStatus,
    ) -> Result<Option<Feedback>, AppError> {
        todo!("UPDATE status с проверкой workflow в transaction")
    }
    async fn delete_owned_feedback(
        &self,
        id: Uuid,
        author_id: Uuid,
    ) -> Result<Option<Feedback>, AppError> {
        todo!("DELETE WHERE id/author_id/status=new")
    }
    async fn delete_feedback(&self, id: Uuid) -> Result<Option<Feedback>, AppError> {
        todo!("DELETE admin и attachments metadata в transaction")
    }
    async fn save_attachment(&self, attachment: Attachment) -> Result<(), AppError> {
        todo!("INSERT metadata")
    }
    async fn attachment_by_id(&self, id: Uuid) -> Result<Option<Attachment>, AppError> {
        todo!("SELECT attachment")
    }
    async fn list_attachments(&self, feedback_id: Uuid) -> Result<Vec<Attachment>, AppError> {
        todo!("SELECT attachments по feedback_id")
    }
    async fn delete_attachment(&self, id: Uuid) -> Result<Option<Attachment>, AppError> {
        todo!("DELETE attachment RETURNING")
    }
}
