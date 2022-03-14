use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "Trivia Game".to_string(),
            ..Default::default()
        })
        .add_startup_system(setup)
        .run();
}

fn setup(mut cmds: Commands) {
    cmds.spawn_bundle(OrthographicCameraBundle::new_2d());
}

