# Домашнее задание: роли, владельцы и routers

```rust
pub fn FeedbackStatus::can_transition_to(self, next: Self) -> bool;
pub fn require_user(actor: &CurrentUser) -> Result<(), AuthzError>;
pub fn require_moderator(actor: &CurrentUser) -> Result<(), AuthzError>;
pub fn authorize_view(actor: &CurrentUser, feedback: &Feedback) -> Result<(), AuthzError>;
pub fn authorize_user_update(actor: &CurrentUser, feedback: &Feedback)
    -> Result<(), AuthzError>;
pub fn authorize_moderation(actor: &CurrentUser, from: FeedbackStatus, to: FeedbackStatus)
    -> Result<(), AuthzError>;
pub fn AppState::new(feedback: Feedback) -> Self;
pub async fn public_feedback(...) -> Result<Json<Feedback>, ApiError>;
pub async fn private_feedback(...) -> Result<Json<Feedback>, ApiError>;
pub fn public_app(state: AppState) -> Router;
pub fn private_app(state: AppState) -> Router;
```

## Правила

- `require_user`: только User; `require_moderator`: Moderator или Admin. Неподходящая роль → `Forbidden`.
- `authorize_view`: User может читать только своё, чужое → `NotFound`; Moderator/Admin читают любое.
- `authorize_user_update`: сначала require_user, затем чужой id → `NotFound`, статус не New → `Forbidden`.
- `authorize_moderation`: require_moderator, затем workflow: same разрешён, New→InReview/Rejected, InReview→Resolved/Rejected; остальное → `InvalidTransition`.
- В handlers несовпадение path id с id записи state → `NotFound`. Public handler сначала require_user, private — require_moderator, затем id и authorize_view.
- `public_app` и `private_app` регистрируют `GET /feedback/{id}` и передают state. Extension actor в тестах моделирует JWT middleware.
- Готовый `ApiError` отображает NotFound→404, Forbidden→403, InvalidTransition→409.

Не добавляйте возможность читать чужие записи в public API ради удобства модератора: для этого существует private router.

## Проверка

```bash
cargo test -p web-08-authorization
cargo clippy -p web-08-authorization --all-targets -- -D warnings
```
