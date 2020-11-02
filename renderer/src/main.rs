use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod utils;


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
    log("teste")
}

#[wasm_bindgen]
pub fn get_canvas(canvas: web_sys::HtmlCanvasElement) {

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    
    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, std::f64::consts::PI * 2.0)
        .unwrap();

    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, std::f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, std::f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, std::f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();
}
