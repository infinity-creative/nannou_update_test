use std::cell::RefCell;

use palette::Srgba;
use nannou::color::rgb_u32;

use nannou::image::flat::View;
use nannou::prelude::*;
use nannou::wgpu::{Backends, BufferView, DeviceDescriptor, Limits};
use nannou_egui::{Egui, egui};
use nannou_egui::egui::Shape;
use points_on_curve::points_on_bezier_curves;
use rand::rngs::StdRng;
use roughr::core::{Drawable, OpSetType, OpType, OptionsBuilder, FillStyle};
use roughr::generator::Generator;
use roughr::Point2D;

use crate::sketch_model::{HigResWorker, LayoutItem, Model, Shapes};
use crate::carbon;
use carbon::carbon_sketch_helpers;

const DESIGN_WIDTH: i32 = 900 / 2;
const DESIGN_HEIGHT: i32 = 1200 / 2;

pub struct Settings {
    show_grid: bool,
}


fn update(app: &App, model: &mut Model, update: Update) {
    if !model.is_setup {
        model.setup(
            app,
            DESIGN_HEIGHT,
            DESIGN_WIDTH,
        );

    }

    if model.e_gui.is_some() {
        let egui = &mut model.e_gui.as_mut().unwrap();
        let setttings = &mut model.settings;

        egui.set_elapsed_time(update.since_start);
        let ctx = egui.begin_frame();

        egui::Window::new("Settings").show(&ctx, |ui| {
            ui.label("Debug");
            ui.add(egui::Checkbox::new(&mut setttings.show_grid, "Show Grid"));
            ui.separator();

            ui.label("Layout");
            ui.add(
                egui::Slider::new(
                    &mut setttings.page_padding, 0..=100,
                ).text("Page Padding")
            );
            ui.add(
                egui::Slider::new(
                    &mut setttings.row_total, 0..=30,
                ).text("Rows Total")
            );
            ui.add(
                egui::Slider::new(
                    &mut setttings.col_total, 0..=30,
                ).text("Cols Total")
            );
        });
    }


    model.layout = Some(
        generate_layout(
            app.window_rect(),
            model.settings.page_padding,
            model.settings.row_total,
            model.settings.col_total,
        )
    );

}


fn generate_layout(
    win_rect: Rect,
    page_padding: i32,
    rows: i32,
    cols: i32,
) -> Vec<Vec<LayoutItem>> {
    let r = Rect::from_xy_wh(
        win_rect.xy(),
        win_rect.wh()
    );
    r.pad(page_padding.to_f32().unwrap());

    let mut layout = vec![];


    let row_h = r.h() / rows.to_f32().unwrap();
    let col_w = r.w() / cols.to_f32().unwrap();

    let mut y = r.bottom() + row_h / 2.0;

    for _ in 0..rows {
        let row_rect = Rect::from_xy_wh(
            pt2(0.0, 0.0),
            pt2(r.w(), row_h),
        );

        let mut x = row_rect.left() + col_w / 2.0;
        let mut row_items = vec![];

        for _ in 0..cols {
            row_items.push(
                LayoutItem {
                    shape: Shapes::Square,
                    dimensions: Rect::from_xy_wh(
                        pt2(x, y),
                        pt2(col_w, row_h),
                    ).pad(2.0),
                }
            );

            x = x + col_w;
        }

        layout.push(row_items);
        y = y + row_h;
    }

    layout
}


// fn generate_layout(
//     win_rect: Rect,
//     page_padding: i32,
//     rows: i32,
//     cols: i32,
// ) -> Vec<Vec<LayoutItem>> {
//     let r = Rect::from_xy_wh(win_rect.xy(), win_rect.wh());
//     r.pad(page_padding.to_f32().unwrap());
//
//     let col_w = r.w() / cols.to_f32().unwrap();
//     let row_h = r.h() / rows.to_f32().unwrap();
//
//     let mut x = r.x() - win_rect.w()/2.0;
//     let mut y = r.y() - win_rect.h()/2.0;
//
//     let mut layout = vec![];
//
//     for _ in 0..rows {
//         let mut layout_row = vec![];
//
//         let mut col_count = 0;
//
//         while col_count < cols  {
//
//             let s = get_rnd_shape();
//
//             let mut size = 1.0;
//             if s == Shapes::Square {
//                 let mut last_s = Shapes::Square;
//                 while last_s == Shapes::Square && col_count < cols {
//                     last_s = get_rnd_shape();
//                     if last_s == Shapes::Square {
//                         size += 1.0;
//                     }
//                     col_count += 1;
//                 }
//             } else {
//                 col_count += 1;
//             }
//
//             let current_rect = Rect::from_x_y_w_h(
//                 x,
//                 y,
//                 col_w * size,
//                 row_h,
//             );
//
//             current_rect.pad(5.0);
//
//             layout_row.push(LayoutItem {
//                 shape: s,
//                 dimensions: current_rect,
//             });
//
//             x = x + (col_w * size);
//         }
//
//         x = r.x() - win_rect.w()/2.0;
//         y = y + row_h;
//
//         layout.push(layout_row);
//     }
//
//     layout
// }

fn get_rnd_shape() -> Shapes {
    let s: Shapes;
    match random_range(0, 5) {
        0 => s = Shapes::Circle,
        1 => s = Shapes::Triangle,
        _ => s = Shapes::Square,
    }
    s
}

fn view(app: &App, model: &Model, frame: nannou::Frame) {
    if !model.is_setup {
        return; // not ready exit
    }

    // get the working drawing object
    // let worker = model.high_res_worker.as_ref();
    // let high: &HigResWorker = worker.unwrap();
    // let worker_draw = &high.draw;


    let win_rect = app.window_rect();

    let draw = app.draw();
    draw.background().color(WHITE);

    let layouts = model.layout.as_ref().unwrap();
    for row in layouts {
        for item in row {
            draw.rect()
                .color(gray(random_f32()))
                .xy(item.dimensions.xy())
                .wh(item.dimensions.wh());
        }
    }


    // draw.rect()
    //     .color(gray(0.5))
    //     .x_y(0.0, 0.0)
    //     .w_h(300.0, 300.0);

    // bg rect to see spacer
    // draw.rect()
    //     .x(0.0)
    //     .y(200.0)
    //     .w(2000.0)
    //     .h(100.0)
    //
    //     .color(gray(0.3));

    // draw grid helpers
    // draw.background().color(gray(0.9));


    if model.settings.show_grid {
        carbon_sketch_helpers::draw_grid(&draw, &win_rect, 20.0, 1.0);
        carbon_sketch_helpers::draw_crosshair(&draw, &win_rect);
    }

    draw.to_frame(app, &frame).unwrap();

    if model.e_gui.is_some() {
        model.e_gui.as_ref().unwrap()
            .draw_to_frame(&frame).unwrap();
    }
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
        .title("nn_001")
        .raw_event(raw_window_event)
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

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    if model.e_gui.is_some() {
        let mut e = model.e_gui.as_mut().unwrap();
        e.handle_raw_event(event);
    }
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

