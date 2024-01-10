use crate::player::components::{BottomWall, LeftWall, PlayerShip, RightWall, TopWall};
use crate::shots::components::*;
use crate::sprite_loader::mapper::XMLSpriteSheetLoader;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const WEAPON_SPRITE_NAME: &str = "laserGreen02.png";

const DEFAULT_WEAPON: Weapon = Weapon {
    damage: 25.0,
    speed: 1500.0,
};
const FIRE_DISTANCE_FROM_PLAYER: f32 = 25.0;

pub fn player_fire_weapon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    sprite_loader: Res<XMLSpriteSheetLoader>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_ship_query: Query<(&Transform, &mut WeaponFireTimer), With<PlayerShip>>,
    time: Res<Time>,
) {
    if let Ok((transform, mut weapon_fire_timer)) = player_ship_query.get_single_mut() {
        weapon_fire_timer.timer.tick(time.delta());
        if keyboard_input.pressed(KeyCode::Space) || keyboard_input.just_pressed(KeyCode::Space) {
            if weapon_fire_timer.timer.elapsed() >= weapon_fire_timer.fire_delay {
                weapon_fire_timer.timer.reset();
                let rotation = transform.rotation.to_scaled_axis();
                let linvel = Vec2::from_angle(rotation.z).rotate(Vec2::Y) * DEFAULT_WEAPON.speed;
                spawn_weapon_at_position(
                    &mut commands,
                    &asset_server,
                    &mut texture_atlases,
                    &sprite_loader,
                    WEAPON_SPRITE_NAME,
                    DEFAULT_WEAPON,
                    transform,
                    Velocity {
                        linvel,
                        angvel: 0.0,
                    },
                );
            }
        }
    }
}
fn spawn_weapon_at_position(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    sprite_loader: &Res<XMLSpriteSheetLoader>,
    sprite_name: &str,
    weapon: Weapon,
    ship_transform: &Transform,
    force: Velocity,
) {
    let texture_handle = asset_server.load(&sprite_loader.file);
    let sprite = sprite_loader.get_sprite(sprite_name.to_string()).unwrap();
    let ship_offset = (sprite.x as f32, sprite.y as f32);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(sprite.width as f32, sprite.height as f32),
        1,
        1,
        None,
        Some(Vec2::new(ship_offset.0, ship_offset.1)),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let scale = 1.0;
    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0),
                ..default()
            },
            weapon,
        ))
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(0.0))
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled())
        .insert(Collider::ball(sprite.h_radius()))
        .insert(ColliderMassProperties::Density(0.001))
        .insert(Sensor)
        .insert(middle_shot_from_transform(ship_transform).with_scale(Vec3::splat(scale)))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(force);
}

fn middle_shot_from_transform(transform: &Transform) -> Transform {
    let shot_vec = Vec3::new(0.0, FIRE_DISTANCE_FROM_PLAYER, 0.0);
    shot_from_transform(shot_vec, transform)
}

fn shot_from_transform(shot_vec: Vec3, transform: &Transform) -> Transform {
    let angle = transform.rotation.to_scaled_axis().z;
    let angle_cos = angle.cos();
    let angle_sin = angle.sin();
    let new_x = angle_cos * shot_vec.x - angle_sin * shot_vec.y + transform.translation.x;
    let new_y = angle_sin * shot_vec.x + angle_cos * shot_vec.y + transform.translation.y;

    Transform {
        translation: Vec3::new(new_x, new_y, 0.0),
        rotation: transform.rotation.clone(),
        scale: Default::default(),
    }
}

pub fn handle_shot_intersections_with_wall(
    mut commands: Commands,
    shot_query: Query<(Entity, &Transform), With<Weapon>>,
    left_wall_query: Query<&Transform, With<LeftWall>>,
    right_wall_query: Query<&Transform, With<RightWall>>,
    top_wall_query: Query<&Transform, With<TopWall>>,
    bottom_wall_query: Query<&Transform, With<BottomWall>>,
) {
    for (shot_entity, shot_transform) in shot_query.iter() {
        let mut check_and_despawn = |check: bool| {
            if check {
                commands.entity(shot_entity).despawn();
            }
            check
        };
        if let Ok(wall_transform) = left_wall_query.get_single() {
            if check_and_despawn(shot_transform.translation.x < wall_transform.translation.x) {
                break;
            }
        }
        if let Ok(wall_transform) = right_wall_query.get_single() {
            if check_and_despawn(shot_transform.translation.x > wall_transform.translation.x) {
                break;
            }
        }
        if let Ok(wall_transform) = top_wall_query.get_single() {
            if check_and_despawn(shot_transform.translation.y > wall_transform.translation.y) {
                break;
            }
        }
        if let Ok(wall_transform) = bottom_wall_query.get_single() {
            if check_and_despawn(shot_transform.translation.y < wall_transform.translation.y) {
                break;
            }
        }
    }
}
