use crate::app::ai::{client::AiClient, page_context::PageContext};

pub fn summarize_page(client: &AiClient, ctx: &PageContext) -> anyhow::Result<String> {
    let system = "你是浏览器内置助手。请输出结构化中文摘要，包含：主题、要点、结论。";
    let user = format!(
        "URL: {}\nTitle: {}\nContent:\n{}",
        ctx.url, ctx.title, ctx.visible_text
    );
    client.chat(system, &user)
}
