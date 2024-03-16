#![feature(associated_type_defaults)]
#![feature(associated_type_bounds)]


pub mod client;
pub mod backend;

#[cfg(test)]
mod tests {
    use reqwest::Url;

    use crate::{backend::vllm::RunpodvLLM, client::client::{ RunpodClientBuilder, RunpodClientBuilderTrait}};
    #[test]
    fn test_builder_pattern() {
        let backend = RunpodvLLM; // Placeholder for actual backend type

        let client = RunpodClientBuilder::new(backend)
            .with_api_base(Url::parse("https://example.com/").unwrap())
            .with_api_key("test_key".to_string())
            .with_machine_id("test_machine".to_string())
            .build();

        assert_eq!(client.api_base.as_str(), "https://example.com/");
        assert_eq!(client.api_key, "test_key");
        assert_eq!(client.machine_id, "test_machine");
    }
}
