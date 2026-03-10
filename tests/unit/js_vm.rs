use ai_browser::engine::js::vm::Interpreter;

#[test]
fn unit_vm_should_eval_for_loop() {
    let mut vm = Interpreter::default();
    let out = vm
        .eval(
            r#"
            let sum = 0;
            for (let i = 0; i < 4; i = i + 1) {
                sum = sum + i;
            }
            sum;
        "#,
        )
        .unwrap();
    assert_eq!(out.to_string(), "6");
}
