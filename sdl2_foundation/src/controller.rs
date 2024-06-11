use sdl2::event::Event;
use sdl2::mouse::MouseWheelDirection;
use sdl2::controller::Axis;
use sdl2::sys::{SDL_Init, SDL_INIT_JOYSTICK, SDL_INIT_GAMECONTROLLER, SDL_INIT_HAPTIC, SDL_GameControllerOpen};
use sdl2::Sdl;
use sdl2::controller::Button;

extern crate game_controller;
use game_controller::{ControllerResult, ControllerUpdator, ControllerData};
use sdl2::EventPump;

struct UpdatorImpl {
    event_pump: EventPump,
    c: ControllerData,
}

fn to_trigger_value(i: i16) -> u8 {
    (i as f64 / 32767.0 * 100.0) as u8
}
fn to_stick_value(i: i16) -> i8 {
    if i < 0 {
        return (i as f64 / 32768.0 * 100.0) as i8;
    }
    (i as f64 / 32767.0 * 100.0) as i8
}

impl ControllerUpdator for UpdatorImpl {
    fn update(&mut self) -> ControllerResult {

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return ControllerResult::Quit,
                Event::KeyDown { timestamp: _, window_id: _, keycode, scancode, keymod, repeat } => {
                    let key = match keycode {
                        Some(key) => key,
                        _ => continue,
                    };
                    match key {
                        sdl2::keyboard::Keycode::Escape => return ControllerResult::Quit,
                        _ => continue,
                    }
                },
                Event::ControllerButtonDown { timestamp: _, which: _, button } => {
                    match button {
                        Button::A => self.c.a = true,
                        Button::B => self.c.b = true,
                        Button::X => self.c.x = true,
                        Button::Y => self.c.y = true,
                        Button::DPadUp => self.c.up = true,
                        Button::DPadDown => self.c.down = true,
                        Button::DPadLeft => self.c.left = true,
                        Button::DPadRight => self.c.right = true,
                        Button::LeftShoulder => self.c.l = true,
                        Button::RightShoulder => self.c.r = true,
                        _ => {}
                    }
                },
                Event::ControllerButtonUp { timestamp: _, which: _, button } => {
                    match button {
                        Button::A => self.c.a = false,
                        Button::B => self.c.b = false,
                        Button::X => self.c.x = false,
                        Button::Y => self.c.y = false,
                        Button::DPadUp => self.c.up = false,
                        Button::DPadDown => self.c.down = false,
                        Button::DPadLeft => self.c.left = false,
                        Button::DPadRight => self.c.right = false,
                        Button::LeftShoulder => self.c.l = false,
                        Button::RightShoulder => self.c.r = false,
                        _ => {}
                    }
                },
                Event::ControllerAxisMotion { timestamp: _, which: _, axis, value } => {
                    match axis {
                        Axis::LeftX => self.c.left_stick_x = to_stick_value(value),
                        Axis::LeftY => self.c.left_stick_y = to_stick_value(value),
                        Axis::RightX => self.c.right_stick_x = to_stick_value(value),
                        Axis::RightY => self.c.right_stick_y = to_stick_value(value),
                        Axis::TriggerLeft => self.c.trigger_left = to_trigger_value(value),
                        Axis::TriggerRight => self.c.trigger_right = to_trigger_value(value),
                    }
                },
                Event::JoyDeviceAdded { timestamp: _, which: _ } => {
                    unsafe {
                        SDL_GameControllerOpen(0);
                    }
                }
                _ => {}
            }
        }
        ControllerResult::OK(self.c.clone())
    }
}

pub fn create_controller_updator(sdl_context: &Sdl) -> Box<dyn ControllerUpdator> {
    unsafe {
        SDL_Init(SDL_INIT_JOYSTICK | SDL_INIT_HAPTIC | SDL_INIT_GAMECONTROLLER);
    };
    let event_pump = sdl_context.event_pump().unwrap();
    Box::new(UpdatorImpl{ event_pump, c: game_controller::default_data() })
}