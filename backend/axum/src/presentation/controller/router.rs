use axum::{routing::get, Router};

pub fn app() -> Router {
    let router: Router = Router::new()
        .route("/", get(|| async { "Hello, World! 1" }))
        .route("/test", get(|| async { "Hello, World! 2" }));
    return router;
}