mod player;
mod bullet;

const RESOURCE_PATH: &str = "texture/";

pub fn get_texture_path(name: &str) -> String {
    format!("{}{}", RESOURCE_PATH, name)
}