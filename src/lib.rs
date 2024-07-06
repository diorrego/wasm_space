use js_sys::Math::random;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let win = window().expect("should have a window in this context");
    let document = win.document().expect("window should have a document");
    let body = document.body().expect("document should have a body");

    let canvas: HtmlCanvasElement = document.create_element("canvas")?.dyn_into()?;
    update_canvas_size(&canvas, &win);
    body.append_child(&canvas)?;

    let context = Rc::new(
        canvas
            .get_context("2d")?
            .ok_or("Failed to get 2d context")?
            .dyn_into::<CanvasRenderingContext2d>()?,
    );

    let stars = create_stars(1000, canvas.width(), canvas.height());
    let mut stars_clone = stars.clone();

    let context_for_mouse = Rc::clone(&context);
    let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
        let mouse_x = event.client_x() as f64;
        let mouse_y = event.client_y() as f64;
        draw_stars_with_movement(&context_for_mouse, &mut stars_clone[..], mouse_x, mouse_y);
    }) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
    closure.forget();

    // Handle window resize
    let win_clone = win.clone();
    let resize_closure = Closure::wrap(Box::new(move || {
        update_canvas_size(&canvas, &win_clone);
        draw_stars(&context, &stars);
    }) as Box<dyn FnMut()>);

    win.add_event_listener_with_callback("resize", resize_closure.as_ref().unchecked_ref())?;
    resize_closure.forget();

    Ok(())
}

fn update_canvas_size(canvas: &HtmlCanvasElement, window: &web_sys::Window) {
    canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
    canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);
}

#[derive(Clone)]
struct Star {
    x: f64,
    y: f64,
    size: f64,
}

fn create_stars(count: usize, width: u32, height: u32) -> Vec<Star> {
    (0..count)
        .map(|_| Star {
            x: random() * width as f64,
            y: random() * height as f64,
            size: random() * 3.0 + 1.0,
        })
        .collect()
}

fn draw_stars(context: &CanvasRenderingContext2d, stars: &[Star]) {
    let width = context.canvas().unwrap().width() as f64;
    let height = context.canvas().unwrap().height() as f64;
    context.set_fill_style(&JsValue::from_str("black"));
    context.fill_rect(0.0, 0.0, width, height);
    for star in stars {
        context.begin_path();
        let gradient = context
            .create_radial_gradient(star.x, star.y, star.size * 0.1, star.x, star.y, star.size)
            .unwrap();
        gradient.add_color_stop(0.0, "white").unwrap();
        gradient
            .add_color_stop(1.0, "rgba(255, 255, 255, 0)")
            .unwrap();
        context.set_fill_style(&gradient);
        context
            .arc(star.x, star.y, star.size, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();
        context.fill();
    }
}

fn draw_stars_with_movement(
    context: &CanvasRenderingContext2d,
    stars: &mut [Star],
    mouse_x: f64,
    mouse_y: f64,
) {
    let radius = 50.0;
    let width = context.canvas().unwrap().width() as f64;
    let height = context.canvas().unwrap().height() as f64;
    context.set_fill_style(&JsValue::from_str("black"));
    context.fill_rect(0.0, 0.0, width, height);
    for star in stars.iter_mut() {
        let distance = ((mouse_x - star.x).powi(2) + (mouse_y - star.y).powi(2)).sqrt();
        if distance < radius {
            let angle = (mouse_y - star.y).atan2(mouse_x - star.x);
            let dx = angle.cos() * 5.0;
            let dy = angle.sin() * 5.0;
            star.x += dx;
            star.y += dy;
        }
        context.begin_path();
        let gradient = context
            .create_radial_gradient(star.x, star.y, star.size * 0.1, star.x, star.y, star.size)
            .unwrap();
        gradient.add_color_stop(0.0, "white").unwrap();
        gradient
            .add_color_stop(1.0, "rgba(255, 255, 255, 0)")
            .unwrap();
        context.set_fill_style(&gradient);
        context
            .arc(star.x, star.y, star.size, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();
        context.fill();
    }
}
