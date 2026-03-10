use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct AiClientConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
}

#[derive(Debug, Clone)]
pub struct AiClient {
    config: AiClientConfig,
    http: reqwest::blocking::Client,
}

#[derive(Debug, Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: AssistantMessage,
}

#[derive(Debug, Deserialize)]
struct AssistantMessage {
    content: String,
}

impl AiClient {
    pub fn from_env() -> anyhow::Result<Self> {
        let api_key = std::env::var("AI_BROWSER_API_KEY").context("缺少 AI_BROWSER_API_KEY")?;
        let base_url = std::env::var("AI_BROWSER_BASE_URL")
            .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
        let model = std::env::var("AI_BROWSER_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string());
        Ok(Self {
            config: AiClientConfig {
                api_key,
                base_url,
                model,
            },
            http: reqwest::blocking::Client::new(),
        })
    }

    pub fn chat(&self, system_prompt: &str, user_prompt: &str) -> anyhow::Result<String> {
        let req = ChatRequest {
            model: &self.config.model,
            messages: vec![
                Message {
                    role: "system",
                    content: system_prompt,
                },
                Message {
                    role: "user",
                    content: user_prompt,
                },
            ],
        };
        let endpoint = format!(
            "{}/chat/completions",
            self.config.base_url.trim_end_matches('/')
        );
        let response = self
            .http
            .post(endpoint)
            .bearer_auth(&self.config.api_key)
            .json(&req)
            .send()?
            .error_for_status()?;
        let body: ChatResponse = response.json()?;
        body.choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| anyhow!("AI 返回为空"))
    }
}
