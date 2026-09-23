use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(HelloPlugin)
    .run();
}

fn hello_world() {
    println!("Hello world!");
}

fn add_person(mut commands: Commands) {
    commands.spawn( (Person, Name("Sir Lancelot".to_string())));
    commands.spawn( (Person, Name("King Arthur".to_string())));
    commands.spawn( (Person, Name("Elaina Proctor".to_string())));
}


pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_person);
        app.add_systems(Update, (hello_world, 
        (greet_people, update_people, greet_people).chain()));
    }
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);
