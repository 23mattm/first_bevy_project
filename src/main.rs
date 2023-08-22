use std::f32::consts::PI;

use bevy::{prelude::*, window::{WindowMode, WindowResolution}, time};

use devcaders::{DevcadeControls, Player, Button};


enum Direction {
    Left,
    Right
}

//devcade res: 1080 x 2560

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        mode: WindowMode::Windowed,
        resolution: WindowResolution::new(270.0, 630.0),
        ..default()
      }),
      ..default()
    }))
    .add_startup_system(startup_system)
    .add_system(runtime_system)
    .run();
}


#[derive(Component)]
struct PlayerSprite {
    direction: Direction,
    velocity: f32
}



// ///mary strodl test
// fn hello_world_system(
//     time: Res<Time>,
//     mut sprite_position: Query<(&mut PlayerSprite, &mut Transform)>,
//     // devcade_controls: DevcadeControls
//   ) {
//     for (_, mut transform) in &mut sprite_position {
//         println!("{}", transform.translation.x);
//         transform.rotate_local_x(time.delta_seconds() * PI/4.0);
//         println!("rotation quat: {}", transform.rotation)
//     //   if devcade_controls.pressed(Player::P1, Button::StickLeft) {
//     //     transform.translation.x -= 5.0 * time.delta_seconds();
//     //   }
//     //   if devcade_controls.pressed(Player::P1, Button::StickRight) {
//     //     transform.translation.x += 5.0 * time.delta_seconds();
//     //   }
//     }
//     println!("hello world");
//   }
// /// 


fn startup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        PlayerSprite {
            direction: Direction::Left,
            velocity: 0.0
        },
        SpriteBundle {
            texture: asset_server.load("player_sprite.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
    ));
}

fn runtime_system(
    time: Res<Time>,
    mut sprite_position: Query<(&mut PlayerSprite, &mut Transform)>,
    devcade_controls: DevcadeControls,
) {
    for (mut sprite, mut transform) in &mut sprite_position {
        println!("{}", match sprite.as_mut().direction {
            Direction::Left => "Left",
            Direction::Right => "Right",
            _ => "uhhh"
        });
        // if devcade_controls.pressed(Player::P1, Button::StickLeft) {
        //     set_direction(sprite.as_mut(), Direction::Left)
        // } else if devcade_controls.pressed(Player::P1, Button::StickRight) {
        //     set_direction(&mut sprite, Direction::Right)
        // }
        // sprite.velocity += match sprite.direction {
        //     Direction::Left => {println!("Going left"); -9.8 * time.delta_seconds()},
        //     Direction::Right => {println!("Going right"); 9.8 * time.delta_seconds()},
        //     _ => 0.0
        // }
    }
}

fn set_direction(player: &mut PlayerSprite, direction: Direction) {
    player.direction = direction;
    // player.direction = 
    //     match player.direction {
    //         Direction::Left => Direction::Right,
    //         Direction::Right => Direction::Left
    //     };
}

fn move_sprite(player: PlayerSprite) {

}