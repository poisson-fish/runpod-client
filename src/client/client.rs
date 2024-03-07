use reqwest::Url;

const DEFAULT_API_BASE: &str = "https://api.runpod.ai/v2";

struct RunpodClient<T> {
    backend: T,
    api_base: Url,
    api_key: String,
    machine_id: String
}

struct RunpodClientBuilder<T> {
    backend: T,
    api_base: Option<Url>,
    api_key: Option<String>,
    machine_id: Option<String>
}

impl<T> RunpodClientBuilder<T> { 
    pub fn new(backend: T) -> Self {
        RunpodClientBuilder::<T> {
            backend,
            api_base: None,
            api_key: None,
            machine_id: None
        }
    }
}

trait RunpodClientBuilderTrait<T> {
    fn with_api_base(self, api_base: Url) -> Self;
    fn with_api_key(self, api_key: String) -> Self;
    fn with_machine_id(self, machine_id: String) -> Self;
    fn build(self) -> RunpodClient<T>;
}

impl<T> RunpodClientBuilderTrait<T> for RunpodClientBuilder<T> {
    fn with_api_base(mut self, api_base: Url) -> Self {
        self.api_base = Some(api_base);
        self
    }

    fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    fn with_machine_id(mut self, machine_id: String) -> Self {
        self.machine_id = Some(machine_id);
        self
    }

    fn build(self) -> RunpodClient<T> {
        RunpodClient::<T> {
            backend: self.backend,
            api_base: self.api_base.unwrap_or(Url::parse(DEFAULT_API_BASE).unwrap()),
            api_key: self.api_key.unwrap_or_default(),
            machine_id: self.machine_id.unwrap_or_default()
        }
    }
}