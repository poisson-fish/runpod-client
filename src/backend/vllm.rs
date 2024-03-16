use std::{ collections::HashMap, time::Duration };

use async_trait::async_trait;
use serde_json::{json, Value};

use crate::client::client::{ RequestFuture, RunpodClient, RunpodClientAPI };

use super::backend::{ RunpodBackend, RunpodParams };

use reqwest::Url;

use anyhow::Error;

use serde::{ Deserialize, Serialize };

pub struct RunpodvLLM;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VLLMSamplingParams {
    
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    best_of: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repetition_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_k: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_beam_search: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    length_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    early_stopping: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_token_ids: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_eos: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_special_tokens: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    space_between_special_tokens: Option<bool>,
}

impl VLLMSamplingParams {
    pub fn new() -> Self {
        VLLMSamplingParams {
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
impl Default for VLLMSamplingParams {
    fn default() -> Self {
        VLLMSamplingParams {
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

pub trait VLLMSamplingParamBuilderTrait {
    fn with_n(self, n: u64) -> Self;
    fn with_best_of(self, best_of: u64) -> Self;
    fn with_presence_penalty(self, presence_penalty: f64) -> Self;
    fn with_frequency_penalty(self, frequency_penalty: f64) -> Self;
    fn with_repetition_penalty(self, repetition_penalty: f64) -> Self;
    fn with_temperature(self, temperature: f64) -> Self;
    fn with_top_p(self, top_p: f64) -> Self;
    fn with_top_k(self, top_k: u64) -> Self;
    fn with_min_p(self, min_p: f64) -> Self;
    fn use_beam_search(self, beam_search: bool) -> Self;
    fn with_length_penalty(self, length_penalty: f64) -> Self;
    fn with_early_stopping(self, early_stopping: String) -> Self;
    fn with_stop(self, stop_tokens: Vec<String>) -> Self;
    fn with_stop_token_ids(self, stop_token_ids: Vec<u64>) -> Self;
    fn with_ignore_eos(self, ignore_eos: bool) -> Self;
    fn with_max_tokens(self, max_tokens: u64) -> Self;
    fn with_skip_special_tokens(self, skip_special: bool) -> Self;
    fn with_space_between_special_tokens(self, space_special: bool) -> Self;
}

impl VLLMSamplingParamBuilderTrait for VLLMSamplingParams {
    fn with_n(mut self, n: u64) -> Self {
        self.n = Some(n);
        self
    }

    fn with_best_of(mut self, best_of: u64) -> Self {
        self.best_of = Some(best_of);
        self
    }

    fn with_presence_penalty(mut self, presence_penalty: f64) -> Self {
        self.presence_penalty = Some(presence_penalty);
        self
    }

    fn with_frequency_penalty(mut self, frequency_penalty: f64) -> Self {
        self.frequency_penalty = Some(frequency_penalty);
        self
    }

    fn with_repetition_penalty(mut self, repetition_penalty: f64) -> Self {
        self.repetition_penalty = Some(repetition_penalty);
        self
    }

    fn with_temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    fn with_top_p(mut self, top_p: f64) -> Self {
        self.top_p = Some(top_p);
        self
    }

    fn with_top_k(mut self, top_k: u64) -> Self {
        self.top_k = Some(top_k);
        self
    }

    fn with_min_p(mut self, min_p: f64) -> Self {
        self.min_p = Some(min_p);
        self
    }

    fn use_beam_search(mut self, beam_search: bool) -> Self {
        self.use_beam_search = Some(beam_search);
        self
    }

    fn with_length_penalty(mut self, length_penalty: f64) -> Self {
        self.length_penalty = Some(length_penalty);
        self
    }

    fn with_early_stopping(mut self, early_stopping: String) -> Self {
        self.early_stopping = Some(early_stopping);
        self
    }

    fn with_stop(mut self, stop_tokens: Vec<String>) -> Self {
        self.stop = Some(stop_tokens);
        self
    }

    fn with_stop_token_ids(mut self, stop_token_ids: Vec<u64>) -> Self {
        self.stop_token_ids = Some(stop_token_ids);
        self
    }

    fn with_ignore_eos(mut self, ignore_eos: bool) -> Self {
        self.ignore_eos = Some(ignore_eos);
        self
    }

    fn with_max_tokens(mut self, max_tokens: u64) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    fn with_skip_special_tokens(mut self, skip_special: bool) -> Self {
        self.skip_special_tokens = Some(skip_special);
        self
    }

    fn with_space_between_special_tokens(mut self, space_special: bool) -> Self {
        self.space_between_special_tokens = Some(space_special);
        self
    }

    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VLLMParams {
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    messages: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apply_chat_template: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sampling_params: Option<VLLMSamplingParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_batch_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_batch_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_size_growth_factor: Option<u64>,
}

impl Default for VLLMParams {
    fn default() -> Self {
        Self {
            prompt: Default::default(),
            messages: Default::default(),
            apply_chat_template: Default::default(),
            sampling_params: Default::default(),
            stream: Default::default(),
            max_batch_size: Default::default(),
            min_batch_size: Default::default(),
            batch_size_growth_factor: Default::default(),
        }
    }
}

impl RunpodBackend for RunpodvLLM {}

impl RunpodParams for VLLMParams {}

pub trait VLLMParamBuilderTrait {
    fn with_prompt(self, prompt: String) -> Self;
    fn with_messages(self, messages: HashMap<String, String>) -> Self;
    fn apply_chat_template(self, apply_chat_template: bool) -> Self;
    fn with_sampling_params(self, sampling_params: VLLMSamplingParams) -> Self;
    fn enable_streaming(self, enable_streaming: bool) -> Self;
    fn with_max_batch_size(self, max_batch_size: u64) -> Self;
    fn with_min_batch_size(self, min_batch_size: u64) -> Self;
    fn with_batch_size_growth_factor(self, batch_size_growth_factor: u64) -> Self;
    fn build(self) -> VLLMParams;
}

impl VLLMParamBuilderTrait for VLLMParams {
    fn build(self) -> VLLMParams {
        VLLMParams {
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

    fn with_messages(mut self, messages: HashMap<String, String>) -> Self {
        self.messages = Some(messages);
        self
    }

    fn apply_chat_template(mut self, apply_chat_template: bool) -> Self {
        self.apply_chat_template = Some(apply_chat_template);
        self
    }

    fn with_sampling_params(mut self, sampling_params: VLLMSamplingParams) -> Self {
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

async fn queue_job(
    api_base: Url,
    machine_id: String,
    api_key: String,
    params: VLLMParams
) -> Result<Value, Error> {
    let machine_run_async: Url = api_base
        .join(std::format!("{}/", machine_id).as_str())
        .unwrap()
        .join("run")
        .unwrap();
    println!("req url: {:#?}", machine_run_async);
    let client = reqwest::Client::new();


    let request = json!({
        "input": params
    });
    println!("params: {:#?}", request);

    client
        .post(machine_run_async)
        .bearer_auth(api_key)
        .json(&request)
        .send().await?
        .json::<Value>().await
        .map_err(|x| x.into())
}

async fn wait_for_completion(job_id: &str, api_base: Url, machine_id: String, api_key: String) -> Result<Value, Error> {

    let machine_status_async: Url = api_base
        .join(std::format!("{}/", machine_id).as_str())
        .unwrap()
        .join("status/")
        .unwrap()
        .join(std::format!("{}/", job_id).as_str())
        .unwrap();

    let client = reqwest::Client::new();

    loop {
        let response = client
            .get(machine_status_async.clone())
            .bearer_auth(api_key.clone())
            .send().await?
            .json::<Value>().await?;

        match response["status"].as_str() {
            Some("COMPLETED") => {
                // All done
                break Ok(response);
            },
            Some("FAILED") => {
                break Err(Error::msg("RunPod job status FAILED."));
            },
            _ => {
                tokio::time::sleep(Duration::from_millis(750)).await;
            }
        }
    }

}
#[async_trait]
impl RunpodClientAPI<VLLMParams, Result<Value, Error>> for RunpodClient<RunpodvLLM> {
    fn request(&self, params: VLLMParams) -> RequestFuture<Result<Value, Error>> {
        Box::pin(async move {

            let response = queue_job(self.api_base.clone(), self.machine_id.clone(), self.api_key.clone(), params).await?;

            let comp: Result<Value, Error> = match response.get("status").unwrap().as_str() {
                Some("IN_QUEUE") => async {
                    //Queued successfully
                    let id = response["id"].as_str().unwrap();
                    let status = response["status"].as_str().unwrap();
                    println!("{}: {}", id, status);
                    let completion = wait_for_completion(id, self.api_base.clone(), self.machine_id.clone(), self.api_key.clone()).await.unwrap(); // This will only crash the pinned thread
                    Ok(completion)
                }.await,
                _ => {
                    //Something happened
                    println!("Couldn't queue prompt!");
                    Err(Error::msg("Couldn't queue prompt!"))
                }
            };
            comp
        })

        /*println!("txt2txt_res: {:#?}", txt2txt_res);
        let response = txt2txt_res["output"]["choices"][0]["text"].as_str();
        println!("{:#?}", response);
        response.unwrap().to_owned();*/
    }
}
