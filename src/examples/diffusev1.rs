use std::{ fs::File, io::{ stdin, stdout, Write } };

use rpc::{
    backend::sdv1::{
        StableDiffusionV1,
        StableDiffusionV1OutputFetch,
        StableDiffusionV1ParamBuilderTrait,
        StableDiffusionV1Params,
    },
    client::client::{ RunpodClientAPI, RunpodClientBuilder, RunpodClientBuilderTrait },
};
use reqwest::Url;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), anyhow::Error> {
    let mut key = String::new();
    print!("Please enter your RunPod API Key: ");
    let _ = stdout().flush();
    stdin().read_line(&mut key).expect("Did not enter a correct string");
    key = key.trim().to_string();

    let client = RunpodClientBuilder::new(StableDiffusionV1)
        .with_api_base(Url::parse("https://api.runpod.ai/v2/").unwrap())
        .with_api_key(key)
        .build();

    loop {
        let mut s = String::new();
        print!("Prompt: ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        s = s.clone().trim().to_string();

        println!("Queueing job...");

        let resp = client.request(StableDiffusionV1Params::new().with_prompt(s.clone()).build()).await?;
        let image_bytes = resp.output.clone().unwrap()[0].fetch().await.unwrap();
        s.truncate(7);
        let mut f = File::create(std::format!("./{}.png", s))?;
        f.write_all(image_bytes.as_slice())?;
        println!("\n\nResult: {:#?}", resp);
    }
}
