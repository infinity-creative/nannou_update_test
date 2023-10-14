use nannou::{App, Frame, wgpu};
use nannou::geom::Rect;
use nannou::prelude::ToPrimitive;
use nannou_egui::Egui;

#[derive(PartialEq, Clone)]
pub enum Shapes {
    Square,
    Circle,
    Triangle,
    Unset,
}

pub struct Settings {
    pub show_grid: bool,

    pub page_padding: i32,
    pub col_total: i32,
    pub row_total: i32,
    pub gap: i32,
}

#[derive(Clone)]
pub struct LayoutItem {
    pub shape: Shapes,
    pub dimensions: Rect,
}

pub struct HigResWorker {
    //WORKER_SPACE -----------------------------------------
    // The texture that we will draw to.
    texture: wgpu::Texture,
    // Create a `Draw` instance for drawing to our texture.
    pub draw: nannou::Draw,
    // The type used to render the `Draw` vertices to our texture.
    renderer: nannou::draw::Renderer,
    // The type used to capture the texture.
    texture_capturer: wgpu::TextureCapturer,
    // The type used to resize our texture to the window texture.
    texture_reshaper: wgpu::TextureReshaper,

}

pub struct Model {
    pub is_setup: bool,
    pub settings: Settings,
    pub e_gui: Option<Egui>,
    pub high_res_worker: Option<HigResWorker>,
    pub layout: Option<Vec<Vec<LayoutItem>>>,
}

impl Model {
    pub fn new() -> Model {
        Model {
            is_setup: false,
            settings: Settings {
                show_grid: true,

                page_padding: 30,
                col_total: 14,
                row_total: 15,
                gap:1,
            },

            // will be setup on first update call
            e_gui: None,
            high_res_worker: None,
            layout: None,
        }
    }

    pub fn setup(
        &mut self,
        app: &App,
        worker_w: i32, worker_h: i32,
    ) {

        // self.build_woker(app, worker_w, worker_h);

        // EGUI --------------------------------------------
        if self.e_gui.is_none() {
            let (w_px, h_px) = app.main_window().inner_size_pixels();
            let egui = Egui::new(
                app.main_window().device(),
                nannou::Frame::TEXTURE_FORMAT,
                app.main_window().msaa_samples(),
                app.main_window().scale_factor(),
                [w_px, h_px],
            );
            self.e_gui = Some(egui);
        }

        // mark the model as ready to go
        self.is_setup = true;
    }

    /*
    fn build_woker(
        &mut self,
        app: &App,
        worker_w: i32, worker_h: i32,
    ) -> HigResWorker {
        let texture_size = [
            worker_w.to_u32().unwrap(),
            worker_h.to_u32().unwrap(),
        ];

        let window = app.window(app.window_id()).unwrap();

        // Retrieve the wgpu device.
        let device = window.device();

        // Create our custom texture.
        let sample_count = window.msaa_samples();
        let texture = wgpu::TextureBuilder::new()
            .size(texture_size)
            // Our texture will be used as the RENDER_ATTACHMENT for our `Draw` render pass.
            // It will also be SAMPLED by the `TextureCapturer` and `TextureResizer`.
            .usage(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING)
            // Use nannou's default multisampling sample count.
            .sample_count(sample_count)
            // Use a spacious 16-bit linear sRGBA format suitable for high quality drawing.
            .format(wgpu::TextureFormat::Rgba16Float)
            // Build it!
            .build(device);

        let draw = nannou::Draw::new();
        let descriptor = texture.descriptor();
        let renderer =
            nannou::draw::RendererBuilder::new().build_from_texture_descriptor(device, descriptor);

        // Create the texture capturer.
        let texture_capturer = wgpu::TextureCapturer::default();

        // Create the texture reshaper.
        let texture_view = texture.view().build();
        let texture_sample_type = texture.sample_type();
        let dst_format = Frame::TEXTURE_FORMAT;
        let texture_reshaper = wgpu::TextureReshaper::new(
            device,
            &texture_view,
            sample_count,
            texture_sample_type,
            sample_count,
            dst_format,
        );

        HigResWorker {
            texture,
            draw,
            renderer,
            texture_capturer,
            texture_reshaper,
        }
    }

     */
}
