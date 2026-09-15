use crate::shared::Speed;
use godot::classes::{CharacterBody3D, ICharacterBody3D, Input, Node3D};
use godot::global::move_toward;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
struct Player {
    base: Base<CharacterBody3D>,

    #[export]
    #[init(val = Speed::WALK.0)]
    speed: f32,

    #[export]
    #[init(val = 75.0)]
    fall_acceleration: f32,

    #[init(val = 4.5)]
    jump_velocity: f32,

    target_velocity: Vector3,
    pitch: f32,

    #[init(node = "Head")]
    head: OnReady<Gd<Node3D>>,
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn ready(&mut self) {
        Input::singleton().set_mouse_mode(godot::classes::input::MouseMode::CAPTURED);
    }

    fn physics_process(&mut self, delta: f64) {
        let delta = delta as f32;
        let input = Input::singleton();

        if !self.base().is_on_floor() {
            self.target_velocity += self.base().get_gravity() * delta;
        }

        if input.is_action_just_pressed("jump") && self.base().is_on_floor() {
            self.target_velocity.y = self.jump_velocity;
        }

        let input_dir = input.get_vector("move_left", "move_right", "move_up", "move_down");
        let basis = self.base().get_transform().basis;
        let raw_direction = basis * Vector3::new(input_dir.x, 0.0, input_dir.y);
        let direction = if raw_direction != Vector3::ZERO {
            raw_direction.normalized()
        } else {
            Vector3::ZERO
        };

        if direction != Vector3::ZERO {
            self.target_velocity.x = direction.x * self.speed;
            self.target_velocity.z = direction.z * self.speed;
        } else {
            self.target_velocity.x =
                move_toward(self.target_velocity.x as f64, 0.0, self.speed as f64) as f32;
            self.target_velocity.z =
                move_toward(self.target_velocity.z as f64, 0.0, self.speed as f64) as f32;
        }

        let velocity = self.target_velocity;
        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }

    fn input(&mut self, event: Gd<godot::classes::InputEvent>) {
        if let Ok(motion) = event.try_cast::<godot::classes::InputEventMouseMotion>() {
            let relative = motion.get_relative();
            self.base_mut().rotate_y(-relative.x * 0.002);

            self.pitch -= relative.y * 0.002;
            self.pitch = self.pitch.clamp(-89f32.to_radians(), 89f32.to_radians());

            let mut head_rot = self.head.get_rotation();
            head_rot.x = self.pitch;
            self.head.set_rotation(head_rot);
        }
    }
}
