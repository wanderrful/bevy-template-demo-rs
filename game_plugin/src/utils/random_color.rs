use rand::thread_rng;
use rand::seq::SliceRandom;

use bevy::render::color::Color;

pub fn get_random_color() -> Color {
    *vec![
        Color::BLUE,
        Color::GREEN,
        Color::RED,
        Color::PINK,
        Color::PURPLE,
        Color::YELLOW,
        Color::YELLOW_GREEN,
        Color::BEIGE,
        Color::INDIGO,
        Color::MAROON,
        Color::SEA_GREEN,
        Color::ORANGE,
        Color::ORANGE_RED
    ].choose(&mut thread_rng()).unwrap()
}