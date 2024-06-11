use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::controller::Axis;
use sdl2::mouse::MouseWheelDirection;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::sys::{SDL_GameControllerOpen, SDL_Haptic, SDL_HapticOpen, SDL_HapticRumbleInit, SDL_HapticRumblePlay, SDL_Init, SDL_JoystickOpen, SDL_NumJoysticks, _SDL_Haptic, _SDL_Joystick, SDL_INIT_GAMECONTROLLER, SDL_INIT_HAPTIC, SDL_INIT_JOYSTICK};
use std::time::Duration;
mod input;

// TODO:move event_listener
//               + SDL2
//               + SDL3

/*
    TODO:入力の抽象化

    window builder

    input looper

*/
pub fn sample() {
    let sdl_context = sdl2::init().unwrap();
    let video_sybsystem = sdl_context.video().unwrap();

    let window = video_sybsystem
        .window("SDL", 1024, 768)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    sdl2::log::log("sample");

    let mov = input::create();
    mov.walk(1.0);


    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(255,255,255));
    canvas.clear();
    canvas.present();


    let haptic: *mut _SDL_Haptic;
    unsafe {
        // haptic 振動？
        SDL_Init(SDL_INIT_JOYSTICK | SDL_INIT_HAPTIC | SDL_INIT_GAMECONTROLLER);

        haptic = SDL_HapticOpen(0);
        SDL_HapticRumbleInit(haptic);
    }

    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut tx = 0;
    let mut ty = 0;


    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Window { timestamp, window_id, win_event } => {
                    match win_event {
                        WindowEvent::Resized(x, y) => {
                            println!("Resized:{}x{}", x, y);

                        },
                        _ => {},
                    }

                },
                Event::Quit { .. } => {
                    break 'running;
                },
                // keymod shift押しながら、alt押しながらのフラグ
                Event::KeyUp { timestamp, window_id, keycode, scancode, keymod, repeat } => {
                    let key = match keycode {
                        Some(key) => key,
                        _ => break,
                    };
                    let s = scancode.unwrap();
                    println!("KeyUp:{} Scan:{} Keymod:{} Repeat:{}", key.name(), s.name(), keymod, repeat);
                },
                Event::KeyDown { timestamp, window_id, keycode, scancode, keymod, repeat } => 
                {
                    let key = match keycode {
                        Some(key) => key,
                        _ => break,
                    };
                    let s = scancode.unwrap();
                    println!("KeyDown:{} Scan:{} Keymod:{} Repeat:{}", key.name(), s.name(), keymod, repeat);
                    if keycode == Some(Keycode::Escape) {
                        break 'running;
                    }
                },
                Event::ControllerAxisMotion { timestamp, which, axis, value } => {
                    // -32768 <->32767
                    match axis {
                        Axis::LeftX | Axis::RightX => x = value as f32 / 32768.0,
                        Axis::LeftY | Axis::RightY => y = value as f32 / 32768.0,
                        Axis::TriggerLeft => tx = value,
                        Axis::TriggerRight => ty = value,
                    }
                    //println!("ControllerAxis Time:{} axis:{} value:{}", timestamp, axis.string(), value);
                },
                Event::ControllerButtonDown { timestamp, which, button } => {
                    println!("BtnDown:{}", button.string());

                    unsafe {
                        SDL_HapticRumblePlay(haptic, 1.0, 2000);
                    }
                },
                Event::ControllerButtonUp { timestamp, which, button } => {
                    println!("BtnUp:{}", button.string());
                },
                Event::JoyAxisMotion { timestamp, which, axis_idx, value } => {

                },
                Event::JoyDeviceAdded { timestamp, which } => {
                    println!("Joypad Added.");
                    unsafe {
                        SDL_GameControllerOpen(0);
                    }
                },
                Event::ControllerDeviceRemapped { timestamp, which } => {
                    println!("device remapped.");
                },
                Event::ControllerDeviceRemoved { timestamp, which } => {
                    println!("device removed.");
                },
                Event::MouseButtonDown { timestamp, window_id, which, mouse_btn, clicks, x, y } => {
                    println!("MDown:{} window:{} which:{} btn:{} click:{} x:{} y:{}", timestamp, window_id, which, 0, clicks, x, y);
                },
                Event::MouseButtonUp { timestamp, window_id, which, mouse_btn, clicks, x, y } => {
                    println!("Mup:{} window:{} which:{} btn:{} click:{} x:{} y:{}", timestamp, window_id, which, 0, clicks, x, y);
                },
                Event::MouseWheel { timestamp, window_id, which, x, y, direction, precise_x, precise_y } => {
                    let dir = match direction {
                        MouseWheelDirection::Flipped => "flip",
                        MouseWheelDirection::Normal => "normal",
                        _ => "unknown",
                    };
                    println!("x:{} y:{} dir:{} preX:{} preY:{}", x, y, dir, precise_x, precise_y);
                },
                Event::DropFile { timestamp, window_id, filename } => {
                    println!("DropFile: Window:{} filename:{}", window_id, filename);

                },
                Event::AppTerminating { timestamp } => {
                    println!("terminating:{}", timestamp);

                },
                Event::JoyDeviceRemoved { timestamp, which } => {
                    println!("JoyRemoved:{} {}", timestamp, which);
                },
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(255,255,255));
        canvas.clear();

        canvas.set_draw_color(Color::RGBA(200,0,0,200));
        let dx = 200 + (100.0 * x) as i32;
        let dy = 200 + (100.0 * y) as i32;
        let s = canvas.fill_rect(Rect::new(dx, dy, 10, 10));
        match s {
            Ok(..) => {},
            Err(e) => println!("Error:{}", e),
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    println!("Quit.");
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
