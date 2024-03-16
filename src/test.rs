use clientlib::{backend::vllm::{VLLMParams, VLLMSamplingParams, RunpodvLLM, VLLMParamBuilderTrait, VLLMSamplingParamBuilderTrait}, client::client::{RunpodClientAPI, RunpodClientBuilder, RunpodClientBuilderTrait}};
use reqwest::Url;

#[tokio::main(flavor = "multi_thread", worker_threads = 20)]
async fn main() -> Result<(), anyhow::Error> {
    let backend = RunpodvLLM; // Placeholder for actual backend type

    let client = RunpodClientBuilder::new(backend)
        .with_api_base(Url::parse("https://api.runpod.ai/v2/").unwrap())
        .with_api_key("F8BL5VS8A6IQTRTZ04R8K1VRW9ZZWNZVJHDCT44S".to_string())
        .with_machine_id("npylzv8htsc4v5".to_string())
        .build();
    let resp = client.request(
        VLLMParams::default()
            .with_prompt("<|im_start|>system\nYou are an intelligent AI assistant. Answer the user.<|im_end|>\n<|im_start|>user\nWhat is 5 * 5 + 5?<|im_end|>\n<|im_start|>assistant\n".to_string())
            .with_sampling_params(VLLMSamplingParams::new().with_max_tokens(512))
            .build()
    ).await?;
    println!("{:#?}", resp);
    Ok(())
}
