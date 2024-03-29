mod bat;
mod ball;
mod game_state;
mod menu;
mod movement;
mod setup;

use bevy::{
    core::FrameCount,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowResized, WindowTheme},
};

use game_state::GameStatePlugin;
use movement::MovementPlugin;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 480;
const TITLE: &str = "Boing!";

const HALF_WIDTH: f32 = (WIDTH / 2) as f32;
const HALF_HEIGHT: f32 = (HEIGHT / 2) as f32;

const PLAYER_SPEED: f32 = 400.0;
const MAX_AI_SPEED: f32 = 400.0;

fn main() {
    App::new()
        .add_plugins(GameStatePlugin)
        .add_plugins(menu::MenuPlugin)
        .add_plugins(setup::GameSetupPlugin)
        .run();
}
