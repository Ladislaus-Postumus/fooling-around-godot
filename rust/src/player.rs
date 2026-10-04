use crate::npc::Npc;
use crate::shared::{MovementCharacter, Speed};
use crate::stick::Stick;
use godot::classes::{AnimationPlayer, CharacterBody3D, ICharacterBody3D, Input, Node3D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
struct Player {
    base: Base<CharacterBody3D>,

    #[export]
    #[init(val = Speed::WALK)]
    speed: Speed,

    #[init(val = 4.5)]
    jump_velocity: f32,

    #[init(val = false)]
    swinging: bool,

    pitch: f32,

    #[init(node = "Head")]
    head: OnReady<Gd<Node3D>>,

    #[init(node = "AnimationPlayer")]
    animation_player: OnReady<Gd<AnimationPlayer>>,

    #[init(node = "Hand/Stick")]
    stick: OnReady<Gd<Stick>>,
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn ready(&mut self) {
        Input::singleton().set_mouse_mode(godot::classes::input::MouseMode::CAPTURED);

        let mut this = self.to_gd();
        self.stick
            .signals()
            .hit()
            .connect(move |body: Gd<Node3D>| this.bind_mut().on_hit(body));
    }

    fn physics_process(&mut self, delta: f64) {
        let delta = delta as f32;
        let input = Input::singleton();

        let gravity = if self.base().is_on_floor() {
            Vector3::ZERO
        } else {
            self.base().get_gravity()
        };

        if input.is_action_just_pressed("sneak") && self.speed == Speed::WALK {
            self.speed = Speed::SNEAK;
        } else if input.is_action_just_pressed("sprint") && self.speed == Speed::WALK {
            self.speed = Speed::RUN;
        } else if (input.is_action_just_pressed("sneak") && self.speed == Speed::SNEAK)
            || (input.is_action_just_pressed("sprint") && self.speed == Speed::RUN)
        {
            self.speed = Speed::WALK;
        }

        let input_dir = input.get_vector("move_left", "move_right", "move_up", "move_down");
        let raw_direction =
            self.base().get_transform().basis * Vector3::new(input_dir.x, 0.0, input_dir.y);
        let direction = if raw_direction == Vector3::ZERO {
            Vector3::ZERO
        } else {
            raw_direction.normalized()
        };

        let mut movement =
            MovementCharacter::from_input(self.speed, direction, gravity, delta, &self.base_mut());

        if input.is_action_just_pressed("jump") && self.base().is_on_floor() {
            movement.jump(self.jump_velocity);
        }

        movement.apply_to(&mut self.base_mut());
        self.base_mut().move_and_slide();

        if input.is_action_just_pressed("swing") && !self.swinging {
            self.swinging = true;
            self.stick.bind_mut().set_monitoring(true);
            self.animation_player.play_ex().name("swing").done();
        }
        if self.swinging && !self.animation_player.is_playing() {
            self.swinging = false;
            self.stick.bind_mut().set_monitoring(false);
            self.animation_player.play_ex().name("RESET").done();
        }
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
impl Player {
    fn on_hit(&mut self, body: Gd<Node3D>) {
        if let Ok(mut npc) = body.try_cast::<Npc>() {
            let direction = (npc.get_global_position() - self.base().get_global_position())
                .normalized_or_zero();
            let impulse = 10.0 * direction;

            npc.bind_mut().apply_knockback(impulse);
        }
    }
}
