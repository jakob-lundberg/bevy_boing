use bevy::prelude::*;

use crate::movement::Velocity;
use crate::HALF_HEIGHT;
use crate::HALF_WIDTH;

const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 0.0);

#[derive(Bundle)]
struct BatBundle {
    velocity: Velocity,
    model: SpriteBundle,
}

#[derive(Component)]
struct Paddle;


pub struct BatPlugin;

impl Plugin for BatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bat);
    }
}

fn spawn_bat(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn( BatBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        model: SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-(HALF_WIDTH - 40.0) , 0.0, 1.0),
                ..default()
            },
            texture: asset_server.load("images/bat01.png"),
            ..default()
        },
    });
    commands.spawn( BatBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        model: SpriteBundle {
            transform: Transform {
                translation: Vec3::new((HALF_WIDTH - 40.0) , 0.0, 1.0),
                ..default()
            },
            texture: asset_server.load("images/bat10.png"),
            ..default()
        },
    });
}
