use std::process;

use bevy::{prelude::*, window::{WindowMode, WindowResolution}, time, transform::{self, commands}};

use devcaders::{DevcadeControls, Player, Button};

use rand::Rng;

//devcade res: 1080 x 2560
const SCALE: f32 = 0.3;
const SCALE_VEC: Vec3 = Vec3::new(SCALE, SCALE, SCALE);
const SCREEN_WIDTH: f32 = 1080.0 * SCALE; // 360 test
const SCREEN_HEIGHT: f32 = 2560.0 * SCALE; // 840 test
const ACCELERATION: f32 = SCREEN_WIDTH * 1.5;
const PLAYER_LENGTH: f32 = 100.0 * SCALE;
const OBSTACLE_HEIGHT: f32 = 256.0 * SCALE;
const OBSTACLE_GAP_WIDTH: f32 = 373.0 * SCALE;
const OBSTACLE_WAIT: f32 = 2.0;
const OBSTACLE_SPEED: f32 = 5.0; //how long it takes for the pipe to move down

static mut last_obstacle_time: f32 = 0.0;
static mut spawned_obstacles: Vec<Entity> = Vec::new();

#[derive(PartialEq)]
enum Direction {
    Left,
    Right
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameStates {
    MainMenu,
    Playing,
    Dead
}


fn main() {  
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            mode: WindowMode::Windowed,
            resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
            resizable: false,
            ..default()
        }),
        ..default()
        }))
        .add_startup_system(startup_system)
        .add_system(runtime_system)//Move player sprite and much more
        .add_system(spawn_obstacles)
        .add_system(move_obstacles)
        .add_system(check_collisions)
        .add_system(cleanup_system)
        .run();
}


#[derive(Component)]
struct PlayerSprite {
    direction: Direction,
    velocity: f32
}

#[derive(Component)]
struct Obstacle {}

#[derive(Component)]
struct Background {}

#[derive(Component)]
struct TutorialText {}


fn startup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        PlayerSprite {
            direction: Direction::Left,
            velocity: 0.0
        },
        SpriteBundle {
            texture: asset_server.load("spritegen2.png"),
            transform: Transform::from_xyz(0., -SCREEN_HEIGHT/3., 1.).with_scale(SCALE_VEC),
            ..default()
        },
    ));

    commands.spawn((
        TutorialText {},
        SpriteBundle {
            texture: asset_server.load("instructiontext.png"),
            transform: Transform::from_xyz(0., 2. * SCREEN_HEIGHT / 3., 1.).with_scale(SCALE_VEC),
            ..default()
        }
    ));

    commands.spawn((
        Background {},
        SpriteBundle {
            texture: asset_server.load("BackgroundSprite.png"),
            // transform: Transform::from_xyz(0., SCREEN_HEIGHT / 2., 0.).with_scale(SCALE_VEC),
            transform: Transform::from_xyz(0., 0., 0.).with_scale(SCALE_VEC),
            ..default()
        }
    ));

    println!("Background Transform: {:?}", Transform::from_xyz(0., SCREEN_HEIGHT / 2., 0.).with_scale(SCALE_VEC));
}

fn move_bg(time: Res<Time>, mut bg: Query<(&mut Background, &mut Transform)>) {
    for (_, mut transform) in &mut bg {
        let mut temp = transform.translation.y - (time.delta_seconds() * SCREEN_HEIGHT / (OBSTACLE_SPEED * 2.0));
        if temp <= -SCREEN_HEIGHT / 2.0 {
            temp += SCREEN_HEIGHT;
        }
        transform.translation.y = temp;// % SCREEN_HEIGHT;
    }
}

fn spawn_obstacles(
    mut commands: Commands, 
    time: Res<Time>,
    asset_server: Res<AssetServer>) {
    //Spawning obstacles
    // println!("elapsed time: {}", time.elapsed_seconds());

    if time.elapsed_seconds() - unsafe { last_obstacle_time } > OBSTACLE_WAIT {
        unsafe { last_obstacle_time += OBSTACLE_WAIT }
        let x = rand::thread_rng().gen_range(-SCREEN_WIDTH/3.0..=SCREEN_WIDTH/3.0);
        let entity_id = commands.spawn((
            Obstacle {},
            SpriteBundle {
                transform: Transform::from_xyz(x, 3. * SCREEN_HEIGHT / 2., 2.).with_scale(SCALE_VEC),
                texture: asset_server.load("obstaclesprite.png"),
                ..default()
            })
        ).id();
        unsafe {spawned_obstacles.push(entity_id)};
    }
    // println!("Spawned obstacles: {}", unsafe {spawned_obstacles});
}

fn check_collisions(
    mut set: ParamSet<(
        Query<(&mut PlayerSprite, &mut Transform)>,
        Query<(&mut Obstacle, &mut Transform)>
    )>
    ) {
        let mut player_transform: Option<Transform> = None;
        for (_, transform) in set.p0().iter_mut() {
            player_transform = Some(*transform);
            break;
        }
        match player_transform {
            Some(transform) => {
                for (_, obstacle_transform) in &mut set.p1().iter_mut() {
                    //trying /3 for more hitbox leniency on pipe collisions
                    if (transform.translation.y - obstacle_transform.translation.y).abs() <= 
                        (PLAYER_LENGTH / 3.0) + (OBSTACLE_HEIGHT / 2.0) &&//if the player is "touching" a pipe vertically
                        (transform.translation.x - obstacle_transform.translation.x).abs() >=
                        (OBSTACLE_GAP_WIDTH / 2.0) - (PLAYER_LENGTH / 3.0)
                    {
                        // println!("Touching a pipe");
                        end();
                    }
                }
            }
            None => return
        }
    }

fn move_obstacles(
    time: Res<Time>, 
    mut obstacles: Query<(&mut Obstacle, &mut Transform)>
) {
    for (_,mut transform) in &mut obstacles {
        transform.translation.y -= time.delta_seconds() * SCREEN_HEIGHT / OBSTACLE_SPEED;
    }
}


fn runtime_system(
    time: Res<Time>,
    mut sprite_position: Query<(&mut PlayerSprite, &mut Transform)>,
    devcade_controls: DevcadeControls,
) {
    //Player movement
    for (mut sprite, mut transform) in &mut sprite_position {
        //process player input
        if devcade_controls.pressed(Player::P1, Button::StickLeft) {
            set_direction(&mut sprite, Direction::Left)
        } else if devcade_controls.pressed(Player::P1, Button::StickRight) {
            set_direction(&mut sprite, Direction::Right)
        } else if devcade_controls.just_pressed(Player::P1, Button::A1) {
            toggle_direction(&mut sprite)
        }


        let on_wall: bool = (transform.translation.x.abs() + (PLAYER_LENGTH / 2.0)) >= SCREEN_WIDTH / 2.0;
        let mut curr_wall: Option<Direction> = None;

        if on_wall {
            curr_wall = if (transform.translation.x) > 0.0 {Some(Direction::Right)} else {Some(Direction::Left)}
        };

        let add_vel = match sprite.direction {
            Direction::Left => {
                -ACCELERATION * time.delta_seconds()
            },
            Direction::Right => {
                ACCELERATION * time.delta_seconds()
            }
        };

        sprite.velocity += match curr_wall {
            None => {add_vel},
            Some(wall_direction) => {
                if wall_direction == sprite.direction {-sprite.velocity} else {add_vel} //if you're trying to move into a wall you cant
            }
        };

        
        transform.translation.x += sprite.velocity * time.delta_seconds();
        transform.rotate_z(-time.delta_seconds() * sprite.velocity / (50.0));
    }

    //End program
    if devcade_controls.pressed(Player::P1, Button::Menu) ||
        devcade_controls.pressed(Player::P2, Button::Menu)
    {end()}
}

fn cleanup_system(mut commands: Commands) {
    for pipe in unsafe {spawned_obstacles.iter()} {
        println!("Vector length: {}", unsafe {spawned_obstacles.len()});
        println!("{:?}", pipe)
    }
}


fn set_direction(player: &mut PlayerSprite, direction: Direction) {
    player.direction = direction;
}

fn toggle_direction(player: &mut PlayerSprite) {
    player.direction = match player.direction {
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left
    }
}

fn end() {
    process::exit(0)
}