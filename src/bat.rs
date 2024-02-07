use bevy::prelude::*;

use crate::game_state::GameState;
use crate::movement::{Velocity, Collider};
use crate::HALF_HEIGHT;
use crate::HALF_WIDTH;
use crate::PLAYER_SPEED;

const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 0.0);

#[derive(Bundle)]
struct BatBundle {
    velocity: Velocity,
    model: SpriteBundle,
}

#[derive(Component)]
struct Bat {
    side: i32,
}

#[derive(Resource)]
pub struct BatImage {
    pub img00: Handle<Image>,
    pub img01: Handle<Image>,
}

pub struct BatPlugin;

impl Plugin for BatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(OnEnter(GameState::Menu), spawn_bats)
            .add_systems(OnEnter(GameState::Game), spawn_bats)
            .add_systems(Update, (player_input_system, update_position));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(BatImage {
        img00: asset_server.load("images/bat00.png"),
        img01: asset_server.load("images/bat01.png"),
    });
}

fn spawn_bats(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<Bat>>,
    state: ResMut<State<GameState>>,
) {
    for bat in query.iter() {
        commands.entity(bat).despawn();
    }
    commands.spawn((
        Bat { side: 0 },
        Collider,
        Velocity {
            value: STARTING_VELOCITY,
        },
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-(HALF_WIDTH - 40.0), 0.0, 1.0),
                ..default()
            },
            texture: asset_server.load("images/bat00.png"),
            ..default()
        },
    ));
    commands.spawn((
        Bat { side: 1 },
        Collider,
        Velocity {
            value: STARTING_VELOCITY,
        },
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(HALF_WIDTH - 40.0, 0.0, 1.0),
                ..default()
            },
            texture: asset_server.load("images/bat10.png"),
            ..default()
        },
    ));
}

fn player_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Bat>>,
    game_state: ResMut<NextState<GameState>>,
) {
    for mut velocity in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Down) {
            velocity.value = Vec3::new(0.0, -PLAYER_SPEED, 0.0);
        } else if keyboard_input.pressed(KeyCode::Up) {
            velocity.value = Vec3::new(0.0, PLAYER_SPEED, 0.0);
        } else {
            velocity.value = Vec3::new(0.0, 0.0, 0.0);
        }
    }
}


fn update_position(mut query: Query<(&Velocity, &mut Transform), With<Bat>>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
        if transform.translation.y > 160.0 {
            transform.translation.y = 160.0;
        }
        if transform.translation.y < -160.0 {
            transform.translation.y = -160.0;
        }
    }
}
