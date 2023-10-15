use std::cell::RefCell;
use nannou::noise::utils::Color;


use nannou::prelude::*;
use nannou::wgpu::{Backends, BufferView, DeviceDescriptor, Limits};
use nannou_egui::{Egui, egui};
use nannou_egui::egui::Shape;
use palette::IntoColor;
use points_on_curve::points_on_bezier_curves;
use rand::rngs::StdRng;
use roughr::core::{Drawable, OpSetType, OpType, OptionsBuilder, FillStyle};
use roughr::generator::Generator;
use roughr::Srgba;
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

        model.layout = Some(
            generate_layout(
                app.window_rect(),
                model.settings.page_padding,
                model.settings.row_total,
                model.settings.col_total,
                model.settings.gap,
            )
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

            if ui.add(
                egui::Slider::new(
                    &mut setttings.row_total, 0..=30,
                ).text("Rows Total")
            ).changed() {
                model.is_setup = false;
            }


            if ui.add(
                egui::Slider::new(
                    &mut setttings.col_total, 0..=30,
                ).text("Cols Total")
            ).changed() {
                model.is_setup = false;
            }

            if ui.add(
                egui::Slider::new(
                    &mut setttings.gap, 0..=30,
                ).text("Gap")
            ).changed() {
                model.is_setup = false;
            }
        });
    }
}


fn generate_layout(
    win_rect: Rect,
    page_padding: i32,
    rows: i32,
    cols: i32,
    gap: i32,
) -> Vec<Vec<LayoutItem>> {
    let r = Rect::from_xy_wh(
        win_rect.xy(),
        win_rect.wh(),
    ).pad(page_padding.to_f32().unwrap());


    let mut layout = vec![];


    let row_h = r.h() / rows.to_f32().unwrap();
    let col_w = r.w() / cols.to_f32().unwrap();

    let mut y = r.bottom() + row_h / 2.0;

    // generate items ---------------------------------------------------
    for _ in 0..rows {
        let row_rect = Rect::from_xy_wh(
            pt2(0.0, 0.0),
            pt2(r.w(), row_h),
        );

        let mut x = row_rect.left() + col_w / 2.0;
        let mut row_items = vec![];

        // gen shapes we want
        for _ in 0..cols {
            row_items.push(
                LayoutItem {
                    shape: get_rnd_shape(),
                    dimensions: Rect::from_xy_wh(
                        pt2(x, y),
                        pt2(col_w, row_h),
                    ),
                }
            );

            x = x + col_w;
        }

        let mut new_row_items = vec![];
        let mut new_layout_item: Option<LayoutItem> = None;
        //.pad(gap.to_f32().unwrap())

        // join the squares together ------------------------------------------------
        for x_item in row_items {
            if x_item.shape == Shapes::Square {
                if new_layout_item.is_none() {
                    new_layout_item = Some(
                        LayoutItem {
                            shape: x_item.shape,
                            dimensions: x_item.dimensions,
                        }
                    );
                } else {
                    // we have one already streatch out
                    let t_item = new_layout_item.unwrap();
                    let w1 = t_item.dimensions.w();
                    let r = t_item.dimensions.right();
                    let new_r = t_item.dimensions
                        .stretch_to_point(
                            [
                                x_item.dimensions.right(),
                                x_item.dimensions.top(),
                            ],
                        );

                    let df = LayoutItem {
                        shape: x_item.shape,
                        dimensions: Rect::from_xy_wh(new_r.xy(), new_r.wh()),
                    };

                    new_layout_item = Some(df);
                }
            } else {
                // where we just working with a square

                if new_layout_item.is_some() {
                    new_row_items.push(
                        new_layout_item.unwrap().clone()
                    );
                }
                new_layout_item = None;

                new_layout_item = Some(
                    LayoutItem {
                        shape: x_item.shape,
                        dimensions: x_item.dimensions,
                    }
                );

                new_row_items.push(
                    new_layout_item.unwrap().clone()
                );
                new_layout_item = None;
            }
        }
        if (new_layout_item.is_some()) {
            new_row_items.push(
                new_layout_item.unwrap().clone()
            );
        }


        // add padding
        let mut padded_row = vec![];
        for non_padded in new_row_items {
            let d = non_padded.dimensions.pad(gap.to_f32().unwrap());
            padded_row.push(
                LayoutItem {
                    shape: non_padded.shape,
                    dimensions: d,
                }
            )
        }

        layout.push(padded_row);
        y = y + row_h;
    }

    layout
}


fn get_rnd_shape() -> Shapes {
    let s: Shapes;
    match random_range(0, 10) {
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

    if model.render_complete {
        return;
    }

    // get the working drawing object
    // let worker = model.high_res_worker.as_ref();
    // let high: &HigResWorker = worker.unwrap();
    // let worker_draw = &high.draw;


    let win_rect = app.window_rect();

    let draw = app.draw();
    draw.background().color(WHITE);

    let mut c = 0;
    let layouts = model.layout.as_ref().unwrap();
    for row in layouts {
        for item in row {
            // draw.rect()
            //     .color(gray(0.8))
            //     .xy(item.dimensions.xy())
            //     .wh(item.dimensions.wh());

            let p = model.raw_palette.as_ref().unwrap();
            let r = random_range(0, 5);
            let fill_color = p[r].clone();

            let parse_color = fill_color.parse::<csscolorparser::Color>().unwrap();

            let sc = Srgba::from_components((
                parse_color.r,
                parse_color.g,
                parse_color.b,
                parse_color.a)
            );

            let mut fill_style = FillStyle::ZigZag;


            match random_range(0,5) {

                0 => fill_style = FillStyle::Dashed,
                1 => fill_style = FillStyle::Dots,
                2 => fill_style = FillStyle::Hachure,
                3 => fill_style = FillStyle::CrossHatch,
                4 => fill_style = FillStyle::ZigZagLine,
                _ => fill_style = FillStyle::ZigZag,
            }

            let options = OptionsBuilder::default()
                .seed(c * 1000)
                .fill(sc.into_format())
                .fill_style(fill_style.clone())

                // .stroke()
                // .curve_tightness(settings.curve_tightness)
                // .curve_fitting(settings.curve_fitting)
                // .bowing(settings.bowing)
                // .max_randomness_offset(settings.max_ran)
                // .roughness(settings.roughness)

                .build()
                .unwrap();

            let g = Generator::default();

            let draw_item: Drawable<f32>;

            match item.shape {
                Shapes::Square => {
                    draw_item = g.rectangle::<f32>(
                        item.dimensions.x() - (item.dimensions.w() / 2.0),
                        item.dimensions.y() - (item.dimensions.h() / 2.0),
                        item.dimensions.w(),
                        item.dimensions.h(),
                        &Some(options.clone()),
                    );
                }

                Shapes::Circle => {
                    draw_item = g.ellipse::<f32>(
                        item.dimensions.x(),
                        item.dimensions.y(),
                        item.dimensions.w(),
                        item.dimensions.h(),
                        &Some(options.clone()),
                    );
                }

                Shapes::Triangle => {
                    let top = Point2D::new(
                        item.dimensions.x(),
                        item.dimensions.y() + (item.dimensions.h() / 2.0),
                    );

                    let left = Point2D::new(
                        item.dimensions.x() - (item.dimensions.w() / 2.0),
                        item.dimensions.y() - (item.dimensions.h() / 2.0),
                    );


                    let right = Point2D::new(
                        item.dimensions.x() + (item.dimensions.w() / 2.0),
                        item.dimensions.y() - (item.dimensions.h() / 2.0),
                    );

                    draw_item = g.polygon::<f32>(
                        &[top, left, right],
                        &Some(options.clone()),
                    );
                }

                _ => {
                    draw_item = g.rectangle::<f32>(
                        item.dimensions.x() - (item.dimensions.w() / 2.0),
                        item.dimensions.y() - (item.dimensions.h() / 2.0),
                        item.dimensions.w(),
                        item.dimensions.h(),
                        &Some(options.clone()),
                    );
                }
            }

            sketch_lines(&draw, &draw_item);


            c = c + 1;
        }
    }

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


fn sketch_lines(draw: &Draw, lp: &Drawable<f32>) {
    for set in lp.sets.iter() {
        if set.op_set_type == OpSetType::Path {
            let working_set = set.clone();
            // println!("{:?}", working_set);

            let mut points = Vec::new();

            // let mut start_pont = pt2(0.0, 0.0);

            for item in working_set.ops {
                match item.op {
                    OpType::Move => {

                        // if we are about to move - draw the points
                        if !points.is_empty() {
                            draw.polyline()
                                .color(gray(0.7))
                                .points(points.clone());
                            points.clear();
                        }

                        points.push(pt2(
                            item.data[0].to_f32().unwrap(),
                            item.data[1].to_f32().unwrap(),
                        ));
                    }
                    OpType::LineTo => {
                        points.push(pt2(
                            item.data[0].to_f32().unwrap(),
                            item.data[1].to_f32().unwrap(),
                        ));
                    }
                    OpType::BCurveTo => {
                        let mut curve_points = Vec::new();
                        let last_point = points.last().clone().unwrap();
                        curve_points.push(Point2D::new(
                            last_point.x,
                            last_point.y,
                        ));

                        curve_points.push(Point2D::new(
                            item.data[0].to_f32().unwrap(),
                            item.data[1].to_f32().unwrap(),
                        ));

                        curve_points.push(Point2D::new(
                            item.data[2].to_f32().unwrap(),
                            item.data[3].to_f32().unwrap(),
                        ));

                        curve_points.push(Point2D::new(
                            item.data[4].to_f32().unwrap(),
                            item.data[5].to_f32().unwrap(),
                        ));

                        let result_015 = points_on_bezier_curves(&curve_points, 0.2, Some(0.01));

                        for p in result_015 {
                            points.push(pt2(p.x, p.y))
                        }
                    }
                }
            }

            draw.polyline()
                .color(gray(0.7))
                .points(points.clone());
            points.clear();
        }

        if set.op_set_type == OpSetType::FillSketch {
            let working_set = set.clone();
            // println!("{:?}", working_set);

            let sb_fill = lp.options.fill.unwrap();


            let mut points = Vec::new();

            // let mut start_pont = pt2(0.0, 0.0);

            for item in working_set.ops {
                match item.op {
                    OpType::Move => {

                        // if we are about to move - draw the points
                        if !points.is_empty() {
                            draw.polyline()
                                .weight(2.0)
                                .color(
                                    srgba(sb_fill.red, sb_fill.green, sb_fill.blue, sb_fill.alpha)
                                )
                                .points(points.clone());
                            points.clear();
                        }

                        points.push(pt2(
                            item.data[0].to_f32().unwrap(),
                            item.data[1].to_f32().unwrap(),
                        ));
                    }
                    OpType::LineTo => {
                        points.push(pt2(
                            item.data[0].to_f32().unwrap(),
                            item.data[1].to_f32().unwrap(),
                        ));
                    }
                    OpType::BCurveTo => {
                        let mut curve_points = Vec::new();
                        let last_point = points.last().clone().unwrap();
                        curve_points.push(Point2D::new(
                            last_point.x,
                            last_point.y,
                        ));

                        curve_points.push(Point2D::new(
                            item.data[0].to_f32().unwrap(),
                            item.data[1].to_f32().unwrap(),
                        ));

                        curve_points.push(Point2D::new(
                            item.data[2].to_f32().unwrap(),
                            item.data[3].to_f32().unwrap(),
                        ));

                        curve_points.push(Point2D::new(
                            item.data[4].to_f32().unwrap(),
                            item.data[5].to_f32().unwrap(),
                        ));

                        let result_015 = points_on_bezier_curves(&curve_points, 0.2, Some(0.01));

                        for p in result_015 {
                            points.push(pt2(p.x, p.y))
                        }
                    }
                }
            }


            draw.polyline()
                .weight(2.0)
                .color(
                    srgba(sb_fill.red, sb_fill.green, sb_fill.blue, sb_fill.alpha)
                )
                .points(points.clone());
            points.clear();
        }

        if set.op_set_type == OpSetType::FillPath {
            let working_set = set.clone();
            println!("{:?}", working_set);

            // let mut points = Vec::new();
        }
    }
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

