use godot::prelude::*;

mod shared;

mod npc;
mod player;
mod site;
mod stick;
mod zone;

struct GameExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GameExtension {}
