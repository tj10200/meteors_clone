use bevy::prelude::*;

#[derive(Component)]
pub struct BottomWall;

#[derive(Component)]
pub struct TopWall;

#[derive(Component)]
pub struct LeftWall;

#[derive(Component)]
pub struct RightWall;

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;
