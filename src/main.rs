extern crate bevy;

use bevy::prelude::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Color Shooting".to_string(),
            width: 960.,
            height: 540.,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(1., 1., 1.)))
        .add_plugins(DefaultPlugins)
        .run();
}