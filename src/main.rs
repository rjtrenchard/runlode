use bevy::prelude::*;

fn main() {
    App::new()
    .add_systems(Startup, add_person)
    .add_systems(Update, (greet_people, hello_world))
    .run();
}

fn hello_world() {
    println!("Hello world!");
}

fn add_person(mut commands: Commands) {
    commands.spawn( (Person, Name("Sir Lancelot".to_string())));
    commands.spawn( (Person, Name("King Arthur".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);