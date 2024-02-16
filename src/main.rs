use bevy::{
    app::App,
    prelude::{Commands, Component, Query, Res, Startup, Update},
    time::{Time, Timer},
    DefaultPlugins,
};

use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, start)
        .add_systems(Update, do_something)
        .run();
}

fn start(mut commands: Commands) {
    commands.spawn((Something(
        "a curious thing".to_string(),
        Timer::new(Duration::from_secs(3), bevy::time::TimerMode::Repeating),
    ),));
}

#[derive(Component)]
struct Something(String, Timer);

fn do_something(time: Res<Time>, mut query: Query<&mut Something>) {
    for mut something in &mut query {
        if something.1.tick(time.delta()).just_finished() {
            println!("doing something with {}", something.0);
        }
    }
}
