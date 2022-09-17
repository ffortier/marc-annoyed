use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement};

pub struct Canvas {
    width: u32,
    height: u32,
    html_element: HtmlCanvasElement,
    rendering_context: CanvasRenderingContext2d,
}

impl Canvas {
    pub fn new(width: u32, height: u32, parent: &HtmlElement) -> Self {
        let html_element = create_canvas(width, height, parent);
        let rendering_context = html_element
            .get_context("2d")
            .unwrap_throw()
            .unwrap_throw()
            .unchecked_into::<CanvasRenderingContext2d>();

        Self {
            width,
            height,
            html_element,
            rendering_context,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl Default for Canvas {
    fn default() -> Self {
        let parent = window()
            .unwrap_throw()
            .document()
            .unwrap_throw()
            .body()
            .unwrap();

        Canvas::new(480, 360, &parent)
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        self.html_element.remove();
    }
}

fn create_canvas(width: u32, height: u32, parent: &HtmlElement) -> HtmlCanvasElement {
    let canvas = window()
        .unwrap_throw()
        .document()
        .unwrap_throw()
        .create_element("canvas")
        .unwrap_throw()
        .unchecked_into::<HtmlCanvasElement>();

    canvas.set_width(width);
    canvas.set_height(height);

    parent.append_child(&canvas).unwrap_throw();

    canvas
}
