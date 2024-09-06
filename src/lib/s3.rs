use aws_config::BehaviorVersion;
use aws_sdk_s3::{Client, Error};
use bytes::Bytes;

pub struct S3Client {
    client: Client,
}

impl S3Client {
    pub async fn new() -> Result<Self, Error> {
        let config = aws_config::from_env()
            .behavior_version(BehaviorVersion::v2023_11_09())
            .load()
            .await;
        let client = Client::new(&config);

        Ok(S3Client { client })
    }

    pub async fn upload_file(
        &self,
        bucket: &str,
        key: &str,
        body: Vec<u8>,
    ) -> Result<(), Error> {
        self.client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(Bytes::from(body))
            .send()
            .await?;

        Ok(())
    }

    pub async fn download_file(
        &self,
        bucket: &str,
        key: &str,
    ) -> Result<Vec<u8>, Error> {
        let resp = self
            .client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;

        Ok(resp.body.collect().await?.into_bytes().to_vec())
    }
}
