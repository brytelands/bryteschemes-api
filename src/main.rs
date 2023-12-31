use std::convert::Infallible;
use std::io::Read;
use std::sync::Arc;

use anyhow::Result as AnyResult;
use utoipa::{Modify, OpenApi};
use utoipa_swagger_ui::Config as SwaggerConfig;
use warp::{
    Filter,
    http::Uri,
    hyper::{Response, StatusCode},
    path::{FullPath, Tail}, Rejection, Reply,
};
use crate::config::Config;

mod handler;
mod config;

type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    env_logger::init();

    let api_doc_config = Arc::new(SwaggerConfig::from("/api-doc.json"));

    #[derive(OpenApi)]
    #[openapi(
        info(
            title = "BryteSchemes API",
            description = "The BryteSchemes API provides functionality to get account and event data.",
            version = "0.0.1"
        ),
        paths(handler::account_handler, handler::discriminator, handler::discriminator_offline, handler::account_schema_handler),
        tags(
        (name = "handler", description = "Account and Event API endpoints")
        )
    )]
    struct ApiDoc;

    let api_doc = warp::path("api-doc.json")
        .and(warp::get())
        .map(|| warp::reply::json(&ApiDoc::openapi()));

    let swagger_ui = warp::path("swagger-ui")
        .and(warp::get())
        .and(warp::path::full())
        .and(warp::path::tail())
        .and(warp::any().map(move || api_doc_config.clone()))
        .and_then(serve_swagger);

    let health_route = warp::path!("health").and_then(handler::health_handler);

    let account_route = warp::path("account")
        .and(warp::path::param())
        .and(warp::path::param())
        .and(warp::path::param())
        .and_then(handler::account_handler);

    let account_schema_route = warp::path("account-schema")
        .and(warp::path::param())
        .and(warp::path::param())
        .and(warp::path::param())
        .and_then(handler::account_schema_handler);

    let discriminator_offline_route = warp::path("discriminator-offline")
        .and(warp::path::param())
        .and(warp::path::param())
        .and_then(handler::discriminator_offline);

    let discriminator_route = warp::path("discriminator")
        .and(warp::path::param())
        .and(warp::path::param())
        .and_then(handler::discriminator);


    let routes = health_route
        .or(account_route)
        .or(account_schema_route)
        .or(discriminator_offline_route)
        .or(discriminator_route)
        .or(swagger_ui)
        .or(api_doc)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

fn with_value(value: String) -> impl Filter<Extract = (String,), Error = Infallible> + Clone {
    warp::any().map(move || value.clone())
}

fn with_config(value: Config) -> impl Filter<Extract = (Config,), Error = Infallible> + Clone {
    warp::any().map(move || value.clone())
}

async fn serve_swagger(
    full_path: FullPath,
    tail: Tail,
    config: Arc<SwaggerConfig<'static>>,
) -> AnyResult<Box<dyn Reply + 'static>, Rejection> {
    if full_path.as_str() == "/swagger-ui" {
        return Ok(Box::new(warp::redirect::found(Uri::from_static(
            "/swagger-ui/",
        ))));
    }

    let path = tail.as_str();
    match utoipa_swagger_ui::serve(path, config) {
        Ok(file) => {
            if let Some(file) = file {
                Ok(Box::new(
                    Response::builder()
                        .header("Content-Type", file.content_type)
                        .body(file.bytes),
                ))
            } else {
                Ok(Box::new(StatusCode::NOT_FOUND))
            }
        }
        Err(error) => Ok(Box::new(
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error.to_string()),
        )),
    }
}
