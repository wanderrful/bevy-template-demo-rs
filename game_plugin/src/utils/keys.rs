use bevy::input::keyboard::KeyCode;

/// Helper: apply KeyCode to user input string
pub fn get_adjusted_user_input(old_value: String, key: &KeyCode) -> String {
    match key {
        KeyCode::Back => String::from(&old_value[..old_value.len()-1]),
        KeyCode::A
        | KeyCode::B
        | KeyCode::C
        | KeyCode::D
        | KeyCode::E
        | KeyCode::F
        | KeyCode::G
        | KeyCode::H
        | KeyCode::I
        | KeyCode::J
        | KeyCode::K
        | KeyCode::L
        | KeyCode::M
        | KeyCode::N
        | KeyCode::O
        | KeyCode::P
        | KeyCode::Q
        | KeyCode::R
        | KeyCode::S
        | KeyCode::T
        | KeyCode::U
        | KeyCode::V
        | KeyCode::W
        | KeyCode::X
        | KeyCode::Y
        | KeyCode::Z => old_value + format!("{:?}", key).as_str(),
        default => old_value
    }
}