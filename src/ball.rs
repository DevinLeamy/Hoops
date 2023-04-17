use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    BasketballThrown, GameAssets, RespawnBasketballTimer, ThrowBasketball, GRAVITY, GROUND_LEVEL,
};

const BALL_START_LOCATION_X: f32 = 300.0;
const BALL_START_LOCATION_Y: f32 = GROUND_LEVEL + BASKETBALL_SIZE / 2.0;
const BASKETBALL_SPEED_X: f32 = -800.0;
const BASKETBALL_MIN_SPEED_Y: f32 = 100.0;
const BASKETBALL_MAX_SPEED_Y: f32 = 4000.0;
const BASKETBALL_SIZE: f32 = 70.0;

#[derive(Component)]
pub struct Basketball {
    pub name: Name,
    // flag indicating whether the basketball has been thrown
    pub thrown: bool,
}

pub fn spawn_basketball(commands: &mut Commands, assets: &Res<GameAssets>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(BALL_START_LOCATION_X, BALL_START_LOCATION_Y, 0.),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(BASKETBALL_SIZE, BASKETBALL_SIZE)),
                ..default()
            },
            texture: assets.basketball.clone(),
            ..default()
        },
        Basketball {
            name: Name::new("basketball"),
            thrown: false,
        },
        RigidBody::Dynamic,
        Velocity::zero(),
        Collider::ball((BASKETBALL_SIZE - 10.0) / 2.0),
        Restitution {
            coefficient: 0.7,
            combine_rule: CoefficientCombineRule::Max,
        },
        Friction {
            coefficient: 0.5,
            combine_rule: CoefficientCombineRule::Max,
        },
        GravityScale(GRAVITY),
        ActiveEvents::COLLISION_EVENTS,
    ));
}

pub fn respawn_basketball(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut respawn_timer: ResMut<RespawnBasketballTimer>,
) {
    if !respawn_timer.timer.paused() && respawn_timer.timer.elapsed_secs() > 3.3 {
        respawn_timer.timer.reset();
        respawn_timer.timer.pause();

        spawn_basketball(&mut commands, &assets);
    }
}

pub fn despawn_basketballs(basketball_query: Query<(Entity, &Basketball)>, mut commands: Commands) {
    let mut has_unthrown: bool = false;
    for (_basketball_entity, basketball) in basketball_query.iter() {
        if !basketball.thrown {
            has_unthrown = true;
        }
    }

    for (basketball_entity, basketball) in basketball_query.iter() {
        if basketball.thrown && has_unthrown {
            commands.entity(basketball_entity).despawn_recursive();
        }
    }
}

pub fn on_throw_basketball(
    mut basketball_query: Query<(&mut Basketball, &mut Velocity)>,
    mut basketball_throw_event: EventReader<ThrowBasketball>,
    mut basketball_thrown_event_sink: EventWriter<BasketballThrown>,
) {
    if basketball_throw_event.is_empty() {
        return;
    }

    let throw = basketball_throw_event.iter().next().unwrap().to_owned();

    for (mut basketball, mut basketball_velocity) in basketball_query.iter_mut() {
        if basketball.thrown {
            continue;
        }
        basketball_velocity.linvel = Vec2::new(
            BASKETBALL_SPEED_X,
            (BASKETBALL_MAX_SPEED_Y - BASKETBALL_MIN_SPEED_Y) * throw.percentage_charged
                + BASKETBALL_MIN_SPEED_Y,
        );
        // 6.76: Average rad/sec of a jump shot.
        basketball_velocity.angvel = -6.76 * 3.0;

        basketball.thrown = true;
        basketball_thrown_event_sink.send(BasketballThrown);
    }
}

pub fn on_basketball_thrown(
    basketball_thrown_event: EventReader<BasketballThrown>,
    mut respawn_timer: ResMut<RespawnBasketballTimer>,
) {
    if basketball_thrown_event.is_empty() {
        return;
    }

    respawn_timer.timer.unpause();
    basketball_thrown_event.clear();
}
