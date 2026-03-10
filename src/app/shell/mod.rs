pub mod address_bar;
pub mod navigation;
pub mod tab_manager;
pub mod window_browser;

use std::{cell::RefCell, rc::Rc};

use anyhow::Context;

use crate::app::ai::{
    client::AiClient, page_context::PageContext, page_qa::ask_page, summarizer::summarize_page,
};
use crate::app::data::{bookmark_repo::BookmarkRepository, history_repo::HistoryRepository};
use crate::engine::{
    dom::parser::parse_html,
    js::{bytecode::eval_via_bytecode, vm::Interpreter},
    net::http_client::HttpClient,
    webapi::script_runner::run_inline_scripts,
};

pub fn run() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    match args.get(1).map(|s| s.as_str()) {
        Some("load") => {
            let url = args
                .get(2)
                .context("用法: cargo run -- load <url>")?
                .to_string();
            let html = HttpClient::default().get_text(&url)?;
            let doc = Rc::new(RefCell::new(parse_html(&html)?));
            run_inline_scripts(Rc::clone(&doc))?;
            let doc_ref = doc.borrow();
            println!(
                "title: {}",
                doc_ref.find_title().unwrap_or_else(|| "N/A".to_string())
            );
            println!("text-preview:\n{}", doc_ref.visible_text(800));
            Ok(())
        }
        Some("js") => {
            let script = args
                .get(2)
                .context("用法: cargo run -- js \"let a=1; a+1;\"")?;
            let mut interpreter = Interpreter::default();
            let result = interpreter.eval(script)?;
            println!("result: {result}");
            Ok(())
        }
        Some("js-bc") => {
            let script = args
                .get(2)
                .context("用法: cargo run -- js-bc \"let a=1; a+1;\"")?;
            let result = eval_via_bytecode(script)?;
            println!("result: {result}");
            Ok(())
        }
        Some("js-dom") => {
            let html = args
                .get(2)
                .context("用法: cargo run -- js-dom \"<html>...</html>\" \"script\"")?;
            let script = args
                .get(3)
                .context("用法: cargo run -- js-dom \"<html>...</html>\" \"script\"")?;
            let doc = parse_html(html)?;
            let doc = Rc::new(RefCell::new(doc));
            let binding =
                crate::engine::webapi::dom_bindings::JsDocumentBinding::new(Rc::clone(&doc));
            let mut interpreter = Interpreter::default();
            interpreter.install_dom_apis(binding);
            let result = interpreter.eval(script)?;
            println!("result: {result}");
            Ok(())
        }
        Some("window") => run_window(args.get(2).cloned()),
        Some("history-add") => {
            let url = args
                .get(2)
                .context("用法: cargo run -- history-add <url> <title>")?;
            let title = args
                .get(3..)
                .map(|parts| parts.join(" "))
                .filter(|s| !s.trim().is_empty())
                .context("用法: cargo run -- history-add <url> <title>")?;
            let repo = HistoryRepository::open(&db_path())?;
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs() as i64;
            repo.add_visit(url, &title, now)?;
            println!("ok");
            Ok(())
        }
        Some("history-list") => {
            let limit = args
                .get(2)
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(20);
            let repo = HistoryRepository::open(&db_path())?;
            for item in repo.list_recent(limit)? {
                println!(
                    "[{}] {} | {} | {}",
                    item.id, item.visited_at, item.title, item.url
                );
            }
            Ok(())
        }
        Some("history-clear") => {
            let repo = HistoryRepository::open(&db_path())?;
            repo.clear()?;
            println!("ok");
            Ok(())
        }
        Some("bookmark-add") => {
            let url = args
                .get(2)
                .context("用法: cargo run -- bookmark-add <url> <title>")?;
            let title = args
                .get(3..)
                .map(|parts| parts.join(" "))
                .filter(|s| !s.trim().is_empty())
                .context("用法: cargo run -- bookmark-add <url> <title>")?;
            let repo = BookmarkRepository::open(&db_path())?;
            repo.add(&title, url)?;
            println!("ok");
            Ok(())
        }
        Some("bookmark-list") => {
            let repo = BookmarkRepository::open(&db_path())?;
            for item in repo.list()? {
                println!("[{}] {} | {}", item.id, item.title, item.url);
            }
            Ok(())
        }
        Some("bookmark-remove") => {
            let url = args
                .get(2)
                .context("用法: cargo run -- bookmark-remove <url>")?;
            let repo = BookmarkRepository::open(&db_path())?;
            repo.remove_by_url(url)?;
            println!("ok");
            Ok(())
        }
        Some("summarize") => {
            let url = args
                .get(2)
                .context("用法: cargo run -- summarize <url>")?
                .to_string();
            let html = HttpClient::default().get_text(&url)?;
            let doc = Rc::new(RefCell::new(parse_html(&html)?));
            run_inline_scripts(Rc::clone(&doc))?;
            let context = PageContext::from_document(url, &doc.borrow());
            let client = AiClient::from_env()?;
            let summary = summarize_page(&client, &context)?;
            println!("{summary}");
            Ok(())
        }
        Some("ask") => {
            let url = args
                .get(2)
                .context("用法: cargo run -- ask <url> <问题>")?
                .to_string();
            let question = args
                .get(3..)
                .map(|parts| parts.join(" "))
                .filter(|s| !s.trim().is_empty())
                .context("用法: cargo run -- ask <url> <问题>")?;
            let html = HttpClient::default().get_text(&url)?;
            let doc = Rc::new(RefCell::new(parse_html(&html)?));
            run_inline_scripts(Rc::clone(&doc))?;
            let context = PageContext::from_document(url, &doc.borrow());
            let client = AiClient::from_env()?;
            let answer = ask_page(&client, &context, &question)?;
            println!("{answer}");
            Ok(())
        }
        _ => {
            print_usage();
            Ok(())
        }
    }
}

fn print_usage() {
    println!("ai-browser (from scratch)");
    println!("usage:");
    println!("  cargo run -- load <url>      # 拉取网页、执行内联脚本并输出文本预览");
    println!("  cargo run -- js <script>     # 运行自研 JS 引擎脚本");
    println!("  cargo run -- js-bc <script>  # 运行字节码解释路径（子集）");
    println!("  cargo run -- js-dom <html> <script> # 在DOM上下文运行JS");
    println!("  cargo run -- history-add <url> <title>");
    println!("  cargo run -- history-list [limit]");
    println!("  cargo run -- history-clear");
    println!("  cargo run -- bookmark-add <url> <title>");
    println!("  cargo run -- bookmark-list");
    println!("  cargo run -- bookmark-remove <url>");
    println!("  cargo run -- summarize <url> # 使用 AI 总结网页");
    println!("  cargo run -- ask <url> <问题> # 基于网页上下文进行问答");
    println!("  cargo run -- window [url]    # 打开窗口壳并加载URL");
}

fn db_path() -> String {
    std::env::var("AI_BROWSER_DB").unwrap_or_else(|_| "ai_browser.db".to_string())
}

fn run_window(start_url: Option<String>) -> anyhow::Result<()> {
    window_browser::run_window(start_url)
}
