

pub struct MessageConfig {
    pub model: String,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>, // 0..1
    pub stream: Option<bool>,
    pub max_tokens: Option<u32>,
    pub repetition_penalty: Option<f32>,
}


impl Default for MessageConfig {
    fn default() -> Self {
        Self { model: "GigaChat".to_owned(), temperature: None, top_p: None, stream: None, max_tokens: None, repetition_penalty: None }
    }
}

#[derive(Default)]
pub struct MessageConfigBuilder {
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>, // 0..1
    pub stream: Option<bool>,
    pub max_tokens: Option<u32>,
    pub repetition_penalty: Option<f32>,
}

impl MessageConfigBuilder {

    pub fn new() -> Self {
        Self { model: None, temperature: None, top_p: None, stream: None, max_tokens: None, repetition_penalty: None }
    }
    pub fn set_model(mut self, model: &str) -> Self {
        self.model = model.to_owned().into();
        self
    }
    pub fn set_temp(mut self, temp: f32) -> Self {
        self.temperature = temp.into();
        self
    }
    pub fn set_top_p(mut self, top_p: f32) -> Self {
        self.top_p = top_p.into();
        self
    }
    pub fn set_stream(mut self, stream: bool) -> Self {
        self.stream = stream.into();
        self
    }
    pub fn set_max_tokens(mut self, tokens: u32) -> Self {
        self.max_tokens = tokens.into();
        self
    }
    pub fn set_rep_penalty(mut self, penalty: f32) -> Self {
        self.repetition_penalty = penalty.into();
        self
    }
    pub fn build(self) -> MessageConfig {
        MessageConfig { model: self.model.expect("Model should be set"), 
        temperature: self.temperature, 
        top_p: self.top_p, stream: self.stream, max_tokens: self.max_tokens, repetition_penalty: self.repetition_penalty }
    }
}