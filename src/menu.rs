use bevy::prelude::*;

use crate::game_state::GameState;

#[derive(Component)]
struct Menu;

#[derive(Resource)]
struct GameSettings {
    players: i32,
}

#[derive(Resource)]
struct MenuImage {
    img0: Handle<Image>,
    img1: Handle<Image>,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnEnter(GameState::Game), despawn_menu)
            .add_systems(Update, (player_input_system,));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameSettings { players: 1 });
    commands.insert_resource(MenuImage {
        img0: asset_server.load("images/menu0.png"),
        img1: asset_server.load("images/menu1.png"),
    });
}

fn spawn_menu(mut commands: Commands, menu_images: ResMut<MenuImage>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 2.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            texture: menu_images.img0.clone(),
            ..default()
        },
        Menu,
    ));
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<Menu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn player_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Handle<Image>, With<Menu>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut settings: ResMut<GameSettings>,
    menu_images: ResMut<MenuImage>,
) {
    for mut texture in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) {
            settings.players = 1;
            *texture = menu_images.img0.clone();
        } else if keyboard_input.pressed(KeyCode::Down) {
            settings.players = 2;
            *texture = menu_images.img1.clone();
        } else if keyboard_input.pressed(KeyCode::Space) {
            game_state.set(GameState::Game);
        }
    }
}
