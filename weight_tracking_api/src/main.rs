use std::env;

use color_eyre::Result;
use poem::{EndpointExt, listener::TcpListener, Route, Server};
use poem::middleware::{AddData, Cors};
use poem_openapi::OpenApiService;
use sqlx::PgPool;

use endpoints::Api;

mod endpoints;
mod models;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let db_url = &env::var("DATABASE_URL")?;
    println!("Connecting to database at {db_url}");
    let pool = PgPool::connect(db_url).await?;

    let host_url = env::var("HOST_URL")?;

    let api_service =
        OpenApiService::new(Api, "Weight Tracking", env!("CARGO_PKG_VERSION"))
            .server(env::var("DOCUMENTATION_TARGET_URL")?);
    let swagger = api_service.swagger_ui();
    let app = Route::new()
        .nest("/", api_service)
        .nest("/swagger", swagger)
        .with(AddData::new(pool))
        .with(Cors::new());

    println!("Running server on {host_url}");
    Server::new(TcpListener::bind(host_url))
        .run(app)
        .await?;

    Ok(())
}