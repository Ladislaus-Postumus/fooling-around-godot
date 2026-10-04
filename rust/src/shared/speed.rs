use std::f32::consts::PI;

use godot::prelude::*;

#[derive(GodotConvert, Var, Export, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[godot(transparent)]
pub struct Speed(pub f32);

impl Speed {
    pub const TURN: Self = Self(PI);

    pub const SNEAK: Self = Self(1.5);
    pub const WALK: Self = Self(2.0);
    pub const RUN: Self = Self(5.0);
}
