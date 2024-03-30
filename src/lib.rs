pub mod client;
pub mod backend;

#[cfg(test)]
mod tests {
    use std::env;

    use crate::{backend::{sdv1::{StableDiffusionV1, StableDiffusionV1ParamBuilderTrait, StableDiffusionV1Params}, vllm::{VLLMParamBuilderTrait, VLLMParams, VLLM}}, client::client::{ RunpodClientAPI, RunpodClientBuilder, RunpodClientBuilderTrait}};
    #[tokio::test]
    async fn test_vllm_provider() {
        let client = RunpodClientBuilder::new(VLLM)
            .with_api_key(env::var("RUNPOD_API_KEY").unwrap())
            .with_machine_id("llama2-7b-chat".to_owned())
            .build();

        let response = client.request(VLLMParams::new()
            .with_prompt("only output this word, and this word only: DONE".to_owned())).await;
        assert_eq!(response.is_err(), false);
    }
    #[tokio::test]
    async fn test_stable_diffusion_v1_provider() {
        let client = RunpodClientBuilder::new(StableDiffusionV1)
            .with_api_key(env::var("RUNPOD_API_KEY").unwrap())
            .build();

        let response = client.request(StableDiffusionV1Params::new()
            .with_prompt("a curious cat".to_owned())).await;
        assert_eq!(response.is_err(), false);
    }

}
