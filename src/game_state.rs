use bevy::prelude::*;

use crate::bat::*;
use crate::ball::*;

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    Splash,
    #[default]
    Menu,
    Game,
    Gameover,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_plugins(BatPlugin)
            .add_plugins(BallPlugin)
            .add_state::<GameState>();
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(Scoreboard {
        score_a: 0,
        score_b: 0,
    });
}

// This resource tracks the game's score
#[derive(Resource)]
pub struct Scoreboard {
    pub score_a: usize,
    pub score_b: usize,
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;
