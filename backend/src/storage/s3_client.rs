use crate::config::Config;
use aws_sdk_s3::{
    config::{Credentials, Region},
    Client as S3Client,
    Endpoint,
};
use std::str::FromStr;

pub struct S3ClientWrapper {
    client: S3Client,
    bucket: String,
}

impl S3ClientWrapper {
    pub async fn new(config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        let credentials = Credentials::new(
            &config.s3_access_key_id,
            &config.s3_secret_access_key,
            None,
            None,
            "hyperswap",
        );

        let mut s3_config = aws_sdk_s3::Config::builder()
            .credentials_provider(credentials)
            .region(Region::new(config.s3_region.clone()));

        // If custom endpoint (e.g., MinIO), use it
        if let Some(endpoint) = &config.s3_endpoint {
            s3_config = s3_config.endpoint_resolver(Endpoint::immutable(
                http::Uri::from_str(endpoint)?,
            ));
        }

        let client = S3Client::from_conf(s3_config.build());

        Ok(S3ClientWrapper {
            client,
            bucket: config.s3_bucket.clone(),
        })
    }

    pub async fn upload(
        &self,
        key: &str,
        data: Vec<u8>,
        content_type: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use aws_sdk_s3::primitives::ByteStream;
        
        let mut request = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(ByteStream::from(data));

        if let Some(ct) = content_type {
            request = request.content_type(ct);
        }

        request.send().await?;

        Ok(())
    }

    pub async fn download(&self, key: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;

        let bytes = response.body.collect().await?;
        let data = bytes.into_bytes().to_vec();

        Ok(data)
    }

    pub async fn delete(&self, key: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;

        Ok(())
    }

    pub fn get_bucket(&self) -> &str {
        &self.bucket
    }
}

// Alias for consistency
pub type S3Client = S3ClientWrapper;

