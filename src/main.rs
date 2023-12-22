mod bat;
mod movement;
mod game_state;

use bevy::{
    core::FrameCount,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowTheme, WindowMode, WindowResized},
};

use game_state::GameStatePlugin;
use movement::MovementPlugin;
use bat::BatPlugin;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 480;
const TITLE: &str = "Boing!";

const HALF_WIDTH: f32 = (WIDTH / 2) as f32;
const HALF_HEIGHT: f32 = (HEIGHT / 2) as f32;

const PLAYER_SPEED: i32 = 6;
const MAX_AI_SPEED: i32 = 6;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Boing!".into(),
                    resolution: (800., 480.).into(),
                    resizable: true,
                    // mode: WindowMode::BorderlessFullscreen,
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    // This will spawn an invisible window
                    // The window will be made visible in the make_visible() system after 3 frames.
                    // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                    visible: false,
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins(GameStatePlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(BatPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update, 
            (
                make_visible,
                window_resized_event,
            ),
        )
        .run();
}

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -1.0),
                scale:Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            texture: asset_server.load("images/table.png"),
            ..default()
        },
    ));
}

fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    // The delay may be different for your app or system.
    if frames.0 == 3 {
        // At this point the gpu is ready to show the app so we can make the window visible.
        // Alternatively, you could toggle the visibility in Startup.
        // It will work, but it will have one white frame before it starts rendering
        window.single_mut().visible = true;
    }
}

fn window_resized_event(mut events: EventReader<WindowResized>, mut window: Query<&mut Window>) {
    for e in events.read() {
        // When resolution is being changed
        println!("{e:?}");
        let height = window.single().height();
        println!("{height:?}");
    }
    // event.width, event.height
}

// This resource tracks the game's score
#[derive(Resource)]
struct Scoreboard {
    score: usize,
}