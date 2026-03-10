use std::{cell::RefCell, rc::Rc};

use ai_browser::engine::{dom::parser::parse_html, js::vm::Interpreter, webapi::dom_bindings::JsDocumentBinding};

#[test]
fn timer_task_flow() {
    let mut vm = Interpreter::default();
    let out = vm
        .eval(
            r#"
            let state = 0;
            function update() { state = 9; }
            setTimeout(update, 0);
            runTasks();
            state;
        "#,
        )
        .unwrap();
    assert_eq!(out.to_string(), "9");
}

#[test]
fn dom_event_bubble_flow() {
    let doc = parse_html("<html><body><div><button>Go</button></div></body></html>").unwrap();
    let binding = JsDocumentBinding::new(Rc::new(RefCell::new(doc)));
    let mut vm = Interpreter::default();
    vm.install_dom_apis(binding);

    let out = vm
        .eval(
            r#"
            let score = 0;
            function onButton() { score = score + 1; }
            function onBody() { score = score + 10; }
            dom_add_event_listener("button", "click", onButton);
            dom_add_event_listener("body", "click", onBody);
            dom_dispatch_event("button", "click");
            score;
        "#,
        )
        .unwrap();
    assert_eq!(out.to_string(), "11");
}
