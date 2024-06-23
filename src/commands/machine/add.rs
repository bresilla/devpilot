extern crate directories;
use directories::ProjectDirs;
use clap::ArgMatches;
use crate::commands::machine::Machine;
use std::io::{stdout, Result};


pub fn handle(matches: ArgMatches){

    if let Some(proj_dirs) = ProjectDirs::from("com", "bresilla", "dotpilot") {
        proj_dirs.config_dir();
    }
    
    let mut machine = Machine::new();
    
    if matches.get_flag("interactive") {
        if interactive(&mut machine).is_err() {
            eprintln!("Error: Could not start interactive mode");
        }
        return;
    }
    
    let name = matches.get_one::<String>("name").unwrap();
    machine.set_name(name);

    let host = matches.get_many::<(String, String, String)>("host");
    for i in host.unwrap() {
        machine.add_host(&i.0, &i.1, &i.2);
    }

    let username = matches.get_one::<String>("username").unwrap();
    machine.set_username(username);

    let key = matches.get_one::<String>("key").unwrap();
    machine.set_key(key);
    
    println!("{}", machine);

}

use inquire::{Text, validator::{StringValidator, Validation}};

fn interactive(machine: &mut Machine) -> Result<()> {

    let name = Text::new("Name")
        .with_validator(|input: &str| {
            if input.is_empty() {
                Ok(Validation::Invalid("Name cannot be empty".into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt();

    if name.is_ok() {
        machine.set_name(&name.unwrap());
    }

    
    println!("{}", machine);

    Ok(())

}