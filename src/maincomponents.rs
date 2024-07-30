use bevy_ecs::prelude::*;
use glam::Vec3;

use crate::JModelIndex;

#[derive(Component)]
pub struct Position { pub pos: Vec3 }
#[derive(Component)]
pub struct Velocity { pub vel: Vec3 }

#[derive(Component)]
pub struct PlayerCamHere;


#[derive(Component)]
pub struct ModelIndex { pub jmodel: JModelIndex }