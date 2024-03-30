use std::time::Duration;

use async_trait::async_trait;
use reqwest::Url;

pub trait RunpodBackend: {}

pub trait RunpodParams: {}

#[async_trait]
pub trait RunpodRequest<R, P, E>: {
    async fn queue_job(
        api_base: Url,
        api_key: String,
        params: P,
    ) -> Result<R, E>;
    async fn wait_for_completion(
        job_id: &str,
        api_base: Url,
        api_key: String,
        poll_time: Duration,
    ) -> Result<R, E>;
}
