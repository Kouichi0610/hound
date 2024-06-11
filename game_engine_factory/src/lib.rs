use game_canvas::Renderer;
use game_controller::ControllerUpdator;
use std::fmt::Error;

pub trait EngineFactory {
    fn create_renderer(&mut self, title: &str, screen_width: u32, screen_height: u32) -> Result<Box<dyn Renderer>, Error>;
    fn create_controller_updator(&mut self) -> Result<Box<dyn ControllerUpdator>, Error>;
}
