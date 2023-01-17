use axum::{
    extract::Path,
    http::{HeaderValue, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use hyper::{header, Method};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower::{Service, ServiceBuilder, ServiceExt};
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
pub struct Metadata {
    pub name: String,
    pub symbol: Option<String>,
    pub description: String,
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
    pub attributes: Vec<Attribute>,
    pub properties: Property,
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
pub struct Property {
    pub files: Vec<FileAttr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creators: Option<Vec<Creator>>,
    pub category: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
pub struct Creator {
    pub address: String,
    pub share: u16,
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
pub struct FileAttr {
    pub uri: String,
    #[serde(rename = "type")]
    pub file_type: String,
    #[serde(default, skip_serializing_if = "bool_is_false")]
    pub cdn: bool,
}

fn bool_is_false(value: &bool) -> bool {
    !value
}

async fn metadata(Path(index): Path<u64>) -> Json<Metadata> {
    Json(Metadata {
        name: format!("Marinade Leader {}", index),
        symbol: Some("ML".to_string()),
        description: "Marinade Crew Leader".to_string(),
        image: "https://aankor.space/leader.jpeg".to_string(),
        animation_url: None,
        external_url: None,
        attributes: vec![],
        properties: Property {
            files: vec![FileAttr {
                uri: "https://aankor.space/leader.jpeg".to_string(),
                file_type: "image/jpeg".to_string(),
                cdn: false,
            }],
            creators: Some(vec![
                Creator {
                    address: "Ev4PX31e4ALCBUeAxRzdGc8tPygV9sTWyk9bJHSzxu4j".to_string(),
                    share: 0,
                },
                Creator {
                    address: "LncCfW24e9cuekEaUJT3RQcgdASWzE3jhwyJ3QDa6Fn".to_string(),
                    share: 100,
                },
            ]),
            category: None,
        },
    })
}

async fn image() -> impl IntoResponse {
    let data = include_bytes!("../assets/leader.jpeg").to_vec();
    (
        [(header::CONTENT_TYPE, HeaderValue::from_static("image/jpeg"))],
        data,
    )
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET])
        // allow requests from any origin
        .allow_origin(Any);

    /*
    let mut service = ServiceBuilder::new()
        .layer(cors)
        .service_fn(handle);
        */

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/ml/:index", get(metadata))
        .route("/leader.jpeg", get(image))
        .layer(cors);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
