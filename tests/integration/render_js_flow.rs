use std::{cell::RefCell, rc::Rc};

use ai_browser::engine::{
    dom::parser::parse_html,
    js::vm::Interpreter,
    layout::block_layout,
    paint::{build_display_list, raster::rasterize},
    webapi::dom_bindings::JsDocumentBinding,
};

#[test]
fn integration_render_js_pipeline() {
    let doc = parse_html("<html><body><h1>Hello</h1><p>Intro</p></body></html>").unwrap();
    let doc_ref = Rc::new(RefCell::new(doc));

    let binding = JsDocumentBinding::new(Rc::clone(&doc_ref));
    let mut vm = Interpreter::default();
    vm.install_dom_apis(binding);
    vm.eval(r#"dom_set_text("h1", "From JS");"#).unwrap();

    let layout = block_layout(&doc_ref.borrow(), 1024.0);
    let display = build_display_list(&layout);
    let frame = rasterize(&display, 1024, 768);
    assert!(frame.text_runs.iter().any(|t| t.contains("From JS")));
}
