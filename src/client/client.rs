use std::{ marker::PhantomData, time::Duration };

use async_trait::async_trait;
use reqwest::Url;

use crate::backend::backend::RunpodBackend;

pub const DEFAULT_API_BASE: &str = "https://api.runpod.ai/v2/";

pub struct RunpodClient<T> {
    pub backend: PhantomData<T>,
    pub poll_time: Duration,
    pub api_base: Url,
    pub api_key: String,
    pub machine_id: String,
}

#[async_trait]
pub trait RunpodClientAPI<Req, Res> {
    async fn request(&self, params: Req) -> Res;
}

pub struct RunpodClientBuilder<Backend> where Backend: RunpodBackend,
 {
    backend: PhantomData<Backend>,
    api_base: Option<Url>,
    api_key: Option<String>,
    poll_timer: Option<Duration>,
    machine_id: Option<String>,
}

impl<T> RunpodClientBuilder<T> where T: RunpodBackend {
    pub fn new(_backend: T) -> Self {
        RunpodClientBuilder::<T> {
            backend: PhantomData::<T>,
            api_base: None,
            api_key: None,
            machine_id: None,
            poll_timer: None,
        }
    }
}

pub trait RunpodClientBuilderTrait<T> where T: RunpodBackend {
    fn with_api_base(self, api_base: Url) -> Self;
    fn with_api_key(self, api_key: String) -> Self;
    fn with_machine_id(self, machine_id: String) -> Self;
    fn with_poll_time(self, poll_time_msec: Duration) -> Self;
    fn build(self) -> RunpodClient<T>;
}

impl<T> RunpodClientBuilderTrait<T> for RunpodClientBuilder<T> where T: RunpodBackend {
    fn with_api_base(mut self, api_base: Url) -> Self {
        self.api_base = Some(api_base);
        self
    }

    fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    fn with_machine_id(mut self, machine_id: String) -> Self {
        self.machine_id = Some(machine_id);
        self
    }

    fn with_poll_time(mut self, poll_time_msec: Duration) -> Self {
        self.poll_timer = Some(poll_time_msec);
        self
    }

    fn build(self) -> RunpodClient<T> {
        RunpodClient::<T> {
            api_base: self.api_base.unwrap_or(Url::parse(DEFAULT_API_BASE).unwrap()),
            api_key: self.api_key.unwrap_or_default(),
            machine_id: self.machine_id.unwrap_or_default(),
            backend: PhantomData::<T>,
            poll_time: self.poll_timer.unwrap_or(Duration::from_millis(750)),
        }
    }
}
