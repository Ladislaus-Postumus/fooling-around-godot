use std::f32::consts::PI;

use godot::prelude::*;

#[derive(GodotConvert, Var, Export, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[godot(transparent)]
pub struct Speed(pub f32);

impl Speed {
    pub const TURN: Speed = Speed(PI);

    pub const SNEAK: Speed = Speed(1.2);
    pub const WALK: Speed = Speed(1.5);
    pub const RUN: Speed = Speed(4.0);
}
