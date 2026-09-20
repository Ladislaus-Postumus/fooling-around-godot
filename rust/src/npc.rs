use std::f32::consts::PI;

use crate::shared::{MovementCharacter, Speed};
use godot::classes::{CharacterBody3D, ICharacterBody3D};
use godot::prelude::*;
use rand::RngExt;

#[derive(GodotConvert, Var, Export, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[godot(transparent)]
struct DurationPaused(f32);

#[derive(Debug)]
enum NpcState {
    Moving(Vector3),
    Paused(DurationPaused),
    Alert(InstanceId),
}

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
struct Npc {
    base: Base<CharacterBody3D>,

    #[init(val = Speed::WALK)]
    speed: Speed,

    #[init(val = NpcState::Paused(DurationPaused(3.0)))]
    state: NpcState,

    #[init(val = 0.0)]
    elapsed: f32,

    #[init(val = rand::rng())]
    rng: rand::rngs::ThreadRng,

    #[init(val = BoundingBox { min: Vector3::new(-30.0, 0.0, -30.0), max: Vector3::new(30.0, 0.0, 30.0) })]
    patrol_area: BoundingBox,
}

#[godot_api]
impl ICharacterBody3D for Npc {
    fn physics_process(&mut self, delta: f64) {
        let delta = delta as f32;
        match self.state {
            NpcState::Moving(target) => {
                self.do_move(target, delta);
            }
            NpcState::Paused(duration) => {
                self.elapsed += delta;
                if self.elapsed >= duration.0 {
                    self.state = NpcState::Moving(self.patrol_area.random_point(&mut self.rng));
                    self.elapsed = 0.0;
                }
            }
            NpcState::Alert(_instance) => todo!(),
        };
    }
}

impl Npc {
    fn do_move(&mut self, target: Vector3, delta: f32) {
        // find direction to move in
        let current_pos = self.base().get_global_position();
        let to_target = target - current_pos;
        let direction = if to_target.length() >= 0.3 {
            to_target.normalized()
        } else {
            self.state = NpcState::Paused(DurationPaused(self.rng.random_range(0.0..5.0)));
            Vector3::ZERO
        };

        // rotate npc
        if direction != Vector3::ZERO {
            let target_y_rotation = direction.x.atan2(direction.z);
            self.rotate_towards_y(target_y_rotation, delta);
        }

        // move npc
        let gravity = if !self.base().is_on_floor() {
            self.base().get_gravity()
        } else {
            Vector3::ZERO
        };

        MovementCharacter::from_input(self.speed, direction, gravity, delta, &self.base())
            .apply_to(&mut self.base_mut());
        self.base_mut().move_and_slide();
    }

    fn rotate_towards_y(&mut self, angle: f32, delta: f32) {
        let current_rotation = self.base().get_rotation().y;
        let mut diff = current_rotation - angle;
        diff = (diff + PI).rem_euclid(2.0 * PI) - PI;
        let max_step = Speed::TURN.0 * delta;
        let step = diff.clamp(-max_step, max_step);
        self.base_mut().rotate_y(step);
    }
}

trait PatrolArea {
    fn random_point(&self, rng: &mut impl RngExt) -> Vector3;
}

struct BoundingBox {
    min: Vector3,
    max: Vector3,
}

impl PatrolArea for BoundingBox {
    fn random_point(&self, rng: &mut impl RngExt) -> Vector3 {
        Vector3::new(
            rng.random_range(self.min.x..self.max.x),
            0.0, //rng.random_range(self.min.y..self.max.y),
            rng.random_range(self.min.z..self.max.z),
        )
    }
}
