use crate::*;
use bevy::prelude::*;

/// Plugin that does the game setup and related things
pub struct GameSetupPlugin;

impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: TITLE.into(),
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
        .add_systems(Startup, setup)
        .add_systems(Update, (make_visible, window_resized_event));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -1.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..default()
        },
        texture: asset_server.load("images/table.png"),
        ..default()
    },));
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

fn window_resized_event(
    mut events: EventReader<WindowResized>,
    window: Query<&mut Window>,
    mut cameras: Query<&mut OrthographicProjection, With<Camera>>,
) {
    let event: Option<&WindowResized> = events.read().last();
    if let Some(_e) = event {
        let window_height = window.single().height();
        let factor_x = (window.single().width()) / (WIDTH) as f32;
        let factor_y = (window_height) / (HEIGHT) as f32;
        for mut projection in cameras.iter_mut() {
            projection.scale = 1.0 / factor_x.min(factor_y);
        }
    }
}
