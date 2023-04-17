use bevy::prelude::*;

pub const MAX_CHARGE_DURATION: f32 = 1.0;

#[derive(Component)]
pub struct ChargeBar;

#[derive(Resource)]
pub struct Charge {
    // currently charging the throw
    charging: bool,
    // the strength of the throw
    charge: f32,
}

impl Charge {
    pub fn initialize() -> Self {
        Self {
            charging: false,
            charge: 0.0,
        }
    }

    pub fn percentage_charged(&self) -> f32 {
        self.charge / MAX_CHARGE_DURATION
    }

    pub fn charging(&self) -> bool {
        self.charging
    }

    pub fn tick(&mut self, charge_delta: f32) {
        self.charge = (self.charge + charge_delta).min(MAX_CHARGE_DURATION);
    }

    pub fn reset(&mut self) {
        self.charging = false;
        self.charge = 0.0;
    }

    pub fn start_charging(&mut self) {
        self.reset();
        self.charging = true;
    }
}

pub fn spawn_chargebar(commands: &mut Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(400.0, 10.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 350.0, 0.0),
                ..default()
            },
            ..default()
        },
        Name::new("charge_bar_background"),
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(0.0, 10.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 350.0, 0.0),
                ..default()
            },
            ..default()
        },
        ChargeBar,
    ));
}

pub fn update_charge_bar(
    mut charge_bar_query: Query<&mut Sprite, With<ChargeBar>>,
    charge: Res<Charge>,
) {
    let new_width = charge.percentage_charged() * 400.0;
    let mut charge_bar_sprite = charge_bar_query.single_mut();
    charge_bar_sprite.custom_size = Some(Vec2::new(new_width, 10.0));
}
