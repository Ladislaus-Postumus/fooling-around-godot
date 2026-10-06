use std::f32::consts::PI;
use std::panic;

use crate::shared::{MovementCharacter, Speed};
use crate::site::Site;
use crate::zone::ZoneKind;
use godot::classes::{CharacterBody3D, ICharacterBody3D, NavigationAgent3D, NavigationServer3D};
use godot::prelude::*;
use rand::RngExt;

#[derive(GodotConvert, Var, Export, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[godot(transparent)]
struct DurationPaused(f32);

#[derive(Clone, Copy, PartialEq, Debug)]
struct NextTarget(ZoneKind);

#[derive(Debug)]
enum NpcState {
    Moving(Vector3),
    Paused(DurationPaused, NextTarget),
    Alert(InstanceId),
}

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub struct Npc {
    base: Base<CharacterBody3D>,

    #[init(val = Speed::WALK)]
    speed: Speed,

    #[init(val = NpcState::Paused(DurationPaused(3.0), NextTarget(ZoneKind::Patrol)))]
    state: NpcState,

    #[init(val = 0.0)]
    elapsed: f32,

    #[init(val = rand::rng())]
    rng: rand::rngs::ThreadRng,

    #[init(val = Vector3::ZERO)]
    knockback: Vector3,

    #[init(node = "NavigationAgent3D")]
    agent: OnReady<Gd<NavigationAgent3D>>,

    home: Option<Gd<Site>>,
}

#[godot_api]
impl ICharacterBody3D for Npc {
    fn ready(&mut self) {
        self.home = self.find_site();
        if self.home.is_none() {
            godot_warn!("Npc `{}` is not below a Site", self.base().get_path());
        }
    }

    fn physics_process(&mut self, delta: f64) {
        #[allow(clippy::cast_possible_truncation)]
        let delta = delta as f32;
        match self.state {
            NpcState::Moving(_) => {
                self.do_move(delta);
            }
            NpcState::Paused(duration, next_target) => {
                self.elapsed += delta;
                if self.elapsed >= duration.0 {
                    if let Some(target) = self.pick_target(next_target.0) {
                        self.agent.set_target_position(target);
                        self.state = NpcState::Moving(target);
                    }
                    self.elapsed = 0.0;
                }
            }
            NpcState::Alert(_instance) => todo!(),
        }
    }
}

impl Npc {
    pub fn apply_knockback(&mut self, knockback: Vector3) {
        self.knockback += knockback;
    }

    fn do_move(&mut self, delta: f32) {
        // find direction to move in
        let direction = if self.agent.is_navigation_finished() {
            match self.rng.random_range(0..=1) {
                0 => {
                    self.state = NpcState::Paused(
                        DurationPaused(self.rng.random_range(3.0..10.0)),
                        NextTarget(ZoneKind::Patrol),
                    );
                }
                1 => {
                    self.state = NpcState::Paused(
                        DurationPaused(self.rng.random_range(0.0..5.0)),
                        NextTarget(ZoneKind::Relaxing),
                    );
                }
                _ => {
                    (); // pass, can't happen due to rng range
                }
            }
            Vector3::ZERO
        } else {
            let to_next = self.agent.get_next_path_position() - self.base().get_global_position();
            Vector3::new(to_next.x, 0.0, to_next.z).normalized_or_zero()
        };

        // rotate npc
        if direction != Vector3::ZERO {
            let target_y_rotation = (-direction.x).atan2(-direction.z);
            self.rotate_towards_y(target_y_rotation, delta);
        }

        // move npc
        let gravity = if self.base().is_on_floor() {
            Vector3::ZERO
        } else {
            self.base().get_gravity()
        };

        let mut movement =
            MovementCharacter::from_input(self.speed, direction, gravity, delta, &self.base());
        movement.current_velocity += self.knockback;
        movement.apply_to(&mut self.base_mut());
        self.knockback = self.knockback.move_toward(Vector3::ZERO, 1.0);

        self.base_mut().move_and_slide();
    }

    fn rotate_towards_y(&mut self, angle: f32, delta: f32) {
        let current_rotation = self.base().get_rotation().y;
        let mut diff = angle - current_rotation;
        diff = (diff + PI).rem_euclid(2.0 * PI) - PI;
        let max_step = Speed::TURN.0 * delta;
        let step = diff.clamp(-max_step, max_step);
        self.base_mut().rotate_y(step);
    }

    fn find_site(&self) -> Option<Gd<Site>> {
        let mut node = self.base().get_parent();
        while let Some(current) = node {
            match current.try_cast::<Site>() {
                Ok(site) => return Some(site),
                Err(other) => node = other.get_parent(),
            }
        }
        None
    }

    fn pick_target(&mut self, kind: ZoneKind) -> Option<Vector3> {
        let site = self.home.as_ref()?;
        let map = self.agent.get_navigation_map();
        site.bind().random_target(kind, &mut self.rng, map)
    }
}
