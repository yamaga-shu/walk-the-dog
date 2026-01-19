use web_sys::{console, window};

fn main() {
    browser_panic_hook::set_once_default();

    console::log_1(&"Hello, console form Rust".into());

    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access the document");
    let body = document.body().expect("Could not access document.body");
    let text_node = document.create_text_node("Hello, world from Vanilla Rust!");
    body.append_child(text_node.as_ref())
        .expect("Failed to append text");
}
