use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use game_canvas;

pub struct RendererImpl {
    canvas: Canvas<Window>,
}

impl game_canvas::Renderer for RendererImpl {
    fn clear_screen(&mut self) {
        self.canvas.clear();
    }
    fn present(&mut self) {
        self.canvas.present();
    }
    fn fill_rect(&mut self, x: i32, y: i32, w: u32, h: u32) {
        self.canvas.fill_rect(Rect::new(x, y, w, h)).unwrap();
    }
    fn color_rgb(&mut self, r: u8, g: u8, b: u8) {
        self.canvas.set_draw_color(Color::RGB(r, g, b));
    }
    fn color_rgba(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.canvas.set_draw_color(Color::RGBA(r, g, b, a));
    }

}

pub fn create_canvas(sdl_context: &Sdl, title: &str, width: u32, height: u32) -> Box<dyn game_canvas::Renderer> {
    let video_subsystem = sdl_context.video().unwrap();

    let window = match video_subsystem
        .window(title, width, height)
        .build() {
            Ok(window) => window,
            Err(e) => {
                panic!("failed build window {:?}", e);
            },
        };

    let canvas = window.into_canvas()
        .build()
        .unwrap();

    Box::new(RendererImpl { canvas })
}
