use std::{ collections::HashMap, future::Future, pin::Pin };

use async_trait::async_trait;

use crate::client::client::{ RequestFuture, RunpodClient, RunpodClientAPI };

use super::backend::{ RunpodBackend, RunpodParams };

pub struct RunpodvLLM;
pub struct vLLMSamplingParams {
    prompt: String,
    n: Option<u64>,
    best_of: Option<u64>,
    presence_penalty: Option<f64>,
    frequency_penalty: Option<f64>,
    repetition_penalty: Option<f64>,
    temperature: Option<f64>,
    top_p: Option<f64>,
    top_k: Option<u64>,
    min_p: Option<f64>,
    use_beam_search: Option<bool>,
    length_penalty: Option<f64>,
    early_stopping: Option<String>,
    stop: Option<Vec<String>>,
    stop_token_ids: Option<Vec<u64>>,
    ignore_eos: Option<bool>,
    max_tokens: Option<u64>,
    skip_special_tokens: Option<bool>,
    space_between_special_tokens: Option<bool>
}

impl Default for vLLMSamplingParams {
    fn default() -> Self {
        vLLMSamplingParams {
            prompt: Default::default(),
            n: None,
            best_of: None,
            presence_penalty: None,
            frequency_penalty: None,
            repetition_penalty: None,
            temperature: None,
            top_p: None,
            top_k: None,
            min_p: None,
            use_beam_search: None,
            length_penalty: None,
            early_stopping: None,
            stop: None,
            stop_token_ids: None,
            ignore_eos: None,
            max_tokens: None,
            skip_special_tokens: None,
            space_between_special_tokens: None,
        }
    }
}

pub struct vLLMParams {
    prompt: String,
    messages: Option<HashMap<String,String>>,
    apply_chat_template: Option<bool>,
    sampling_params: Option<vLLMSamplingParams>,
    stream: Option<bool>,
    max_batch_size: Option<u64>,
    min_batch_size: Option<u64>,
    batch_size_growth_factor: Option<u64>
}

impl Default for vLLMParams {
    fn default() -> Self {
        Self { prompt: Default::default(), messages: Default::default(), apply_chat_template: Default::default(), sampling_params: Default::default(), stream: Default::default(), max_batch_size: Default::default(), min_batch_size: Default::default(), batch_size_growth_factor: Default::default() }
    }
}

impl RunpodBackend for RunpodvLLM {}

impl RunpodParams for vLLMParams {}

pub trait VLLMParamBuilderTrait {
    fn with_prompt(self, prompt: String) -> Self;
    fn with_messages(self, messages: HashMap<String,String>) -> Self;
    fn apply_chat_template(self, apply_chat_template: bool) -> Self;
    fn with_sampling_params(self, sampling_params: vLLMSamplingParams) -> Self;
    fn enable_streaming(self, enable_streaming: bool) -> Self;
    fn with_max_batch_size(self, max_batch_size: u64) -> Self;
    fn with_min_batch_size(self, min_batch_size: u64) -> Self;
    fn with_batch_size_growth_factor(self, batch_size_growth_factor: u64) -> Self;
    fn build(self) -> vLLMParams;
}

impl VLLMParamBuilderTrait for vLLMParams {

    fn build(self) -> vLLMParams {
        vLLMParams {
            prompt: self.prompt,
            messages: self.messages,
            apply_chat_template: self.apply_chat_template,
            sampling_params: self.sampling_params,
            stream: self.stream,
            max_batch_size: self.max_batch_size,
            min_batch_size: self.min_batch_size,
            batch_size_growth_factor: self.batch_size_growth_factor,
        }
    }
    
    fn with_prompt(mut self, prompt: String) -> Self {
        self.prompt = prompt;
        self
    }
    
    fn with_messages(mut self, messages: HashMap<String,String>) -> Self {
        self.messages = Some(messages);
        self
    }
    
    fn apply_chat_template(mut self, apply_chat_template: bool) -> Self {
        self.apply_chat_template = Some(apply_chat_template);
        self
    }
    
    fn with_sampling_params(mut self, sampling_params: vLLMSamplingParams) -> Self {
        self.sampling_params = Some(sampling_params);
        self
    }
    
    fn enable_streaming(mut self, enable_streaming: bool) -> Self {
        self.stream = Some(enable_streaming);
        self
    }
    
    fn with_max_batch_size(mut self, max_batch_size: u64) -> Self {
        self.max_batch_size = Some(max_batch_size);
        self
    }
    
    fn with_min_batch_size(mut self, min_batch_size: u64) -> Self {
        self.min_batch_size = Some(min_batch_size);
        self
    }
    
    fn with_batch_size_growth_factor(mut self, batch_size_growth_factor: u64) -> Self {
        self.batch_size_growth_factor = Some(batch_size_growth_factor);
        self
    }
}

#[async_trait]
impl RunpodClientAPI<vLLMParams, ()> for RunpodClient<RunpodvLLM> {
    async fn request(&self, params: vLLMParams) -> RequestFuture<()> {
        println!("api_base: {:#?}", self.api_base);
        todo!()
    }
}
