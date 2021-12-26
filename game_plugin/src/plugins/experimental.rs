use std::collections::HashMap;

use bevy::prelude::*;

use crate::GameState;


pub struct MyExperimentalPlugin;

impl Plugin for MyExperimentalPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(get_game_events())
            .add_event::<MyGameEvent>()
            .add_system_set(SystemSet::on_update(GameState::Playing)
                .with_system(handle_spawn_cube.system())
                .with_system(handle_spawn_sphere.system())
                .with_system(handle_spawn_capsule.system())
                .with_system(handle_inputs.system())
            )
        ;
    }
}

#[derive(PartialEq, Clone)]
pub enum MyEventType {
    SPAWN_CUBE,
    SPAWN_SPHERE,
    SPAWN_CAPSULE
}

pub struct MyGameEvent(MyEventType);


fn handle_spawn_cube(mut spawn_cube: EventReader<MyGameEvent>) {
    spawn_cube.iter().for_each(|it: &MyGameEvent| {
        if MyEventType::SPAWN_CUBE == it.0 {
            info!("Spawning cube...");
        }
    });
}

fn handle_spawn_sphere(mut spawn_sphere: EventReader<MyGameEvent>) {
    spawn_sphere.iter().for_each(|it: &MyGameEvent| {
        if MyEventType::SPAWN_SPHERE == it.0 {
            info!("Spawning sphere...");
        }
    });
}

fn handle_spawn_capsule(mut spawn_capsule: EventReader<MyGameEvent>) {
    spawn_capsule.iter().for_each(|it: &MyGameEvent| {
        if MyEventType::SPAWN_CAPSULE == it.0 {
            info!("Spawning capsule...");
        }
    });
}

/// Event Publisher
fn handle_inputs(
    keys: Res<Input<KeyCode>>,
    bindings: Res<MyInputBindings>,
    mut my_game_event: EventWriter<MyGameEvent>,
) {
    keys.get_just_pressed()
        .filter(|key: &&KeyCode| bindings.contains_key(key))
        .for_each(|&it| {
            let (key_code, event_type) = bindings.get_key_value(&it).unwrap();
            my_game_event.send(MyGameEvent(event_type.clone()));
            // match it {
            //     KeyCode::Key1 => { my_game_event.send(MyGameEvent(MyEventType::SPAWN_CUBE)) },
            //     KeyCode::Key2 => { my_game_event.send(MyGameEvent(MyEventType::SPAWN_SPHERE)) },
            //     KeyCode::Key3 => { my_game_event.send(MyGameEvent(MyEventType::SPAWN_CAPSULE)) },
            //     _default => {}
            // }
        });
}


type MyInputBindings = HashMap<KeyCode, MyEventType>;

pub fn get_game_events() -> MyInputBindings {
    [
        (KeyCode::Key1, MyEventType::SPAWN_CUBE),
        (KeyCode::Key2, MyEventType::SPAWN_SPHERE),
        (KeyCode::Key3, MyEventType::SPAWN_CAPSULE),
    ].iter().cloned().collect()
}
