use std::sync::Arc;

use anyhow::{Context, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{config::Region, Client};
use axum::{response::Html, Extension};
use dotenv::dotenv;

#[derive(Debug)]
pub struct S3Client {
    pub client: Client,
}

impl S3Client {
    pub async fn new() -> Result<Self> {
        dotenv().context("Failed to load .env file")?;

        let region_provider =
            RegionProviderChain::first_try(Region::new("us-east-1"));
        // let _region = region_provider.region().await.unwrap();

        let shared_config =
            aws_config::defaults(aws_config::BehaviorVersion::latest())
                .region(region_provider)
                .load()
                .await;

        println!("Shared config: {:#?}", shared_config);

        let client = Client::new(&shared_config);

        println!("Client: {:?}", client);

        Ok(Self { client }) // Your other methods...
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
) -> Result<Html<&'static str>, anyhow::Error> {
    println!("running Koen code");
    let obj = state
        .s3
        .client
        .get_object()
        .bucket("your-bucket-name")
        .key("path/to/image.jpg")
        .send()
        .await
        .context("Failed to get object from S3")?;
    let _data = obj
        .body
        .collect()
        .await
        .context("Failed to collect S3 object body")?;
    Ok(Html("<p>hi</p>"))
}
