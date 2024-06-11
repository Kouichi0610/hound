
pub enum ControllerResult {
    OK(ControllerData),
    Quit,
}
pub trait ControllerUpdator {
    fn update(&mut self) -> ControllerResult;
}

#[derive(Clone, Copy)]
pub struct ControllerData {
    pub left_stick_x: i8,
    pub left_stick_y: i8,
    pub right_stick_x: i8,
    pub right_stick_y: i8,
    pub trigger_left: u8,
    pub trigger_right: u8,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
    pub x: bool,
    pub y: bool,
    pub l: bool,
    pub r: bool,
}

pub fn default_data() -> ControllerData {
    ControllerData {
        left_stick_x: 0,
        left_stick_y: 0,
        right_stick_x: 0,
        right_stick_y: 0,
        trigger_left: 0,
        trigger_right: 0,
        up: false,
        down: false,
        left: false,
        right: false,
        a: false,
        b: false,
        x: false,
        y: false,
        l: false,
        r: false,
    }
}

impl ControllerData {
    pub fn to_string(&self) -> String {
        format!("left X:{} Y:{} right X:{} Y:{}", self.left_stick_x, self.left_stick_y, self.right_stick_x, self.right_stick_y)
    }
}