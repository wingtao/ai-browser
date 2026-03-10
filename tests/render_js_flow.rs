use std::{cell::RefCell, rc::Rc};

use ai_browser::engine::js::vm::Interpreter;
use ai_browser::engine::{
    dom::parser::parse_html, layout::block_layout, paint::build_display_list,
    webapi::dom_bindings::JsDocumentBinding,
};

#[test]
fn full_flow_html_js_layout_paint() {
    let doc = parse_html("<html><body><h1>Hello</h1><p>Intro</p></body></html>").unwrap();
    let doc_ref = Rc::new(RefCell::new(doc));

    let binding = JsDocumentBinding::new(Rc::clone(&doc_ref));
    let mut vm = Interpreter::default();
    vm.install_dom_apis(binding);
    vm.eval(r#"dom_set_text("h1", "From JS");"#).unwrap();

    let layout = block_layout(&doc_ref.borrow(), 1200.0);
    let draw = build_display_list(&layout);
    assert!(!draw.is_empty());
    assert!(draw.iter().any(|cmd| cmd.text.contains("From JS")));
}
