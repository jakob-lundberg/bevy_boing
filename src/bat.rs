use bevy::prelude::*;

use crate::PLAYER_SPEED;
use crate::movement::Velocity;
use crate::game_state::GameState;
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

#[derive(Resource)]
struct BatImage {
    img00: Handle<Image>,
    img01: Handle<Image>,
}

pub struct BatPlugin;

impl Plugin for BatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(OnEnter(GameState::Menu), spawn_bats)
            .add_systems(OnEnter(GameState::Game), spawn_bats)
            .add_systems(
                Update, 
                (
                    player_input_system,
                ),
            );
    }
}

fn setup (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource( BatImage { 
        img00: asset_server.load("images/bat00.png"),
        img01: asset_server.load("images/bat01.png"),
    });
}


fn spawn_bats(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<Paddle>>,
    state: ResMut<State<GameState>>,
) {
    for bat in query.iter() {
        commands.entity(bat).despawn();
    }
    commands.spawn((
        Paddle,
        Velocity {
            value: STARTING_VELOCITY,
        },
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-(HALF_WIDTH - 40.0) , 0.0, 1.0),
                ..default()
            },
            texture: asset_server.load("images/bat00.png"),
            ..default()
        },
    ));
    commands.spawn((
        Paddle,
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
    mut query: Query<&mut Velocity, With<Paddle>>,
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
