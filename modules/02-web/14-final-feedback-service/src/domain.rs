use bytes::Bytes;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    User,
    Moderator,
    Admin,
}

impl Role {
    pub fn is_staff(self) -> bool {
        todo!("Moderator или Admin")
    }

    pub fn is_admin(self) -> bool {
        todo!("только Admin")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Bug,
    Idea,
    Question,
    Praise,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum FeedbackStatus {
    New,
    InReview,
    Resolved,
    Rejected,
}

impl FeedbackStatus {
    pub fn can_transition_to(self, next: Self) -> bool {
        todo!("same, New→InReview/Rejected, InReview→Resolved/Rejected")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role: Role,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub role: Role,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            role: user.role,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentUser {
    pub id: Uuid,
    pub role: Role,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: Role,
    pub iss: String,
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
    pub jti: String,
    pub token_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub access_expires_at: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefreshToken {
    pub token_hash: String,
    pub user_id: Uuid,
    pub expires_at: i64,
    pub revoked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewRefreshToken {
    pub token_hash: String,
    pub user_id: Uuid,
    pub expires_at: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Feedback {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub body: String,
    pub rating: u8,
    pub category: Category,
    pub status: FeedbackStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct NewFeedback {
    pub title: String,
    pub body: String,
    pub rating: u8,
    pub category: Category,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct UpdateFeedback {
    pub title: Option<String>,
    pub body: Option<String>,
    pub rating: Option<u8>,
    pub category: Option<Category>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ModerateFeedback {
    pub status: FeedbackStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, ToSchema)]
#[serde(default)]
pub struct FeedbackFilter {
    pub status: Option<FeedbackStatus>,
    pub category: Option<Category>,
    pub author_id: Option<Uuid>,
    pub limit: i64,
    pub offset: i64,
}

impl Default for FeedbackFilter {
    fn default() -> Self {
        Self {
            status: None,
            category: None,
            author_id: None,
            limit: 20,
            offset: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Attachment {
    pub id: Uuid,
    pub feedback_id: Uuid,
    pub key: String,
    pub original_name: String,
    pub content_type: String,
    pub size_bytes: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachmentUpload {
    pub file_name: String,
    pub content_type: String,
    pub bytes: Bytes,
    pub max_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Notification {
    pub feedback_id: Uuid,
    pub title: String,
    pub category: Category,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct CreationOutcome {
    pub feedback: Feedback,
    pub notification_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct DeleteOutcome {
    pub feedback: Feedback,
    pub attachment_cleanup_errors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct AttachmentDeleteOutcome {
    pub attachment: Attachment,
    pub cleanup_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AppError {
    #[error("{0}")]
    Validation(String),
    #[error("authentication required")]
    Unauthorized,
    #[error("access denied")]
    Forbidden,
    #[error("resource not found")]
    NotFound,
    #[error("{0}")]
    Conflict(String),
    #[error("request body too large")]
    PayloadTooLarge,
    #[error("upstream service timed out")]
    Timeout,
    #[error("service temporarily unavailable")]
    ServiceUnavailable,
    #[error("internal server error")]
    Internal,
}
