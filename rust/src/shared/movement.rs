use crate::shared::Speed;
use godot::classes::CharacterBody3D;
use godot::prelude::*;

pub struct MovementIntent {
    pub speed: Speed,
    pub direction: Vector3,
}

pub struct MovementCharacter {
    pub intent: MovementIntent,
    pub current_velocity: Vector3,
}

impl MovementCharacter {
    pub fn apply_acceleration(&mut self, delta: f32) -> &mut Self {
        let target = self.intent.direction * self.intent.speed.0;
        self.current_velocity = self
            .current_velocity
            .move_toward(target, self.intent.speed.0);
        self
    }

    pub fn apply_gravity(&mut self, gravity: Vector3, delta: f32) -> &mut Self {
        self.current_velocity += gravity * delta;
        self
    }

    pub fn apply_to(&self, base: &mut CharacterBody3D) {
        base.set_velocity(self.current_velocity);
    }

    pub fn from_input(
        speed: Speed,
        direction: Vector3,
        gravity: Vector3,
        delta: f32,
        base: &CharacterBody3D,
    ) -> Self {
        let mut movement = Self {
            intent: MovementIntent { speed, direction },
            current_velocity: base.get_velocity(),
        };
        movement.apply_acceleration(delta);
        movement.apply_gravity(gravity, delta);
        movement
    }
}
