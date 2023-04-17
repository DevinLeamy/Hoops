use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// Benimator bevy integration
// https://github.com/jcornaz/benimator/blob/main/examples/bevy.rs
#[derive(Component, DerefMut, Deref)]
pub struct Animation(pub benimator::Animation);

#[derive(Default, Component, Deref, DerefMut)]
pub struct AnimationState(benimator::State);

pub trait PixelSprite {
    fn size() -> Vec2;
    fn pixel_size() -> f32;
    fn pixel_offset(x: u32, y: u32, collider_size: &Vec2) -> Vec2 {
        let mut offset = Self::pixel_dimensions(x, y);
        offset -= Self::size() / 2.0;

        offset + *collider_size / 2.0
    }
    fn pixel_dimensions(width: u32, height: u32) -> Vec2 {
        Vec2::new(width as f32, height as f32) * Self::pixel_size()
    }
}

pub fn create_collider<T: PixelSprite>(
    commands: &mut Commands,
    width: u32,
    height: u32,
    offset_x: u32,
    offset_y: u32,
) -> Entity {
    let collider_size = T::pixel_dimensions(width, height);
    let collider_position = T::pixel_offset(offset_x, offset_y, &collider_size);
    let collider = commands
        .spawn((
            Collider::cuboid(collider_size.x / 2.0, collider_size.y / 2.0),
            RigidBody::Fixed,
            TransformBundle::from(Transform::from_xyz(
                collider_position.x,
                collider_position.y,
                0.0,
            )),
        ))
        .id();
    collider
}
