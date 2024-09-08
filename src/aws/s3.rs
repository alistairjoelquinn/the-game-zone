use anyhow::{Context, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{config::Region, Client};
use dotenv::dotenv;

#[derive(Debug)]
pub struct S3Client {
    pub client: Client,
}

impl S3Client {
    pub async fn new() -> Result<Self> {
        let region_provider =
            RegionProviderChain::first_try(Region::new("us-east-1"));
        let region = region_provider.region().await.unwrap();

        println!("Region: {:?}", region);

        let shared_config =
            aws_config::defaults(aws_config::BehaviorVersion::latest())
                .region(region_provider)
                .load()
                .await;

        println!("Shared config: {:?}", shared_config);

        let client = Client::new(&shared_config);

        println!("Client: {:?}", client);

        // Load the .env file
        dotenv().context("Failed to load .env file")?;

        Ok(Self { client }) // Your other methods...
    }
}
