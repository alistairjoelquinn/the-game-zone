use std::sync::Arc;

use crate::state::State;
use anyhow::{Context, Result};
use aws_config::default_provider::credentials::DefaultCredentialsChain;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::{config::Region, Client};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Extension;
use tracing::{error, info, warn};

#[derive(Debug)]
pub struct S3Client {
    pub client: Client,
}

impl S3Client {
    pub async fn new() -> Result<Self> {
        dotenv::dotenv().context("Failed to load .env file")?;

        let region_provider =
            RegionProviderChain::first_try(Region::new("us-east-1"));

        // Use the default credentials chain
        let credentials_provider =
            DefaultCredentialsChain::builder().build().await;

        let shared_config = aws_config::from_env()
            .region(region_provider)
            .credentials_provider(credentials_provider)
            .load()
            .await;

        let client = Client::new(&shared_config);

        Ok(Self { client })
    }
}

pub async fn init_s3() -> Result<Arc<S3Client>> {
    match S3Client::new().await {
        Ok(s3) => {
            println!("S3 client successfully initialized");
            Ok(Arc::new(s3))
        }
        Err(e) => {
            eprintln!("Failed to create S3 client: {}", e);
            Err(e)
        }
    }
}

pub async fn get_s3_object(
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    match retrieve_s3_object(state).await {
        Ok(Some(response)) => response,
        Ok(None) => {
            error!("S3 object not found");
            (StatusCode::NOT_FOUND, "Object not found").into_response()
        }
        Err(e) => {
            error!("Failed to retrieve S3 object: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

async fn retrieve_s3_object(state: Arc<State>) -> Result<Option<Response>> {
    info!("Starting S3 object retrieval");

    let bucket = "askama-game-zone";
    let key = "IMG_6740.jpeg";

    info!(
        "Attempting to fetch object from S3. Bucket: {}, Key: {}",
        bucket, key
    );

    let obj = match state
        .s3
        .client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
    {
        Ok(output) => output,
        Err(err) => match err {
            SdkError::ServiceError(service_err) => {
                if service_err.err().is_no_such_key() {
                    info!("S3 object not found");
                    return Ok(None);
                }
                error!("S3 service error: {:?}", service_err);
                return Err(anyhow::anyhow!(
                    "S3 service error: {:?}",
                    service_err
                ));
            }
            _ => {
                error!("S3 error: {:?}", err);
                return Err(anyhow::anyhow!("S3 error: {:?}", err));
            }
        },
    };

    info!("Successfully retrieved object from S3");

    info!("Reading object data");
    let data = obj
        .body
        .collect()
        .await
        .map_err(|e| {
            error!("Failed to read S3 object body: {:?}", e);
            anyhow::anyhow!("Failed to read S3 object body: {:?}", e)
        })?
        .into_bytes();
    info!("Successfully read {} bytes of object data", data.len());

    let content_type = obj
        .content_type
        .clone()
        .unwrap_or_else(|| {
            warn!("Content-Type not provided by S3, defaulting to application/octet-stream");
            "application/octet-stream".to_string()
        });
    info!("Content-Type: {}", content_type);

    info!("Constructing response");
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .body(axum::body::Body::from(data))
        .map_err(|e| {
            error!("Failed to construct response: {:?}", e);
            anyhow::anyhow!("Failed to construct response: {:?}", e)
        })?;

    info!("Successfully constructed response");
    Ok(Some(response))
}
