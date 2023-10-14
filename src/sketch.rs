use std::cell::RefCell;

use palette::Srgba;
use nannou::color::rgb_u32;
use nannou::image::flat::View;
use nannou::prelude::*;
use nannou::wgpu::{Backends, BufferView, DeviceDescriptor, Limits};
use nannou_egui::{Egui, egui};
use points_on_curve::points_on_bezier_curves;
use rand::rngs::StdRng;
use roughr::core::{Drawable, OpSetType, OpType, OptionsBuilder, FillStyle};
use roughr::generator::Generator;
use roughr::Point2D;

const DESIGN_WIDTH: i32 = 800;
const DESIGN_HEIGHT: i32 = 800;


pub struct Model {}


fn update(app: &App, _model: &mut Model, _update: Update) {}


fn view(app: &App, model: &Model, frame: nannou::Frame) {


    // Begin drawing
    let draw = app.draw();
    let win_rect = app.window_rect();

    // bg rect to see spacer
    draw.rect()
        .x(0.0)
        .y(200.0)
        .w(2000.0)
        .h(100.0)

        .color(gray(0.3));

    // draw grid helpers
    draw.background().color(gray(0.9));
    draw_grid(&draw, &win_rect, 20.0, 1.0);
    draw_crosshair(&draw, &win_rect);

    draw.to_frame(app, &frame).unwrap();
}


fn draw_grid(draw: &Draw, win: &Rect, step: f32, weight: f32) {

    let step_by = || (0..).map(|i| i as f32 * step);
    let r_iter = step_by().take_while(|&f| f < win.right());
    let l_iter = step_by().map(|f| -f).take_while(|&f| f > win.left());
    let x_iter = r_iter.chain(l_iter);
    for x in x_iter {
        draw.line()
            .color(gray(0.8))
            .weight(weight)
            .points(pt2(x, win.bottom()), pt2(x, win.top()));
    }
    let t_iter = step_by().take_while(|&f| f < win.top());
    let b_iter = step_by().map(|f| -f).take_while(|&f| f > win.bottom());
    let y_iter = t_iter.chain(b_iter);
    for y in y_iter {
        draw.line()
            .color(gray(0.8))
            .weight(weight)
            .points(pt2(win.left(), y), pt2(win.right(), y));
    }
}


fn draw_crosshair(draw: &Draw, win: &Rect) {

    // Crosshair.
    let crosshair_color = gray(0.5);
    let ends = [
        win.mid_top(),
        win.mid_right(),
        win.mid_bottom(),
        win.mid_left(),
    ];
    for &end in &ends {
        draw.arrow()
            .start_cap_round()
            .head_length(16.0)
            .head_width(8.0)
            .color(crosshair_color)
            .end(end);
    }

    // Crosshair text.
    let top = format!("{:.1}", win.top());
    let bottom = format!("{:.1}", win.bottom());
    let left = format!("{:.1}", win.left());
    let right = format!("{:.1}", win.right());
    let x_off = 30.0;
    let y_off = 20.0;
    draw.text("0.0")
        .x_y(15.0, 15.0)
        .color(crosshair_color)
        .font_size(14);
    draw.text(&top)
        .h(win.h())
        .font_size(14)
        .align_text_top()
        .color(crosshair_color)
        .x(x_off);
    draw.text(&bottom)
        .h(win.h())
        .font_size(14)
        .align_text_bottom()
        .color(crosshair_color)
        .x(x_off);
    draw.text(&left)
        .w(win.w())
        .font_size(14)
        .left_justify()
        .color(crosshair_color)
        .y(y_off);
    draw.text(&right)
        .w(win.w())
        .font_size(14)
        .right_justify()
        .color(crosshair_color)
        .y(y_off);
}

pub async fn run_app(model: Model) {
    // Since ModelFn is not a closure we need this workaround to pass the calculated model
    thread_local!(static MODEL: RefCell<Option<Model>> = Default::default());

    MODEL.with(|m| m.borrow_mut().replace(model));

    app::Builder::new_async(|app| {
        Box::new(async move {
            create_window(app).await;
            MODEL.with(|m| m.borrow_mut().take().unwrap())
        })
    })
        .backends(Backends::PRIMARY | Backends::GL)
        .update(update)
        .run_async()
        .await;
}

async fn create_window(app: &App) {
    let device_desc = DeviceDescriptor {
        limits: Limits {
            max_texture_dimension_2d: 8192,
            ..Limits::downlevel_webgl2_defaults()
        },
        ..Default::default()
    };

    // var app2 = app.;

    app.new_window()
        .size(
            DESIGN_WIDTH.to_u32().unwrap(),
            DESIGN_HEIGHT.to_u32().unwrap(),
        )
        .device_descriptor(device_desc)
        .title("nannou test")
        // .raw_event(raw_window_event)
        // .key_pressed(key_pressed)
        // .key_released(key_released)
        // .mouse_pressed(mouse_pressed)
        // .mouse_moved(mouse_moved)
        // .mouse_released(mouse_released)
        // .mouse_wheel(mouse_wheel)
        // .touch(touch)
        .view(view)
        .build_async()
        .await
        .unwrap();
}

// fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
//     if model.e_gui.is_some() {
//         let mut e = model.e_gui.as_mut().unwrap();
//         e.handle_raw_event(event);
//     }
// }

