use std::sync::Arc;
use std::time::Duration;

use uuid::Uuid;

use crate::domain::*;
use crate::external::Notifier;
use crate::repository::FeedbackRepository;
use crate::storage::{ObjectStorage, validate_upload};

pub struct FeedbackService {
    repository: Arc<dyn FeedbackRepository>,
    storage: Arc<dyn ObjectStorage>,
    notifier: Arc<dyn Notifier>,
}

impl FeedbackService {
    pub fn new(
        repository: Arc<dyn FeedbackRepository>,
        storage: Arc<dyn ObjectStorage>,
        notifier: Arc<dyn Notifier>,
    ) -> Self {
        todo!("сохранить зависимости")
    }

    pub async fn create_public(
        &self,
        actor: CurrentUser,
        draft: NewFeedback,
    ) -> Result<CreationOutcome, AppError> {
        todo!("User + validate + INSERT + notify с явным notification_error")
    }

    pub async fn get_public(&self, actor: CurrentUser, id: Uuid) -> Result<Feedback, AppError> {
        todo!("User + только своё, чужое → NotFound")
    }

    pub async fn list_public(
        &self,
        actor: CurrentUser,
        filter: FeedbackFilter,
    ) -> Result<Vec<Feedback>, AppError> {
        todo!("User + author=actor.id + pagination")
    }

    pub async fn update_public(
        &self,
        actor: CurrentUser,
        id: Uuid,
        update: UpdateFeedback,
    ) -> Result<Feedback, AppError> {
        todo!("User/owner/New + validate + scoped UPDATE")
    }

    pub async fn delete_public(
        &self,
        actor: CurrentUser,
        id: Uuid,
    ) -> Result<DeleteOutcome, AppError> {
        todo!("User/owner/New + DELETE + best-effort S3 cleanup с ошибками в outcome")
    }

    pub async fn list_private(
        &self,
        actor: CurrentUser,
        filter: FeedbackFilter,
    ) -> Result<Vec<Feedback>, AppError> {
        todo!("staff + все фильтры + pagination")
    }

    pub async fn get_private(&self, actor: CurrentUser, id: Uuid) -> Result<Feedback, AppError> {
        todo!("staff + любой feedback")
    }

    pub async fn moderate_private(
        &self,
        actor: CurrentUser,
        id: Uuid,
        status: FeedbackStatus,
    ) -> Result<Feedback, AppError> {
        todo!("staff + workflow + UPDATE status")
    }

    pub async fn delete_private(
        &self,
        actor: CurrentUser,
        id: Uuid,
    ) -> Result<DeleteOutcome, AppError> {
        todo!("только Admin + DELETE + cleanup")
    }

    pub async fn upload_attachment_public(
        &self,
        actor: CurrentUser,
        feedback_id: Uuid,
        upload: AttachmentUpload,
    ) -> Result<Attachment, AppError> {
        todo!("owner/New + validate + S3 + metadata + compensation")
    }

    pub async fn list_attachments_public(
        &self,
        actor: CurrentUser,
        feedback_id: Uuid,
    ) -> Result<Vec<Attachment>, AppError> {
        todo!("owner + metadata")
    }

    pub async fn download_attachment_public(
        &self,
        actor: CurrentUser,
        feedback_id: Uuid,
        attachment_id: Uuid,
        ttl: Duration,
    ) -> Result<String, AppError> {
        todo!("owner + attachment принадлежит feedback + presign")
    }

    pub async fn delete_attachment_public(
        &self,
        actor: CurrentUser,
        feedback_id: Uuid,
        attachment_id: Uuid,
    ) -> Result<AttachmentDeleteOutcome, AppError> {
        todo!("owner/New + DB delete + S3 cleanup outcome")
    }

    pub async fn list_attachments_private(
        &self,
        actor: CurrentUser,
        feedback_id: Uuid,
    ) -> Result<Vec<Attachment>, AppError> {
        todo!("staff + metadata")
    }

    pub async fn download_attachment_private(
        &self,
        actor: CurrentUser,
        feedback_id: Uuid,
        attachment_id: Uuid,
        ttl: Duration,
    ) -> Result<String, AppError> {
        todo!("staff + presign")
    }

    pub async fn delete_attachment_private(
        &self,
        actor: CurrentUser,
        feedback_id: Uuid,
        attachment_id: Uuid,
    ) -> Result<AttachmentDeleteOutcome, AppError> {
        todo!("Admin + DB delete + S3 cleanup outcome")
    }
}
