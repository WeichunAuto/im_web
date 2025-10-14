use std::{ops::Deref, sync::Arc};

use crate::{handlers, AppConfig};
use axum::{routing::get, Router};

pub(crate) mod user;

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    pub(crate) inner: Arc<AppStateInner>,
}

#[derive(Debug)]
pub(crate) struct AppStateInner {
    pub(crate) config: AppConfig,
}

// 当我调用 state.config 时 相当于调用 state.inner.config
impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        AppState {
            inner: Arc::new(AppStateInner { config }),
        }
    }
}

/// Create and configure application routes.
///
/// - Registers `/` as the root route.
/// - Uses `.nest("/api", user::routes())` to mount all routes from the `user` module
///   under the `/api` path.
///   For example, if `user::routes()` defines `/login`, the full path will be `/api/login`.
///   Sets a `fallback` handler for unmatched routes.
pub fn create_routes(config: AppConfig) -> Router {
    let state = AppState::new(config);

    let api = Router::new()
        .route("/", get(handlers::index))
        .nest("/api", user::routes())
        .with_state(state)
        .fallback(handlers::fallback);

    api
}
