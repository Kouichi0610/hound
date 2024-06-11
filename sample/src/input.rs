/*
    地上
    空中
    壁はりつき


*/

struct SampleMover {
}

pub trait MoveActor {
    fn walk(&self, f: f32);
}
pub trait ActionActor {
    fn action(&self);
}

impl MoveActor for SampleMover {
    fn walk(&self, f: f32) {
        println!("Move:{}", f);
    }
}
impl ActionActor for SampleMover {
    fn action(&self) {
        println!("Action");
    }
}

pub fn create() -> Box<dyn MoveActor> {
    Box::new(SampleMover{})
}
