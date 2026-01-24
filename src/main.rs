use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d, HtmlImageElement};
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
    
    fn midpoint(&self, other: &Point) -> Point {
        Point::new((self.x + other.x) / 2.0, (self.y + other.y) / 2.0)
    }
}

struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

impl Triangle {
    fn equilateral(top: Point, side_length: f64) -> Self {
        let height = side_length * (3_f64.sqrt() / 2.0);
        Triangle {
            p1: top,
            p2: Point::new(top.x - side_length / 2.0, top.y + height),
            p3: Point::new(top.x + side_length / 2.0, top.y + height),
        }
    }
    
    fn draw(&self, context: &CanvasRenderingContext2d) {
        context.begin_path();
        context.move_to(self.p1.x, self.p1.y);
        context.line_to(self.p2.x, self.p2.y);
        context.line_to(self.p3.x, self.p3.y);
        context.close_path();
        context.fill();
    }
}

fn draw_sierpinski_triangle(
    context: &CanvasRenderingContext2d,
    top: Point,
    side_length: f64,
    level: u32,
) {
    let triangle = Triangle::equilateral(top, side_length);
    draw_sierpinski_triangle_internal(context, triangle.p1, triangle.p2, triangle.p3, level);
}

fn draw_sierpinski_triangle_internal(
    context: &CanvasRenderingContext2d,
    p1: Point,
    p2: Point,
    p3: Point,
    level: u32,
) {
    if level == 0 {
        let triangle = Triangle { p1, p2, p3 };
        triangle.draw(context);
    } else {
        // 各辺の中点を計算
        let mid12 = p1.midpoint(&p2);
        let mid23 = p2.midpoint(&p3);
        let mid31 = p3.midpoint(&p1);

        // 3つの部分三角形を再帰的に描画
        draw_sierpinski_triangle_internal(context, p1, mid12, mid31, level - 1);
        draw_sierpinski_triangle_internal(context, mid12, p2, mid23, level - 1);
        draw_sierpinski_triangle_internal(context, mid31, mid23, p3, level - 1);
    }
}

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
    canvas.set_height(1000);
    
    // canvasを左右中央に配置するためにスタイルを設定
    canvas.set_attribute("style", "display: block; margin: 0 auto;").expect("Failed to set style attribute");
    
    body.append_child(&canvas).expect("Failed to append canvas");
    
    let context: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .expect("Could not get 2d context")
        .expect("2d context is None")
        .dyn_into()
        .expect("Could not convert to CanvasRenderingContext2d");
    
    // 黒い色で塗りつぶす設定
    context.set_fill_style_str("black");
    
    // 上頂点座標(400, 100)、辺の長さ500、レベル2
    draw_sierpinski_triangle(&context, Point::new(400.0, 100.0), 600.0, 6);
    
    // 画像を読み込んで描画
    let img: HtmlImageElement = document
        .create_element("img")
        .expect("Could not create img element")
        .dyn_into()
        .expect("Could not convert to HtmlImageElement");
    
    let context_clone = context.clone();
    let img_clone = img.clone();
    
    let onload_closure = Closure::wrap(Box::new(move || {
        // シェルピンスキーの三角形の左上は (400 - 600/2, 100) = (100, 100)
        context_clone
            .draw_image_with_html_image_element(&img_clone, 100.0, 100.0)
            .expect("Failed to draw image");
    }) as Box<dyn FnMut()>);
    
    img.set_onload(Some(onload_closure.as_ref().unchecked_ref()));
    img.set_src("static/Idle.png");
    
    onload_closure.forget();
}
