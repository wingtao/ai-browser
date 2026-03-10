pub mod address_bar;
pub mod navigation;
pub mod tab_manager;

use std::{cell::RefCell, rc::Rc};

use anyhow::Context;

use crate::engine::{
    dom::parser::parse_html, js::vm::Interpreter, net::http_client::HttpClient,
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
        Some("window") => run_window(),
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
    println!("  cargo run -- window          # 打开最小窗口事件循环");
}

fn run_window() -> anyhow::Result<()> {
    use winit::{
        application::ApplicationHandler,
        event::WindowEvent,
        event_loop::{ActiveEventLoop, EventLoop},
        window::{Window, WindowAttributes, WindowId},
    };

    #[derive(Default)]
    struct App {
        window: Option<Window>,
    }

    impl ApplicationHandler for App {
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            let attrs = WindowAttributes::default().with_title("ai-browser kernel shell");
            if let Ok(window) = event_loop.create_window(attrs) {
                self.window = Some(window);
            }
        }

        fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            _window_id: WindowId,
            event: WindowEvent,
        ) {
            if let WindowEvent::CloseRequested = event {
                event_loop.exit();
            }
        }
    }

    let event_loop = EventLoop::new()?;
    let mut app = App::default();
    event_loop.run_app(&mut app)?;
    Ok(())
}
