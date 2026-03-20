use axum::response::IntoResponse;
use axum_template::RenderHtml;

use crate::incoming::http::AppEngine;

#[derive(Debug, serde::Serialize)]
struct Greeting {}

pub async fn greeting(engine: AppEngine) -> impl IntoResponse {
    let ctx = Greeting {};

    RenderHtml("greeting.html", engine, ctx)
}
