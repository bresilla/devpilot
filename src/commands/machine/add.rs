extern crate directories;
use directories::ProjectDirs;
use clap::ArgMatches;
use crate::commands::machine::Machine ;
use std::io::Result;
use serde::Deserialize;
use figment::{providers::{Format, Toml}, Figment};
use std::fs;


#[derive(Debug, Deserialize)]
struct Machines {
    machines: Vec<Machine>,
}

pub fn handle(matches: ArgMatches){
    let settings: Machines;
    if let Some(proj_dirs) = ProjectDirs::from("com", "bresilla", "devpilot") {
        if !proj_dirs.config_dir().exists() {
            std::fs::create_dir_all(proj_dirs.config_dir()).expect("Could not create config directory");
        }

        let toml_content = fs::read_to_string(proj_dirs.config_dir().join("machines.toml")).expect("Could not read config file");
        settings = toml::from_str(&toml_content).expect("Failed to parse TOML file");

        println!("{:?}", settings);

        // let config_file = proj_dirs.config_dir().join("machines.toml");
        // if !config_file.exists() {
        //     std::fs::write(&config_file, "").expect("Could not create config file");
        // }
        // let settings: Machines = Figment::new()
        //     .merge(Toml::file(config_file).nested())
        //     .extract().unwrap();
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

use inquire::{Text, validator::Validation};

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