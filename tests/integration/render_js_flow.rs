use std::{cell::RefCell, rc::Rc};

use ai_browser::engine::{
    dom::parser::parse_html,
    js::vm::Interpreter,
    layout::block_layout,
    paint::{build_display_list, raster::rasterize},
    webapi::dom_bindings::JsDocumentBinding,
};

const SAMPLE_HTML: &str = "<html><body><h1>Hello</h1><p>Intro</p></body></html>";
const DOM_SCRIPT: &str = r#"dom_set_text("h1", "From JS");"#;

fn setup_vm_with_dom() -> (
    Rc<RefCell<ai_browser::engine::dom::node::Document>>,
    Interpreter,
) {
    let doc = parse_html(SAMPLE_HTML).unwrap();
    let doc_ref = Rc::new(RefCell::new(doc));
    let binding = JsDocumentBinding::new(Rc::clone(&doc_ref));
    let mut vm = Interpreter::default();
    vm.install_dom_apis(binding);
    vm.eval(DOM_SCRIPT).unwrap();
    (doc_ref, vm)
}

#[test]
fn integration_render_js_pipeline() {
    let (doc_ref, _vm) = setup_vm_with_dom();
    let layout = block_layout(&doc_ref.borrow(), 1024.0);
    let display = build_display_list(&layout);
    let frame = rasterize(&display, 1024, 768);
    assert!(frame.text_runs.iter().any(|t| t.contains("From JS")));
}

#[test]
fn full_flow_html_js_layout_paint() {
    let (doc_ref, _vm) = setup_vm_with_dom();
    let layout = block_layout(&doc_ref.borrow(), 1200.0);
    let draw = build_display_list(&layout);
    assert!(!draw.is_empty());
    assert!(draw.iter().any(|cmd| cmd.text.contains("From JS")));
}
