extern crate directories;
use directories::ProjectDirs;
use clap::{ArgMatches, Error};
use crate::commands::machine::{Machines, Machine, Host};
use std::io::{stdout, Result};

use config::{Config, ConfigError, File};


pub fn handle(matches: ArgMatches){

    let settings: Config;

    if let Some(proj_dirs) = ProjectDirs::from("com", "bresilla", "devpilot") {

        //if folder does not exist, create it
        if !proj_dirs.config_dir().exists() {
            std::fs::create_dir_all(proj_dirs.config_dir()).expect("Could not create config directory");
        }

        let config_file = proj_dirs.config_dir().join("machines.toml");
        //if file does not exist, create it
        if !config_file.exists() {
            std::fs::write(&config_file, "").expect("Could not create config file");
        }

        settings = Config::builder()
            .add_source(File::from(config_file))
            .build().expect("Could not build config");

        // let machines: Machines = settings.try_into().expect("Could not parse config");
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
        // .with_default("Super_cool")
        .prompt();

    if name.is_ok() {
        machine.set_name(&name.unwrap());
    }

    
    println!("{}", machine);

    Ok(())

}