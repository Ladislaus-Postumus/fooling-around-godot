use godot::{
    classes::{Area3D, BoxShape3D, CollisionShape3D, IArea3D},
    prelude::*,
};
use rand::RngExt;

#[derive(GodotConvert, Var, Export, Clone, Copy, PartialEq, Eq, Debug)]
#[godot(via = GString)]
pub enum ZoneKind {
    Sleeping,
    Relaxing,
    Patrol,
}

#[derive(GodotClass)]
#[class(init, base=Area3D)]
pub struct Zone {
    base: Base<Area3D>,

    #[export]
    #[init(val = ZoneKind::Patrol)]
    pub kind: ZoneKind,

    #[init(node = "CollisionShape3D")]
    shape: OnReady<Gd<CollisionShape3D>>,
}

#[godot_api]
impl IArea3D for Zone {
    fn ready(&mut self) {
        self.half_extent();
    }
}

impl Zone {
    fn half_extent(&self) -> Vector3 {
        let Some(shape) = self.shape.get_shape() else {
            godot_error!("Zone has no collision");
            return Vector3::ZERO;
        };

        let boxx = match shape.try_cast::<BoxShape3D>() {
            Ok(boxx) => boxx,
            Err(other) => {
                godot_error!(
                    "Zone `{}` must be box shaped, found {}",
                    self.base().get_path(),
                    other.get_class()
                );
                return Vector3::ZERO;
            }
        };

        boxx.get_size() * 0.5
    }

    pub fn random_point(&self, rng: &mut impl RngExt) -> Vector3 {
        let box_size = self.half_extent();
        let point = Vector3::new(
            rng.random_range(-box_size.x..=box_size.x),
            rng.random_range(-box_size.y..=box_size.y),
            rng.random_range(-box_size.z..=box_size.z),
        );
        self.shape.get_global_transform() * point
    }

    pub fn contains_xz(&self, point: Vector3) -> bool {
        let box_size = self.half_extent();
        let local = self.shape.get_global_transform().affine_inverse() * point;
        local.x.abs() <= box_size.x && local.z.abs() <= box_size.z
    }
}
