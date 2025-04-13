use bevy::prelude::*;

mod fps_counter;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            fps_counter::FpsCounterPlugin,
        ))
        .run();
}
//region first app
//region system
fn hello_world() {
    println!("hello world!");
}
//endregion

//region component
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}
#[derive(Resource)]
struct GreetTimer(Timer);
//endregion
//region query
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}
fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break;
        }
    }
}
//endregion
//region Plugin
pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, ((update_people, greet_people).chain()));
    }
}
//endregion
//endregion
