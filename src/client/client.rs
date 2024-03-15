use std::{ future::Future, marker::PhantomData, pin::Pin };

use async_trait::async_trait;
use reqwest::Url;

use crate::backend::backend::{ RunpodBackend, RunpodParams };

const DEFAULT_API_BASE: &str = "https://api.runpod.ai/v2";

pub type RequestFuture<T> = Pin<Box<dyn Future<Output = T>>>;

pub struct RunpodClient<T> where T: RunpodBackend {
    backend: T,
    pub api_base: Url,
    pub api_key: String,
    pub machine_id: String,
}

#[async_trait]
pub trait RunpodClientAPI<Req, Res> {
    async fn request(&self, params: Req) -> RequestFuture<Res>;
}

pub struct RunpodClientBuilder<Backend> where Backend: RunpodBackend,
 {
    backend: Backend,
    api_base: Option<Url>,
    api_key: Option<String>,
    machine_id: Option<String>,
}

impl<T> RunpodClientBuilder<T> where T: RunpodBackend {
    pub fn new(backend: T) -> Self {
        RunpodClientBuilder::<T> {
            backend,
            api_base: None,
            api_key: None,
            machine_id: None,
        }
    }
}

pub trait RunpodClientBuilderTrait<T> where T: RunpodBackend {
    fn with_api_base(self, api_base: Url) -> Self;
    fn with_api_key(self, api_key: String) -> Self;
    fn with_machine_id(self, machine_id: String) -> Self;
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

    fn build(self) -> RunpodClient<T> {
        RunpodClient::<T> {
            backend: self.backend,
            api_base: self.api_base.unwrap_or(Url::parse(DEFAULT_API_BASE).unwrap()),
            api_key: self.api_key.unwrap_or_default(),
            machine_id: self.machine_id.unwrap_or_default(),
        }
    }
}
