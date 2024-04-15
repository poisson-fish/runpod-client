#![allow(non_snake_case)]

use std::time::Duration;

use async_trait::async_trait;
use log::info;
use serde_json::{ json, Value };

use crate::client::client::{ RunpodClient, RunpodClientAPI, DEFAULT_API_BASE };

use super::backend::{ RunpodBackend, RunpodParams };

use reqwest::Url;

use anyhow::Error;

use serde::{ Deserialize, Serialize };

pub struct StableDiffusionV1;

#[async_trait]
pub trait StableDiffusionV1OutputFetch {
    async fn fetch(&self) -> Result<Vec<u8>, Error>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StableDiffusionV1Output {
    pub image: String,
    pub seed: i64,
}

#[async_trait]
impl StableDiffusionV1OutputFetch for StableDiffusionV1Output {
    async fn fetch(&self) -> Result<Vec<u8>, Error> {
        let url = Url::parse(self.image.as_str()).unwrap_or(Url::parse(DEFAULT_API_BASE).unwrap());
        reqwest::Client
            ::new()
            .get(url)
            .send().await?
            .bytes().await
            .map(|x| x.to_vec())
            .map_err(|x| x.into())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StableDiffusionV1Result {
    pub delayTime: Option<u64>,
    pub executionTime: Option<u64>,
    pub id: Option<String>,
    pub output: Option<Vec<StableDiffusionV1Output>>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StableDiffusionV1Params {
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guidance_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_inference_steps: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_outputs: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt_strength: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduler: Option<String>,
}

impl StableDiffusionV1Params {
    pub fn new() -> Self {
        Self {
            prompt: Default::default(),
            width: None,
            height: None,
            guidance_scale: None,
            num_inference_steps: None,
            num_outputs: None,
            prompt_strength: None,
            scheduler: None,
        }
    }
}

impl Default for StableDiffusionV1Params {
    fn default() -> Self {
        Self {
            prompt: Default::default(),
            width: None,
            height: None,
            guidance_scale: None,
            num_inference_steps: None,
            num_outputs: None,
            prompt_strength: None,
            scheduler: None,
        }
    }
}

impl RunpodBackend for StableDiffusionV1 {}

impl RunpodParams for StableDiffusionV1Params {}

pub trait StableDiffusionV1ParamBuilderTrait {
    fn with_prompt(self, prompt: String) -> Self;
    fn with_resolution(self, width: u64, height: u64) -> Self;
    fn with_guidance_scale(self, cfg_scale: f64) -> Self;
    fn with_steps(self, steps: u64) -> Self;
    fn with_multiple_outputs(self, output_num: u64) -> Self;
    fn with_prompt_strength(self, strength: u64) -> Self;
    fn with_scheduler(self, scheduler: String) -> Self;
    fn build(self) -> StableDiffusionV1Params;
}

impl StableDiffusionV1ParamBuilderTrait for StableDiffusionV1Params {
    fn build(self) -> StableDiffusionV1Params {
        StableDiffusionV1Params {
            prompt: self.prompt,
            width: self.width,
            height: self.height,
            guidance_scale: self.guidance_scale,
            num_inference_steps: self.num_inference_steps,
            num_outputs: self.num_outputs,
            prompt_strength: self.prompt_strength,
            scheduler: self.scheduler,
        }
    }

    fn with_prompt(mut self, prompt: String) -> Self {
        self.prompt = prompt;
        self
    }

    fn with_resolution(mut self, width: u64, height: u64) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    fn with_guidance_scale(mut self, cfg_scale: f64) -> Self {
        self.guidance_scale = Some(cfg_scale);
        self
    }

    fn with_steps(mut self, steps: u64) -> Self {
        self.num_inference_steps = Some(steps);
        self
    }

    fn with_multiple_outputs(mut self, output_num: u64) -> Self {
        self.num_outputs = Some(output_num);
        self
    }

    fn with_prompt_strength(mut self, strength: u64) -> Self {
        self.prompt_strength = Some(strength);
        self
    }

    fn with_scheduler(mut self, scheduler: String) -> Self {
        self.scheduler = Some(scheduler);
        self
    }
}

async fn queue_job(
    api_base: Url,
    api_key: String,
    params: StableDiffusionV1Params
) -> Result<Value, Error> {
    let machine_run_async: Url = api_base
        .join("stable-diffusion-v1/")
        .unwrap()
        .join("run")
        .unwrap();
    let client = reqwest::Client::new();

    let request = json!({
        "input": params
    });

    info!("SDv1 Request: {:#?}", request);

    let result = client
        .post(machine_run_async)
        .bearer_auth(api_key)
        .json(&request)
        .send().await?
        .json::<Value>().await
        .map_err(|x| x.into());
    
    info!("SDv1 Result: {:#?}", result);

    result
}

async fn wait_for_completion(
    job_id: &str,
    api_base: Url,
    api_key: String,
    poll_time: Duration
) -> Result<StableDiffusionV1Result, Error> {
    let machine_status_async: Url = api_base
        .join("stable-diffusion-v1/")?
        .join("status/")?
        .join(std::format!("{}/", job_id).as_str())?;

    let client = reqwest::Client::new();

    loop {
        let response = client
            .get(machine_status_async.clone())
            .bearer_auth(api_key.clone())
            .send().await?
            .json::<StableDiffusionV1Result>().await?;

        match response.status.as_ref().expect("Didn't get status from job queue.").as_str() {
            "COMPLETED" => {
                // All done
                break Ok(response);
            }
            "FAILED" => {
                break Err(Error::msg("RunPod job status FAILED."));
            }
            _ => {
                tokio::time::sleep(poll_time).await;
            }
        }
    }
}

#[async_trait]
impl RunpodClientAPI<StableDiffusionV1Params, Result<StableDiffusionV1Result, Error>>
for RunpodClient<StableDiffusionV1> {
    async fn request(
        &self,
        params: StableDiffusionV1Params
    ) -> Result<StableDiffusionV1Result, Error> {
        let response = queue_job(self.api_base.clone(), self.api_key.clone(), params).await?;
        let comp: Result<StableDiffusionV1Result, Error> = match
            response.get("status").unwrap().as_str()
        {
            Some("IN_QUEUE") => {
                    //Queued successfully
                    let id = response["id"].as_str().unwrap();
                    wait_for_completion(
                        id,
                        self.api_base.clone(),
                        self.api_key.clone(),
                        self.poll_time
                    ).await
            }
            _ => {
                //Something happened
                Err(Error::msg("Couldn't queue prompt!"))
            }
        };
        comp
    }
}
