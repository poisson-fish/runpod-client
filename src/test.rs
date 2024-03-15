use clientlib::{backend::vllm::{vLLMParams, RunpodvLLM, VLLMParamBuilderTrait}, client::{client::{RunpodClientAPI, RunpodClientBuilder, RunpodClientBuilderTrait}, *}};
use reqwest::Url;

#[tokio::main(flavor = "multi_thread", worker_threads = 20)]
async fn main() -> Result<(), anyhow::Error> {
    let backend = RunpodvLLM; // Placeholder for actual backend type

    let client = RunpodClientBuilder::new(backend)
        .with_api_base(Url::parse("https://api.runpod.ai/v2/").unwrap())
        .with_api_key("E16J10BC2R96NL7W3QYBG0ZMSY0VH1AH6CAKCQER".to_string())
        .with_machine_id("npylzv8htsc4v5".to_string())
        .build();
    let _ = client.request(
        vLLMParams::default()
            .with_prompt("<|im_start|>system\nYou are an intelligent AI assistant. Answer the user.<|im_end|>\n<|im_start|>user\nWhat is 5 * 5 + 5?<|im_end|>\n<|im_start|>assistant\n".to_string())
            .build()
    ).await.await;
    Ok(())
}
