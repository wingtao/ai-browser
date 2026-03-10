use crate::app::ai::{client::AiClient, page_context::PageContext};

pub fn ask_page(client: &AiClient, ctx: &PageContext, question: &str) -> anyhow::Result<String> {
    let system = "你是浏览器内置问答助手。严格依据页面内容回答，不确定时明确说不知道。";
    let user = format!(
        "页面URL: {}\n页面标题: {}\n页面正文:\n{}\n\n问题: {}",
        ctx.url, ctx.title, ctx.visible_text, question
    );
    client.chat(system, &user)
}
