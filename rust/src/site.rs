use godot::{classes::NavigationServer3D, prelude::*};
use rand::RngExt;

use crate::zone::{Zone, ZoneKind};

#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct Site {
    base: Base<Node3D>,
}

impl Site {
    /// all direct child Zone nodes of given kind
    pub fn zones_of(&self, kind: ZoneKind) -> Vec<Gd<Zone>> {
        self.base()
            .get_children()
            .iter_shared()
            .filter_map(|child| child.try_cast::<Zone>().ok())
            .filter(|zone| zone.bind().kind == kind)
            .collect()
    }

    pub fn random_target(
        &self,
        kind: ZoneKind,
        rng: &mut impl RngExt,
        map: Rid,
    ) -> Option<Vector3> {
        let zones = self.zones_of(kind);
        if zones.is_empty() {
            return None; // random_range(0..0) would panic
        }
        let zone = zones.get(rng.random_range(0..zones.len()))?.bind();

        for _ in 0..10 {
            let sample = zone.random_point(rng);
            let snapped = NavigationServer3D::singleton().map_get_closest_point(map, sample);
            if zone.contains_xz(snapped) {
                return Some(snapped);
            }
        }
        None
    }
}
