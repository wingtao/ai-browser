use crate::engine::js::runtime::value::Value;

pub fn format_console_args(args: &[Value]) -> String {
    args.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
}
