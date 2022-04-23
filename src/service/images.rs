use std::env;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{ByteStream, Client, Region, Endpoint};
use rocket::http::hyper::Uri;

/// Service function for uploading an image.
pub async fn upload_image(file_name: &str, bytes: Vec<u8>) {
    let bucket_name = env::var("IMAGE_BUCKET_NAME").unwrap();

    let region_provider = RegionProviderChain::first_try(Region::new("us-east-1"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    let body = ByteStream::from(bytes);

    client
        .put_object()
        .bucket(bucket_name)
        .key(file_name)
        .body(body)
        .send()
        .await
        .unwrap();
}

pub async fn load_image(file_name: &str) -> Vec<u8> {
    let bucket_name = env::var("IMAGE_BUCKET_NAME").unwrap();
    let bucket_endpoint = env::var("IMAGE_ENDPOINT_URI").unwrap();

    let region_provider = RegionProviderChain::first_try(Region::new("us-east-1"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let shared_config = aws_sdk_s3::config::Builder::from(&shared_config).endpoint_resolver(Endpoint::immutable(bucket_endpoint.parse().unwrap())).build();
    let client = Client::from_conf(shared_config);

    let resp = client
        .get_object()
        .bucket(bucket_name)
        .key(file_name)
        .send()
        .await
        .unwrap();

    let data = resp.body.collect().await.unwrap().into_bytes();
    data.to_vec()
}
