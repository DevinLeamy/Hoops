use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    utils::{create_collider, PixelSprite},
    GameAssets, GROUND_LEVEL, WINDOW_WIDTH,
};

const HOOP_WIDTH: f32 = 400.0;
const HOOP_HEIGHT: f32 = HOOP_WIDTH * 1.5;

#[derive(Component)]
pub struct Hoop {
    pub name: Name,
    pub top_sensor_id: Entity,
    pub bottom_sensor_id: Entity,
}
impl PixelSprite for Hoop {
    fn pixel_size() -> f32 {
        HOOP_WIDTH / 32.0
    }

    fn size() -> Vec2 {
        Vec2::new(HOOP_WIDTH, HOOP_HEIGHT)
    }
}

pub fn spawn_hoop(commands: &mut Commands, assets: &Res<GameAssets>) {
    let backboard = create_collider::<Hoop>(commands, 1, 8, 9, 40);
    let front_rim = create_collider::<Hoop>(commands, 1, 1, 18, 40);
    let back_rim = create_collider::<Hoop>(commands, 1, 1, 10, 40);
    let top_sensor = create_collider::<Hoop>(commands, 7, 1, 11, 42);
    let bottom_sensor = create_collider::<Hoop>(commands, 7, 1, 11, 38);

    commands.entity(top_sensor).insert(Sensor);
    commands.entity(bottom_sensor).insert(Sensor);

    let hoop_id = commands
        .spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        -WINDOW_WIDTH / 2.0 + HOOP_WIDTH / 2.0,
                        GROUND_LEVEL + HOOP_HEIGHT / 2.0,
                        0.0,
                    ),
                    ..Default::default()
                },
                texture: assets.hoop.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(HOOP_WIDTH, HOOP_HEIGHT)),
                    ..default()
                },
                ..default()
            },
            Hoop {
                name: Name::new("hoop"),
                top_sensor_id: top_sensor,
                bottom_sensor_id: bottom_sensor,
            },
        ))
        .id();

    commands
        .entity(hoop_id)
        .add_child(backboard)
        .add_child(front_rim)
        .add_child(back_rim)
        .add_child(top_sensor)
        .add_child(bottom_sensor);
}
