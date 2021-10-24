use std::cmp::max;
use bevy::prelude::*;

use crate::game::get_texture_path;

const PLAYER_TEXTURE_NAME : &str = "player.png";

pub struct Player {
    life: usize,
}

impl Player {
    pub fn new(life: usize) -> Player {
        Player {
            life
        }
    }
    
    pub fn damage(&mut self, value: usize) {
        self.life = max(0, self.life - value);
    }
}

// create player entity with components
pub fn create_player(life: usize, position: Vec3, mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let texture_path = get_texture_path(PLAYER_TEXTURE_NAME);
    let texture = asset_server.load(texture_path.as_str());

    let rotation = Quat::from_axis_angle(Vec3::Y, 0.);
    let scale = Vec3::splat(1.);

    let mut player = commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(texture.into()),
            sprite: Sprite::new(Vec2::new(90., 90.)),
            transform: Transform {
                translation: position,
                rotation: rotation,
                scale: scale,
            },
            ..Default::default()
        });
    player.insert(Player::new(life));
}

