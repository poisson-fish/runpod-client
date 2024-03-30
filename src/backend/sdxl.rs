#![allow(non_snake_case)]

use std::time::Duration;

use async_trait::async_trait;
use serde_json::{ json, Value };

use crate::client::client::{ RunpodClient, RunpodClientAPI, DEFAULT_API_BASE };

use super::backend::{ RunpodBackend, RunpodParams };

use reqwest::Url;

use anyhow::Error;

use serde::{ Deserialize, Serialize };

pub struct StableDiffusionXL;

#[async_trait]
pub trait StableDiffusionXLOutputFetch {
    async fn fetch(&self) -> Result<Vec<u8>, Error>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StableDiffusionXLOutput {
    pub image_url: String,
    pub images: Vec<String>,
    pub seed: i64,
}

#[async_trait]
impl StableDiffusionXLOutputFetch for StableDiffusionXLOutput {
    async fn fetch(&self) -> Result<Vec<u8>, Error> {
        let url = Url::parse(self.image_url.as_str()).unwrap_or(Url::parse(DEFAULT_API_BASE).unwrap());
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
pub struct StableDiffusionXLResult {
    pub delayTime: Option<u64>,
    pub executionTime: Option<u64>,
    pub id: Option<String>,
    pub output: Option<StableDiffusionXLOutput>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StableDiffusionXLParams {
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guidance_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strength: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_inference_steps: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refiner_inference_steps: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_images: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduler: Option<String>,
}

impl StableDiffusionXLParams {
    pub fn new() -> Self {
        Self {
            prompt: Default::default(),
            width: None,
            height: None,
            guidance_scale: None,
            strength: None,
            num_inference_steps: None,
            refiner_inference_steps: None,
            num_images: None,
            scheduler: None,
        }
    }
}

impl Default for StableDiffusionXLParams {
    fn default() -> Self {
        Self {
            prompt: Default::default(),
            width: None,
            height: None,
            guidance_scale: None,
            strength: None,
            num_inference_steps: None,
            refiner_inference_steps: None,
            scheduler: None,
            num_images: None,
        }
    }
}

impl RunpodBackend for StableDiffusionXL {}

impl RunpodParams for StableDiffusionXLParams {}

pub trait StableDiffusionXLParamBuilderTrait {
    fn with_prompt(self, prompt: String) -> Self;
    fn with_resolution(self, width: u64, height: u64) -> Self;
    fn with_guidance_scale(self, cfg_scale: f64) -> Self;
    fn with_refiner_steps(self, refiner_steps: u64) -> Self;
    fn with_steps(self, steps: u64) -> Self;
    fn with_num_images(self, output_num: u64) -> Self;
    fn with_strength(self, strength: f64) -> Self;
    fn with_scheduler(self, scheduler: String) -> Self;
    fn build(self) -> StableDiffusionXLParams;
}

impl StableDiffusionXLParamBuilderTrait for StableDiffusionXLParams {
    fn build(self) -> StableDiffusionXLParams {
        StableDiffusionXLParams {
            prompt: self.prompt,
            width: self.width,
            height: self.height,
            guidance_scale: self.guidance_scale,
            num_inference_steps: self.num_inference_steps,
            scheduler: self.scheduler,
            strength: self.strength,
            refiner_inference_steps: self.refiner_inference_steps,
            num_images: self.num_images,
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

    fn with_refiner_steps(mut self, refiner_steps: u64) -> Self {
        self.refiner_inference_steps = Some(refiner_steps);
        self
    }

    fn with_strength(mut self, strength: f64) -> Self {
        self.strength = Some(strength);
        self
    }

    fn with_scheduler(mut self, scheduler: String) -> Self {
        self.scheduler = Some(scheduler);
        self
    }
    
    fn with_num_images(mut self, output_num: u64) -> Self {
        self.num_images = Some(output_num);
        self
    }
    
}

async fn queue_job(
    api_base: Url,
    api_key: String,
    params: StableDiffusionXLParams
) -> Result<Value, Error> {
    let machine_run_async: Url = api_base
        .join("sdxl/")
        .unwrap()
        .join("run")
        .unwrap();
    let client = reqwest::Client::new();

    let request = json!({
        "input": params
    });

    client
        .post(machine_run_async)
        .bearer_auth(api_key)
        .json(&request)
        .send().await?
        .json::<Value>().await
        .map_err(|x| x.into())
}

async fn wait_for_completion(
    job_id: &str,
    api_base: Url,
    api_key: String,
    poll_time: Duration
) -> Result<StableDiffusionXLResult, Error> {
    let machine_status_async: Url = api_base
        .join("sdxl/")?
        .join("status/")?
        .join(std::format!("{}/", job_id).as_str())?;

    let client = reqwest::Client::new();

    loop {
        let response = client
            .get(machine_status_async.clone())
            .bearer_auth(api_key.clone())
            .send().await?
            .json::<StableDiffusionXLResult>().await?;

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
impl RunpodClientAPI<StableDiffusionXLParams, Result<StableDiffusionXLResult, Error>>
for RunpodClient<StableDiffusionXL> {
    async fn request(
        &self,
        params: StableDiffusionXLParams
    ) -> Result<StableDiffusionXLResult, Error> {
        let response = queue_job(self.api_base.clone(), self.api_key.clone(), params).await?;
        let comp: Result<StableDiffusionXLResult, Error> = match
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
