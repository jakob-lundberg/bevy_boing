use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
};
use crate::{game_state::GameState, WIDTH, game_state::Scoreboard};
use crate::bat::BatImage;
use crate::movement::{Velocity, Collider};

const STARTING_VELOCITY: Vec3 = Vec3::new(-200.0, 1.0, 0.0);
const STARTING_SPEED: f32 = 200.0;

#[derive(Component)]
struct Ball {
    speed: f32,
}

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Event)]
struct GoalEvent();

#[derive(Resource)]
struct BallImage {
    img: Handle<Image>,
}


pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_event::<CollisionEvent>()
            .add_event::<GoalEvent>()
            .add_systems(OnEnter(GameState::Menu), spawn_ball)
            .add_systems(OnEnter(GameState::Game), spawn_ball)
            .add_systems(Update, (check_for_collisions, check_for_goal, update_position))
            .add_systems(Update, (on_goal));

    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(BallImage {
        img: asset_server.load("images/ball.png"),
    });
}

fn spawn_ball(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<Ball>>,
) {

    for ball in query.iter() {
        commands.entity(ball).despawn();
    }
    commands.spawn((
        Ball { speed: STARTING_SPEED },
        Velocity {
            value: Vec3 { x: -STARTING_SPEED, y: 0.0, z: 0.0 },
        },

        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..default()
            },
            texture: asset_server.load("images/ball.png"),
            ..default()
        },
    ));
}


fn check_for_collisions(
    mut commands: Commands,
    mut ball_query: Query<(&mut Velocity, &Transform, &mut Ball)>,
    collider_query: Query<(&Transform, &Handle<Image>), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {

    let query = ball_query.get_single_mut();
    let (mut ball_velocity, ball_transform, mut ball) = match query{
        Ok(query_result) => query_result,
        Err(_e) => return,
    };
    let ball_size = Vec2::new(12.0, 12.0); 

    // check collision with bats
    for (transform, _image_handle) in &collider_query {
        let image_dimensions = Vec2::new(18.0, 116.0); 
        let transform_scaled = transform.scale.truncate();
        let scaled_image_dimension = Vec2::new(image_dimensions.x as f32 * transform_scaled.x, image_dimensions.y as f32 * transform_scaled.y);

        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            scaled_image_dimension,
        );
        if let Some(collision) = collision {
            println!("Collision!");
            collision_events.send_default();

            // reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            // only reflect if the ball's velocity is going in the opposite direction of the
            // collision
            match collision {
                Collision::Left => reflect_x = ball_velocity.value.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.value.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.value.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.value.y > 0.0,
                Collision::Inside => { /* do nothing */ }
            }

            let diff_y = ball_transform.translation.y - transform.translation.y;
            ball_velocity.value.x = -ball_velocity.value.x;
            ball_velocity.value.y = diff_y * ball.speed * 0.01;
            ball_velocity.value = ball_velocity.value.normalize() * ball.speed;
            ball.speed += 80.0;

            // reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                println!("reflect_x!");

            }


            // reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                println!("reflect_y!");
                ball_velocity.value.y = -ball_velocity.value.y;
            }

        }
    }
}

fn check_for_goal(
    ball_query: Query<(&Transform, Entity), With<Ball>>,
    mut commands: Commands,
    mut game_state: ResMut<Scoreboard>,
    mut ev_goal: EventWriter<GoalEvent>,
) {

    let query = ball_query.get_single();
    let (ball_transform, ball) = match query{
        Ok(query_result) => query_result,
        Err(_e) => return,
    };
    if ball_transform.translation.x.abs() > (WIDTH / 2) as f32 {
        if ball_transform.translation.x > 0.0 {
            game_state.score_a += 1;
        } else {
            game_state.score_b += 1;
        }
        println!("Goal! {} - {}", game_state.score_a, game_state.score_b);

        commands.entity(ball).despawn();
        ev_goal.send(GoalEvent());
    }
}

fn on_goal(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ev_goal: EventReader<GoalEvent>,
) {
    for _ev in ev_goal.read() {
        // let id = world.register_system(spawn_ball);
        // let _ = world.run_system(id); // prints 1
        
        println!("on_goal");

        commands.spawn((
            Ball { speed: STARTING_SPEED },
            Velocity {
                value: Vec3 { x: -STARTING_SPEED, y: 0.0, z: 0.0 },
            },

            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..default()
                },
                texture: asset_server.load("images/ball.png"),
                ..default()
            },
        ));
     }
}

fn update_position(mut query: Query<(&mut Velocity, &mut Transform), With<Ball>>, time: Res<Time>) {
    for (mut velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
        if transform.translation.y > 220.0 || transform.translation.y < -220.0  {
            velocity.value.y = -velocity.value.y;
        }
    }
}
