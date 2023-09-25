use bevy::prelude::*;

fn main() {
    App::new() // create a new App
        .add_plugins(DefaultPlugins) // add the default plugins
        .add_plugin(PeoplePlugin) // add the PeoplePlugin
        .run(); // run the App
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    // implement the Plugin trait for the PeoplePlugin
    fn build(&self, app: &mut App) {
        // implement the build method for the PeoplePlugin
        app.add_startup_system(setup) // add the setup system to the App
            .add_system(print_names) // add the print_names system to the App
            .add_system(people_with_jobs) // add the people_with_jobs system to the App
            .add_system(people_ready_for_hire) // add the people_ready_for_hire system to the App
            .add_system(person_does_job); // add the person_does_job system to the App
    }
}

pub fn setup(mut commands: Commands) {
    // setup is a function that takes a Commands resource as a parameter

    // This entity doesn't have the `Employed` Component.
    commands.spawn((Person {
        name: "Amethyst".to_string(),
    },));

    commands.spawn((
        Person {
            name: "Elaina Proctor".to_string(),
        },
        Employed {
            job: Job::Programmer,
        },
    ));

    commands.spawn((
        Person {
            name: "Renzo Hume".to_string(),
        },
        Employed { job: Job::Writer },
    ));

    commands.spawn((
        Person {
            name: "Zayna Nieves".to_string(),
        },
        Employed { job: Job::Artist },
    ));
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

pub fn people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    for person in person_query.iter() {
        println!("{} has a job.", person.name);
    }
}

pub fn people_ready_for_hire(person_query: Query<&Person, Without<Employed>>) {
    for person in person_query.iter() {
        println!("{} is ready for hire.", person.name);
    }
}

pub fn person_does_job(person_query: Query<(&Person, &Employed)>) {
    for (person, employed) in person_query.iter() {
        let job_name = match employed.job {
            Job::Programmer => "programmer",
            Job::Writer => "writer",
            Job::Artist => "artist",
        };
        println!("{0} is a {1}.", person.name, job_name);
    }
}

#[derive(Component)] // Component is a trait that indicates that this is a component
pub struct Person {
    // Person is a component
    pub name: String, // name is a field of the Person component
}

#[derive(Component)]
pub struct Employed {
    // Employed is a component
    pub job: Job, // job is a field of the Employed component
}

#[derive(Debug)] // Debug is a trait that indicates that this is a debuggable struct
pub enum Job {
    // Job is a struct
    Programmer, // Programmer is a variant of the Job struct
    Writer,     // Writer is a variant of the Job struct
    Artist,     // Artist is a variant of the Job struct
}
