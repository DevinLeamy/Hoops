use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch, window::PresentMode};
use bevy_asset_loader::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

mod ball;
mod chargebar;
mod hoop;
mod utils;
mod wasm;

use ball::*;
use chargebar::*;
use hoop::*;
use wasm::*;

// config
const WINDOW_WIDTH: f32 = 750.0;
const WINDOW_HEIGHT: f32 = 800.0;

const TIME_STEP: f32 = 1.0 / 60.0;
const GRAVITY: f32 = 40.0;

const GROUND_LEVEL: f32 = -400.0;

// events
struct BasketEvent;

pub struct ThrowBasketball {
    pub percentage_charged: f32,
}
pub struct BasketballThrown;

#[derive(Resource)]
pub struct RespawnBasketballTimer {
    timer: Stopwatch,
}

#[derive(Resource)]
pub struct GlobalState {
    pub points: u32,
}

impl Default for GlobalState {
    fn default() -> Self {
        GlobalState { points: 0 }
    }
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "basketball.png")]
    basketball: Handle<Image>,
    #[asset(path = "standing_hoop.png")]
    hoop: Handle<Image>,
}

// states
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum GameState {
    AssetLoading,
    Running,
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let default_plugins = DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Hoops".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            resizable: false,
            monitor: MonitorSelection::Index(0),
            present_mode: PresentMode::AutoVsync,
            #[cfg(target_arch = "wasm32")]
            canvas: Some(String::from(".bevy")),
            ..default()
        },
        ..default()
    });

    // start the stopwatch in the paused state
    let mut stopwatch = Stopwatch::new();
    stopwatch.pause();

    App::new()
        // plugins
        .add_plugins(default_plugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        // initial state
        .add_loopless_state(GameState::AssetLoading)
        // bevy_asset_loader
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Running)
                .with_collection::<GameAssets>(),
        )
        // set fixed timestep
        .add_fixed_timestep(Duration::from_secs_f32(TIME_STEP), "time_step")
        // resources
        .insert_resource(ClearColor(Color::rgb_u8(252, 252, 252)))
        .insert_resource(Charge::initialize())
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(GlobalState::default())
        .insert_resource(RespawnBasketballTimer { timer: stopwatch })
        // events
        .add_event::<BasketballThrown>()
        .add_event::<ThrowBasketball>()
        .add_event::<BasketEvent>()
        // systems
        .add_system(bevy::window::close_on_esc)
        .add_enter_system_set(
            GameState::Running,
            SystemSet::new()
                .with_system(setup_game)
                .with_system(setup_camera),
        )
        .add_system(handle_input.run_in_state(GameState::Running).label("input"))
        .add_system(
            on_throw_basketball
                .run_in_state(GameState::Running)
                .label("throw")
                .after("input"),
        )
        .add_system(
            on_basketball_thrown
                .run_in_state(GameState::Running)
                .label("thrown")
                .after("throw"),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Running)
                .with_system(detect_point)
                .with_system(on_basket)
                .with_system(despawn_basketballs)
                .into(),
        )
        .add_fixed_timestep_system_set(
            "time_step",
            0,
            ConditionSet::new()
                .run_in_state(GameState::Running)
                .with_system(update_timers)
                .with_system(update_charge_bar)
                .with_system(respawn_basketball)
                .into(),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update_timers(mut respawn_timer: ResMut<RespawnBasketballTimer>) {
    respawn_timer.timer.tick(Duration::from_secs_f32(TIME_STEP));
}

fn spawn_ground(commands: &mut Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::hex("B68E65").unwrap(),
                custom_size: Some(Vec2::new(WINDOW_WIDTH, 20.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, GROUND_LEVEL, 0.0),
                ..default()
            },
            ..default()
        },
        Name::new("charge_bar_background"),
        Collider::cuboid(WINDOW_WIDTH, 10.0),
    ));
}

fn setup_game(
    mut commands: Commands,
    assets: Res<GameAssets>,
    _texture_atlases: Res<Assets<TextureAtlas>>,
) {
    spawn_basketball(&mut commands, &assets);
    spawn_ground(&mut commands);
    spawn_hoop(&mut commands, &assets);
    spawn_chargebar(&mut commands);
}

fn handle_input(
    time: Res<Time>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut charge: ResMut<Charge>,
    mut throw_basketball_event_sink: EventWriter<ThrowBasketball>,
) {
    let start_charging = keyboard_input.just_pressed(KeyCode::Space);
    let continue_charging = keyboard_input.pressed(KeyCode::Space);
    let release = keyboard_input.just_released(KeyCode::Space);

    if start_charging {
        charge.start_charging();
    } else if continue_charging && charge.charging() {
        charge.tick(time.delta_seconds());
    } else if release {
        throw_basketball_event_sink.send(ThrowBasketball {
            percentage_charged: charge.percentage_charged(),
        });

        charge.reset();
        keyboard_input.clear_just_released(KeyCode::Space);
    }
}

fn on_basket(basket_event: EventReader<BasketEvent>, mut global_state: ResMut<GlobalState>) {
    if basket_event.is_empty() {
        return;
    }

    global_state.points += 1;
    println!("Total points: {}", global_state.points);
    basket_event.clear();
    js::on_point_awared();
}

fn detect_point(
    mut hoop: Query<&Hoop>,
    mut hoop_events: EventReader<CollisionEvent>,
    mut last_top_contact: Local<Option<u32>>,
    mut basket_event_sink: EventWriter<BasketEvent>,
) {
    if hoop_events.is_empty() {
        return;
    }

    let hoop = hoop.single_mut();

    let mut top_sensor_contact: Option<Entity> = None;
    let mut bottom_sensor_contact: Option<Entity> = None;

    for event in hoop_events.iter() {
        if let CollisionEvent::Started(a, b, _) = event {
            if a == &hoop.top_sensor_id || b == &hoop.top_sensor_id {
                top_sensor_contact = Some(if a == &hoop.top_sensor_id { *b } else { *a });
            } else if a == &hoop.bottom_sensor_id || b == &hoop.bottom_sensor_id {
                bottom_sensor_contact = Some(if a == &hoop.bottom_sensor_id { *b } else { *a });
            }
        }
    }

    if let Some(id) = bottom_sensor_contact {
        if Some(id.index()) == *last_top_contact {
            *last_top_contact = None;
            basket_event_sink.send(BasketEvent);
        }
    } else if let Some(id) = top_sensor_contact {
        *last_top_contact = Some(id.index());
    }
}
