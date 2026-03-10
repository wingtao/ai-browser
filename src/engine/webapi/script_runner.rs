use std::{cell::RefCell, rc::Rc};

use anyhow::Context;

use crate::engine::{dom::node::Document, js::vm::Interpreter};

use super::dom_bindings::JsDocumentBinding;

pub fn run_inline_scripts(doc: Rc<RefCell<Document>>) -> anyhow::Result<()> {
    let scripts = doc.borrow().collect_text_by_tag("script");
    if scripts.is_empty() {
        return Ok(());
    }

    let binding = JsDocumentBinding::new(Rc::clone(&doc));
    let mut vm = Interpreter::default();
    vm.install_dom_apis(binding);

    for (idx, script) in scripts.into_iter().enumerate() {
        if script.trim().is_empty() {
            continue;
        }
        vm.eval(&script)
            .with_context(|| format!("执行内联脚本失败（索引 {idx}）"))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::run_inline_scripts;
    use crate::engine::dom::parser::parse_html;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn execute_inline_script_and_update_dom() {
        let doc = parse_html(
            r#"
            <html>
              <body>
                <h1>Hello</h1>
                <script>dom_set_text("h1", "FromInline");</script>
              </body>
            </html>
        "#,
        )
        .unwrap();
        let doc = Rc::new(RefCell::new(doc));
        run_inline_scripts(Rc::clone(&doc)).unwrap();
        let h1 = doc.borrow().collect_text_by_tag("h1");
        assert_eq!(h1.first().map(String::as_str), Some("FromInline"));
    }
}
