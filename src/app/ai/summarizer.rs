use crate::app::{
    ai::{client::AiClient, page_context::PageContext},
    security::prompt_guard::sanitize_page_context,
};

pub fn summarize_page(client: &AiClient, ctx: &PageContext) -> anyhow::Result<String> {
    let system = "你是浏览器内置助手。请输出结构化中文摘要，包含：主题、要点、结论。";
    let sanitized = sanitize_page_context(&ctx.visible_text);
    let user = format!(
        "URL: {}\nTitle: {}\nContent:\n{}",
        ctx.url, ctx.title, sanitized
    );
    client.chat(system, &user)
}
