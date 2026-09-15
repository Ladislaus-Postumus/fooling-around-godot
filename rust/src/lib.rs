use godot::prelude::*;

mod player;
mod shared;

struct GameExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GameExtension {}
