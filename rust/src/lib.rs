use godot::prelude::*;

mod shared;

mod npc;
mod player;
mod stick;

struct GameExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GameExtension {}
