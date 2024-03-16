use std::io::{ stdin, stdout, Write };

use rpc::{
    backend::vllm::{
        VLLMParams,
        VLLMSamplingParams,
        RunpodvLLM,
        VLLMParamBuilderTrait,
        VLLMSamplingParamBuilderTrait,
    },
    client::client::{ RunpodClientAPI, RunpodClientBuilder, RunpodClientBuilderTrait },
};
use reqwest::Url;

#[tokio::main(flavor = "multi_thread", worker_threads = 20)]
async fn main() -> Result<(), anyhow::Error> {
    let backend = RunpodvLLM; // Placeholder for actual backend type

    let mut key = String::new();
    print!("Please enter your RunPod API Key: ");
    let _ = stdout().flush();
    stdin().read_line(&mut key).expect("Did not enter a correct string");
    key = key.trim().to_string();

    let mut machineid = String::new();
    print!("Please enter your RunPod Serverless Worker ID: ");
    let _ = stdout().flush();
    stdin().read_line(&mut machineid).expect("Did not enter a correct string");
    machineid = machineid.trim().to_string();

    let client = RunpodClientBuilder::new(backend)
        .with_api_base(Url::parse("https://api.runpod.ai/v2/").unwrap())
        .with_api_key(key)
        .with_machine_id(machineid)
        .build();

    loop {
        let mut s = String::new();
        print!("Please enter a prompt: ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        s = s.trim().to_string();

        let resp = client.request(
            VLLMParams::new()
                .with_prompt(
                    std::format!(
                        "<|im_start|>system\nYou are an intelligent AI assistant. Answer the user.<|im_end|>\n<|im_start|>user\n{}<|im_end|>\n<|im_start|>assistant\n",
                        s
                    )
                )
                .with_sampling_params(VLLMSamplingParams::new().with_max_tokens(512))
                .build()
        ).await?;
        println!("{:#?}", resp);
    }
}
