use boa_engine::{js_string, property::Attribute, Context, Source};
use boa_runtime::Console;

pub fn test_console(code: &str, output: &str) {
    let mut context = Context::default();
    let console = Console::init(&mut context);
    context
        .register_global_property(js_string!(Console::NAME), console, Attribute::all())
        .expect("register_global_property console error");

    let hack_code = Source::from_bytes(include_bytes!("./console/deno-scripts/02_hack_print.js"));
    context.eval(hack_code).expect("msg");

    context
        .eval(Source::from_bytes(code.as_bytes()))
        .expect("msg");
    let output_value = context
        .global_object()
        .get(js_string!("__hack_print_output"), &mut context)
        .expect("msg");
    let output_string: String = output_value
        .as_string()
        .expect("msg")
        .to_std_string()
        .expect("msg");
    assert_eq!(output_string, output);
}

fn main() {
    test_console("console.log(['', \"我啊啊\"])", "");
}
