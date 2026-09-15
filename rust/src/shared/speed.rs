#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Speed(pub f32);

impl Speed {
    pub const SNEAK: Speed = Speed(1.2);
    pub const WALK: Speed = Speed(1.5);
    pub const RUN: Speed = Speed(4.0);
}
