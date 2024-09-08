use crate::aws::s3::S3Client;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct State {
    pub db: sqlx::PgPool,
    pub s3: Arc<S3Client>,
}
