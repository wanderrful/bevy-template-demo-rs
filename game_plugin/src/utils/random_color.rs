use rand::thread_rng;
use rand::seq::SliceRandom;

use bevy::render::color::Color;

pub fn get_random_color() -> Color {
    *vec![
        Color::BLUE,
        Color::GREEN,
        Color::RED,
        Color::PINK,
        Color::PURPLE
    ].choose(&mut thread_rng()).unwrap()
}