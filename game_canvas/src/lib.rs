
pub trait Renderer {
    fn clear_screen(&mut self);
    fn present(&mut self);
    fn fill_rect(&mut self, x: i32, y: i32, w: u32, h: u32);
    fn color_rgb(&mut self, r: u8, g: u8, b: u8);
    fn color_rgba(&mut self, r: u8, g: u8, b: u8, a: u8);
}

