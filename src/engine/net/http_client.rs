use anyhow::Context;

use super::url::parse_url;

#[derive(Debug, Clone)]
pub struct HttpClient {
    client: reqwest::blocking::Client,
}

impl Default for HttpClient {
    fn default() -> Self {
        let client = reqwest::blocking::Client::builder()
            .user_agent("ai-browser/0.1")
            .build()
            .expect("http client should build");
        Self { client }
    }
}

impl HttpClient {
    pub fn get_text(&self, url: &str) -> anyhow::Result<String> {
        parse_url(url).context("URL 格式非法")?;
        let response = self.client.get(url).send()?.error_for_status()?;
        response.text().context("响应体解码失败")
    }
}
