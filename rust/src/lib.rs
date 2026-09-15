use godot::prelude::*;

mod shared;

mod npc;
mod player;

struct GameExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GameExtension {}
