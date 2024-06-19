#![warn(clippy::all, clippy::pedantic)]

mod player;

use godot::{init::ExtensionLibrary, prelude::gdextension};

struct KnightsToSeeYouExtension;

#[gdextension]
unsafe impl ExtensionLibrary for KnightsToSeeYouExtension {}
