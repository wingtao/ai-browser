use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::Context;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::Key,
    window::{Window, WindowAttributes, WindowId},
};

use crate::engine::{
    dom::parser::parse_html, net::http_client::HttpClient,
    webapi::script_runner::run_inline_scripts,
};

use super::{navigation::NavigationState, tab_manager::TabManager};

#[derive(Debug, Clone)]
struct PageSnapshot {
    title: String,
    url: String,
    text_preview: String,
    last_error: Option<String>,
}

#[derive(Debug, Default)]
struct BrowserRuntime {
    tabs: TabManager,
    navs: HashMap<u64, NavigationState>,
    pages: HashMap<u64, PageSnapshot>,
    client: HttpClient,
}

impl BrowserRuntime {
    fn open_new_tab(&mut self, url: &str) -> anyhow::Result<()> {
        let tab_id = self.tabs.create_tab(url.to_string());
        self.navs.entry(tab_id).or_default();
        self.navigate(tab_id, url)
    }

    fn active_tab_id(&self) -> Option<u64> {
        self.tabs.active_tab().map(|t| t.id)
    }

    fn switch_to_index(&mut self, idx: usize) -> bool {
        let tabs = self.tabs.all_tabs();
        if let Some(tab) = tabs.get(idx) {
            return self.tabs.switch_to(tab.id);
        }
        false
    }

    fn close_active(&mut self) {
        if let Some(id) = self.active_tab_id() {
            self.tabs.close_tab(id);
        }
    }

    fn navigate_active(&mut self, url: &str) -> anyhow::Result<()> {
        let id = self
            .active_tab_id()
            .context("当前没有活动标签页，无法导航")?;
        self.navigate(id, url)
    }

    fn reload_active(&mut self) -> anyhow::Result<()> {
        let id = self
            .active_tab_id()
            .context("当前没有活动标签页，无法刷新")?;
        let url = self
            .tabs
            .active_tab()
            .map(|t| t.url.clone())
            .context("活动标签页 URL 缺失")?;
        self.load_url(id, &url)
    }

    fn back_active(&mut self) -> anyhow::Result<()> {
        let id = self.active_tab_id().context("当前没有活动标签页")?;
        let next = self
            .navs
            .entry(id)
            .or_default()
            .back()
            .context("没有可后退历史")?;
        self.load_url(id, &next)
    }

    fn forward_active(&mut self) -> anyhow::Result<()> {
        let id = self.active_tab_id().context("当前没有活动标签页")?;
        let next = self
            .navs
            .entry(id)
            .or_default()
            .forward()
            .context("没有可前进历史")?;
        self.load_url(id, &next)
    }

    fn navigate(&mut self, tab_id: u64, url: &str) -> anyhow::Result<()> {
        self.navs
            .entry(tab_id)
            .or_default()
            .navigate(url.to_string());
        self.load_url(tab_id, url)
    }

    fn load_url(&mut self, tab_id: u64, url: &str) -> anyhow::Result<()> {
        self.tabs.update_url(tab_id, url.to_string());
        if url == "about:blank" {
            self.tabs.update_title(tab_id, "New Tab");
            self.pages.insert(
                tab_id,
                PageSnapshot {
                    title: "New Tab".to_string(),
                    url: url.to_string(),
                    text_preview: String::new(),
                    last_error: None,
                },
            );
            return Ok(());
        }

        match self
            .client
            .get_text(url)
            .with_context(|| format!("请求页面失败: {url}"))
        {
            Ok(html) => {
                let doc = Rc::new(RefCell::new(parse_html(&html)?));
                let _ = run_inline_scripts(Rc::clone(&doc));
                let doc = doc.borrow();
                let title = doc.find_title().unwrap_or_else(|| "Untitled".to_string());
                let preview = doc
                    .visible_text(240)
                    .replace('\n', " | ")
                    .chars()
                    .take(240)
                    .collect::<String>();
                self.tabs.update_title(tab_id, title.clone());
                self.pages.insert(
                    tab_id,
                    PageSnapshot {
                        title: title.clone(),
                        url: url.to_string(),
                        text_preview: preview.clone(),
                        last_error: None,
                    },
                );
                println!("[tab#{tab_id}] {title} => {preview}");
                Ok(())
            }
            Err(err) => {
                let msg = err.to_string();
                self.tabs.update_title(tab_id, "Load Error");
                self.pages.insert(
                    tab_id,
                    PageSnapshot {
                        title: "Load Error".to_string(),
                        url: url.to_string(),
                        text_preview: String::new(),
                        last_error: Some(msg.clone()),
                    },
                );
                Err(err)
            }
        }
    }

    fn title_line(&self) -> String {
        let total = self.tabs.all_tabs().len();
        if let Some(tab) = self.tabs.active_tab() {
            let snapshot = self.pages.get(&tab.id);
            let err_flag = snapshot
                .and_then(|s| s.last_error.as_ref())
                .map(|_| " [ERR]")
                .unwrap_or("");
            format!(
                "ai-browser | tabs:{total} | active:{} | {} | {}{}",
                tab.id, tab.title, tab.url, err_flag
            )
        } else {
            "ai-browser | no tabs".to_string()
        }
    }

    fn status_line(&self) -> String {
        if let Some(tab) = self.tabs.active_tab() {
            if let Some(page) = self.pages.get(&tab.id) {
                if let Some(err) = &page.last_error {
                    return format!("error: {err}");
                }
                if !page.text_preview.is_empty() {
                    return format!("{} ({}) => {}", page.title, page.url, page.text_preview);
                }
            }
            return format!("{} => {}", tab.title, tab.url);
        }
        "无活动标签页".to_string()
    }
}

struct WindowBrowserApp {
    window: Option<Window>,
    runtime: BrowserRuntime,
    startup_url: String,
}

impl WindowBrowserApp {
    fn new(startup_url: String) -> Self {
        Self {
            window: None,
            runtime: BrowserRuntime::default(),
            startup_url,
        }
    }

    fn refresh_window_title(&self) {
        if let Some(window) = &self.window {
            let title = format!(
                "{} | [N新建 O打开 R刷新 B后退 F前进 W关闭 1-9切换 Q退出]",
                self.runtime.title_line()
            );
            window.set_title(&title);
        }
    }

    fn print_status(&self) {
        println!("{}", self.runtime.status_line());
    }

    fn handle_key(&mut self, event_loop: &ActiveEventLoop, key: &Key) {
        let ch = match key {
            Key::Character(c) => c.to_lowercase(),
            _ => return,
        };
        let result = match ch.as_str() {
            "n" => self.runtime.open_new_tab("about:blank"),
            "o" => self.runtime.navigate_active("http://example.com"),
            "r" => self.runtime.reload_active(),
            "b" => self.runtime.back_active(),
            "f" => self.runtime.forward_active(),
            "w" => {
                self.runtime.close_active();
                Ok(())
            }
            "q" => {
                event_loop.exit();
                Ok(())
            }
            "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                let idx = ch.parse::<usize>().unwrap_or(1).saturating_sub(1);
                self.runtime.switch_to_index(idx);
                Ok(())
            }
            _ => Ok(()),
        };
        if let Err(err) = result {
            println!("操作失败: {err}");
        }
        self.refresh_window_title();
        self.print_status();
    }
}

impl ApplicationHandler for WindowBrowserApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let attrs = WindowAttributes::default().with_title("ai-browser shell");
            if let Ok(window) = event_loop.create_window(attrs) {
                self.window = Some(window);
            }
        }
        if self.runtime.tabs.active_tab().is_none() {
            if let Err(err) = self.runtime.open_new_tab(&self.startup_url) {
                println!("初始加载失败: {err}");
            }
        }
        self.refresh_window_title();
        self.print_status();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed && !event.repeat {
                    self.handle_key(event_loop, &event.logical_key);
                }
            }
            _ => {}
        }
    }
}

pub fn run_window(start_url: Option<String>) -> anyhow::Result<()> {
    let startup_url = start_url.unwrap_or_else(|| "http://example.com".to_string());
    let event_loop = EventLoop::new()?;
    let mut app = WindowBrowserApp::new(startup_url);
    event_loop.run_app(&mut app)?;
    Ok(())
}
