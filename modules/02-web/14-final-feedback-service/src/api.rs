use std::sync::Arc;
use std::time::Duration;

use axum::extract::{FromRequestParts, Multipart, Path, Query, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::request::Parts;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

use crate::auth::AuthService;
use crate::domain::*;
use crate::service::FeedbackService;

#[derive(Clone)]
pub struct AppState {
    pub auth: Arc<AuthService>,
    pub feedback: Arc<FeedbackService>,
}

impl AppState {
    pub fn new(auth: Arc<AuthService>, feedback: Arc<FeedbackService>) -> Self {
        todo!("сохранить сервисы")
    }
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DownloadResponse {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemDetails {
    #[serde(rename = "type")]
    pub problem_type: String,
    pub title: String,
    pub status: u16,
    pub detail: String,
    pub instance: String,
    pub request_id: String,
}

pub fn status_for_error(error: &AppError) -> StatusCode {
    todo!("Validation/Unauthorized/Forbidden/NotFound/Conflict/413/Timeout/Unavailable/Internal")
}

#[derive(Debug)]
pub struct ApiError {
    pub problem: Box<ProblemDetails>,
}

impl ApiError {
    pub fn new(error: AppError, headers: &HeaderMap, instance: &str) -> Self {
        todo!("безопасный Problem Details + request_id")
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status =
            StatusCode::from_u16(self.problem.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (
            status,
            [(
                CONTENT_TYPE,
                HeaderValue::from_static("application/problem+json"),
            )],
            Json(self.problem),
        )
            .into_response()
    }
}

pub fn request_id(headers: &HeaderMap) -> String {
    todo!("сохранить валидный x-request-id или создать UUID")
}

impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        todo!("Bearer → auth.validate_access(Utc::now) → CurrentUser; ошибка 401")
    }
}

#[utoipa::path(post, path = "/api/public/v1/auth/register", request_body = RegisterRequest,
    responses((status = 201, body = UserResponse), (status = 422, description = "Validation error")))]
pub async fn register(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<UserResponse>), ApiError> {
    todo!("auth.register, 201")
}

#[utoipa::path(post, path = "/api/public/v1/auth/login", request_body = LoginRequest,
    responses((status = 200, body = TokenPair), (status = 401, description = "Invalid credentials")))]
pub async fn login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<LoginRequest>,
) -> Result<Json<TokenPair>, ApiError> {
    todo!("auth.login с Utc::now")
}

#[utoipa::path(post, path = "/api/public/v1/auth/refresh", request_body = RefreshRequest,
    responses((status = 200, body = TokenPair), (status = 401, description = "Invalid refresh")))]
pub async fn refresh(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<RefreshRequest>,
) -> Result<Json<TokenPair>, ApiError> {
    todo!("auth.refresh с Utc::now")
}

#[utoipa::path(post, path = "/api/public/v1/auth/logout", request_body = RefreshRequest,
    responses((status = 204, description = "Logged out")))]
pub async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<RefreshRequest>,
) -> Result<StatusCode, ApiError> {
    todo!("auth.logout, 204")
}

#[utoipa::path(post, path = "/api/public/v1/feedback", request_body = NewFeedback,
    responses((status = 201, body = CreationOutcome), (status = 422, description = "Validation error")))]
pub async fn create_feedback_handler(
    State(state): State<AppState>,
    actor: CurrentUser,
    headers: HeaderMap,
    Json(draft): Json<NewFeedback>,
) -> Result<(StatusCode, Json<CreationOutcome>), ApiError> {
    todo!("service.create_public, 201")
}

#[utoipa::path(get, path = "/api/public/v1/feedback", params(("status" = Option<FeedbackStatus>, Query),
    ("category" = Option<Category>, Query), ("limit" = i64, Query), ("offset" = i64, Query)),
    responses((status = 200, body = [Feedback])))]
pub async fn list_feedback_handler(
    State(state): State<AppState>,
    actor: CurrentUser,
    headers: HeaderMap,
    Query(filter): Query<FeedbackFilter>,
) -> Result<Json<Vec<Feedback>>, ApiError> {
    todo!("service.list_public; author_id из query игнорируется")
}

#[utoipa::path(get, path = "/api/public/v1/feedback/{id}", params(("id" = Uuid, Path)),
    responses((status = 200, body = Feedback), (status = 404, description = "Not found")))]
pub async fn get_feedback_handler(
    State(state): State<AppState>,
    actor: CurrentUser,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<Feedback>, ApiError> {
    todo!("service.get_public")
}

#[utoipa::path(patch, path = "/api/public/v1/feedback/{id}", params(("id" = Uuid, Path)),
    request_body = UpdateFeedback, responses((status = 200, body = Feedback)))]
pub async fn update_feedback_handler(
    State(state): State<AppState>,
    actor: CurrentUser,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(update): Json<UpdateFeedback>,
) -> Result<Json<Feedback>, ApiError> {
    todo!("service.update_public")
}

#[utoipa::path(delete, path = "/api/public/v1/feedback/{id}", params(("id" = Uuid, Path)),
    responses((status = 200, body = DeleteOutcome)))]
pub async fn delete_feedback_handler(
    State(state): State<AppState>,
    actor: CurrentUser,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<DeleteOutcome>, ApiError> {
    todo!("service.delete_public")
}

pub async fn upload_attachment_handler(
    State(state): State<AppState>,
    actor: CurrentUser,
    headers: HeaderMap,
    Path(feedback_id): Path<Uuid>,
    multipart: Multipart,
) -> Result<(StatusCode, Json<Attachment>), ApiError> {
    todo!("multipart file с limit → AttachmentUpload → service.upload_attachment_public, 201")
}

pub async fn list_attachments_handler(
    State(state): State<AppState>,
    actor: CurrentUser,
    headers: HeaderMap,
    Path(feedback_id): Path<Uuid>,
) -> Result<Json<Vec<Attachment>>, ApiError> {
    todo!("service.list_attachments_public")
}

pub async fn download_attachment_handler(
    State(state): State<AppState>,
    actor: CurrentUser,
    headers: HeaderMap,
    Path((feedback_id, attachment_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<DownloadResponse>, ApiError> {
    todo!("service.download_attachment_public с TTL 60 секунд")
}

pub async fn delete_attachment_handler(
    State(state): State<AppState>,
    actor: CurrentUser,
    headers: HeaderMap,
    Path((feedback_id, attachment_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<AttachmentDeleteOutcome>, ApiError> {
    todo!("service.delete_attachment_public")
}

#[utoipa::path(get, path = "/api/private/v1/feedback", params(("status" = Option<FeedbackStatus>, Query),
    ("category" = Option<Category>, Query), ("author_id" = Option<Uuid>, Query),
    ("limit" = i64, Query), ("offset" = i64, Query)), responses((status = 200, body = [Feedback])))]
pub async fn list_private_feedback_handler(
    State(state): State<AppState>,
    actor: CurrentUser,
    headers: HeaderMap,
    Query(filter): Query<FeedbackFilter>,
) -> Result<Json<Vec<Feedback>>, ApiError> {
    todo!("service.list_private")
}

#[utoipa::path(get, path = "/api/private/v1/feedback/{id}", params(("id" = Uuid, Path)),
    responses((status = 200, body = Feedback), (status = 403, description = "Forbidden")))]
pub async fn get_private_feedback_handler(
    State(state): State<AppState>,
    actor: CurrentUser,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<Feedback>, ApiError> {
    todo!("service.get_private")
}

#[utoipa::path(patch, path = "/api/private/v1/feedback/{id}", params(("id" = Uuid, Path)),
    request_body = ModerateFeedback, responses((status = 200, body = Feedback), (status = 409, description = "Invalid transition")))]
pub async fn moderate_feedback_handler(
    State(state): State<AppState>,
    actor: CurrentUser,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(update): Json<ModerateFeedback>,
) -> Result<Json<Feedback>, ApiError> {
    todo!("service.moderate_private")
}

#[utoipa::path(delete, path = "/api/private/v1/feedback/{id}", params(("id" = Uuid, Path)),
    responses((status = 200, body = DeleteOutcome), (status = 403, description = "Admin only")))]
pub async fn delete_private_feedback_handler(
    State(state): State<AppState>,
    actor: CurrentUser,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<DeleteOutcome>, ApiError> {
    todo!("service.delete_private")
}

pub async fn list_private_attachments_handler(
    State(state): State<AppState>,
    actor: CurrentUser,
    headers: HeaderMap,
    Path(feedback_id): Path<Uuid>,
) -> Result<Json<Vec<Attachment>>, ApiError> {
    todo!("service.list_attachments_private")
}

pub async fn download_private_attachment_handler(
    State(state): State<AppState>,
    actor: CurrentUser,
    headers: HeaderMap,
    Path((feedback_id, attachment_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<DownloadResponse>, ApiError> {
    todo!("service.download_attachment_private с TTL 60 секунд")
}

pub async fn delete_private_attachment_handler(
    State(state): State<AppState>,
    actor: CurrentUser,
    headers: HeaderMap,
    Path((feedback_id, attachment_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<AttachmentDeleteOutcome>, ApiError> {
    todo!("service.delete_attachment_private")
}

#[derive(OpenApi)]
#[openapi(
    info(title = "Feedback Public API", version = "0.1.0"),
    paths(
        register,
        login,
        refresh,
        logout,
        create_feedback_handler,
        list_feedback_handler,
        get_feedback_handler,
        update_feedback_handler,
        delete_feedback_handler
    ),
    components(schemas(
        RegisterRequest,
        LoginRequest,
        RefreshRequest,
        UserResponse,
        TokenPair,
        NewFeedback,
        UpdateFeedback,
        Feedback,
        CreationOutcome,
        DeleteOutcome
    ))
)]
pub struct PublicApiDoc;

#[derive(OpenApi)]
#[openapi(
    info(title = "Feedback Private API", version = "0.1.0"),
    paths(
        list_private_feedback_handler,
        get_private_feedback_handler,
        moderate_feedback_handler,
        delete_private_feedback_handler
    ),
    components(schemas(ModerateFeedback, Feedback, DeleteOutcome))
)]
pub struct PrivateApiDoc;

pub fn public_openapi() -> utoipa::openapi::OpenApi {
    todo!("PublicApiDoc::openapi")
}

pub fn private_openapi() -> utoipa::openapi::OpenApi {
    todo!("PrivateApiDoc::openapi")
}

pub async fn public_docs() -> Json<utoipa::openapi::OpenApi> {
    todo!("public_openapi")
}

pub async fn private_docs(actor: CurrentUser) -> Result<Json<utoipa::openapi::OpenApi>, ApiError> {
    todo!("staff only + private_openapi")
}

pub async fn live() -> Json<serde_json::Value> {
    todo!("{{\"status\":\"ok\"}}")
}

pub async fn ready(State(state): State<AppState>) -> Json<serde_json::Value> {
    todo!("проверить обязательные зависимости без утечки ошибок")
}

/// Собирает пользовательский Router под /api/public/v1.
pub fn public_router(state: AppState) -> Router {
    todo!("auth + feedback CRUD + attachments + public openapi")
}

/// Собирает модераторский Router под /api/private/v1; каждый вызов всё равно проверяет роль.
pub fn private_router(state: AppState) -> Router {
    todo!("moderation + attachments + private openapi")
}

/// Объединяет routers, health и production middleware.
pub fn app(state: AppState) -> Router {
    todo!("public_router + private_router + live/ready + request ID/tracing/body limits")
}
