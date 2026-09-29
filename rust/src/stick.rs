use godot::classes::{Area3D, INode3D, Node3D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct Stick {
    base: Base<Node3D>,

    #[init(node = "CollisionArea")]
    physics: OnReady<Gd<Area3D>>,
}

#[godot_api]
impl Stick {
    #[signal]
    pub fn hit(body: Gd<Node3D>);
}

#[godot_api]
impl INode3D for Stick {
    fn ready(&mut self) {
        let mut this = self.to_gd();
        self.physics
            .signals()
            .body_entered()
            .connect(move |body: Gd<Node3D>| this.bind_mut().on_body_entered(body));
    }
}

impl Stick {
    fn on_body_entered(&mut self, body: Gd<Node3D>) {
        self.signals().hit().emit(&body);
    }

    pub fn set_monitoring(&mut self, monitoring: bool) {
        self.physics.set_monitoring(monitoring);
    }
}
