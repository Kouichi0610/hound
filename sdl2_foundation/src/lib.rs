// sdl2_foundation
use std::{fmt::Error, time::Duration};
use game_canvas::Renderer;
use game_controller::{self, ControllerUpdator, ControllerResult, ControllerData};
use sdl2::Sdl;
use renderer::create_canvas;
use controller::create_controller_updator;
mod renderer;
mod controller;
use game_engine_factory::EngineFactory;


struct FactoryImpl {
    sdl_context: Sdl,
}
impl EngineFactory for FactoryImpl {
    fn create_renderer(&mut self, title: &str, screen_width: u32, screen_height: u32) -> Result<Box<dyn Renderer>, Error> {
        Result::Ok(create_canvas(&self.sdl_context, title, screen_width, screen_height))
    }
    fn create_controller_updator(&mut self) -> Result<Box<dyn ControllerUpdator>, Error> {
        Result::Ok(create_controller_updator(&self.sdl_context))
    }
}

pub fn create_factory() -> Box<dyn EngineFactory> {
    let sdl_context = sdl2::init().unwrap();
    Box::new(FactoryImpl{ sdl_context })
}

pub fn sample() {
    let title = "SAMPLE";
    let width = 640;
    let height = 480;
    let mut factory = create_factory();
    let mut c = match factory.create_renderer(title, width, height) {
        Ok(c) => c,
        Err(e) => {
            panic!("failed {:?}", e);
        },
    };
    let mut controller = factory.create_controller_updator().unwrap();


    c.clear_screen();
    c.present();

    loop {
        let result = match controller.update() {
            ControllerResult::OK(result) =>  result,
            ControllerResult::Quit => break,
        };

        c.color_rgb(255, 255, 255);
        c.clear_screen();
        c.color_rgb(255, 255, 0);
        c.fill_rect(20, 20, 200, 300);

        // left stick
        c.color_rgb(255, 0, 0);
        let x = (result.left_stick_x * 1) as i32;
        let y = (result.left_stick_y * 1) as i32;
        c.fill_rect(200 + x, 200 + y, 20, 20);

        // right stick
        c.color_rgb(0, 0, 255);
        let x = (result.right_stick_x * 1) as i32;
        let y = (result.right_stick_y * 1) as i32;
        c.fill_rect(400 + x, 200 + y, 20, 20);

        // lr trigger
        c.color_rgb(255, 0, 255);
        c.fill_rect(200 - 150, 200 + 0, 20, 0 + result.trigger_left as u32);
        c.fill_rect(400 + 150, 200 + 0, 20, 0 + result.trigger_right as u32);

        c.color_rgb(0, 255, 255);
        if result.left {
            c.fill_rect(200 - 50, 200 + 0, 20, 20);
        }
        if result.right {
            c.fill_rect(200 + 50, 200 + 0, 20, 20);
        }
        if result.up {
            c.fill_rect(200 + 0, 200 - 50, 20, 20);
        }
        if result.down {
            c.fill_rect(200 + 0, 200 + 50, 20, 20);
        }
        if result.x {
            c.fill_rect(400 - 50, 200 + 0, 20, 20);
        }
        if result.b {
            c.fill_rect(400 + 50, 200 + 0, 20, 20);
        }
        if result.y {
            c.fill_rect(400 + 0, 200 - 50, 20, 20);
        }
        if result.a {
            c.fill_rect(400 + 0, 200 + 50, 20, 20);
        }
        if result.l {
            c.fill_rect(200 - 100, 200 - 100, 20, 20);
        }
        if result.r {
            c.fill_rect(400 + 100, 200 - 100, 20, 20);
        }

        c.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    println!("sample quit.");
}
