use crate::app::{
    ai::{client::AiClient, page_context::PageContext},
    security::prompt_guard::sanitize_page_context,
};

pub fn ask_page(client: &AiClient, ctx: &PageContext, question: &str) -> anyhow::Result<String> {
    let system = "你是浏览器内置问答助手。严格依据页面内容回答，不确定时明确说不知道。";
    let sanitized = sanitize_page_context(&ctx.visible_text);
    let user = format!(
        "页面URL: {}\n页面标题: {}\n页面正文:\n{}\n\n问题: {}",
        ctx.url, ctx.title, sanitized, question
    );
    client.chat(system, &user)
}
