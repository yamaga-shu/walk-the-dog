use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d, HtmlElement};
use wasm_bindgen::JsCast;

fn main() {
    browser_panic_hook::set_once_default();

    let window = window().expect("Could not get window");
    let document = window.document().expect("Could not get document");
    let body = document.body().expect("Could not access document.body");
    
    // Canvas要素を作成
    let canvas: HtmlCanvasElement = document
        .create_element("canvas")
        .expect("Could not create canvas element")
        .dyn_into()
        .expect("Could not convert to HtmlCanvasElement");
    
    canvas.set_width(800);
    canvas.set_height(600);
    
    // canvasを左右中央に配置するためにスタイルを設定
    canvas.set_attribute("style", "display: block; margin: 0 auto;").expect("Failed to set style attribute");
    
    body.append_child(&canvas).expect("Failed to append canvas");
    
    let context: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .expect("Could not get 2d context")
        .expect("2d context is None")
        .dyn_into()
        .expect("Could not convert to CanvasRenderingContext2d");
    
    // 黒い三角形を描画
    context.set_fill_style_str("black");
    
    context.begin_path();
    context.move_to(400.0, 100.0);    // 上の頂点
    context.line_to(200.0, 500.0);    // 左の頂点
    context.line_to(600.0, 500.0);    // 右の頂点
    context.close_path();
    context.stroke();
    context.fill();
}
