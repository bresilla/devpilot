extern crate directories;
use clap::ArgMatches;
use crate::commands::{machine::{Machine, Machines}, TerminalSize};
use std::io::Result;
use figment::{providers::{Format, Toml}, Figment};
use inquire::{Text, validator::Validation};
use toml;
use std::env;
extern crate interfaces;
use interfaces::Interface;
use std::path::PathBuf;

pub fn handle(matches: ArgMatches, machines_file: PathBuf, _terminal_size: TerminalSize){
    let mut machines: Machines = Figment::new()
        .merge(Toml::file(&machines_file))
        .extract().unwrap();
        
    let mut machine = Machine::new();
    if matches.get_flag("interactive") {
        if interactive(&mut machine).is_err() {
            eprintln!("Error: Could not start interactive mode");
        }
    } else {
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
    }

    //check if machine already exists
    if machines.machines.iter().any(|m| m.name == machine.name) {
        eprintln!("Error: Machine with name {} already exists", machine.name);
        return;
    }
    
    machines.machines.push(machine);
    let toml = toml::to_string_pretty(&machines).unwrap();
    std::fs::write(&machines_file, toml).expect("Could not write to config file");        


}


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

    let uname= env::var("USER").unwrap_or_else(|_| String::from("root"));
    let username = Text::new("Username")
        .with_validator(|input: &str| {
            if input.is_empty() {
                Ok(Validation::Invalid("Username cannot be empty".into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .with_default(&uname)
        .prompt();

    if username.is_ok() {
        machine.set_username(&username.unwrap());
    }

    let host = Text::new("Host")
        .with_validator(|input: &str| {
            let ifs = Interface::get_all().expect("could not get interfaces");
            let parts: Vec<&str> = input.split(":").collect();
            if parts.len() < 3 {
                return Ok(Validation::Invalid("Invalid host format, should be ip:port:iface".into()));
            }
            if parts[0].parse::<std::net::IpAddr>().is_err() {
                return Ok(Validation::Invalid("Invalid ip format".into()));
            }
            if parts.len() == 3 {
                if parts[1].parse::<u16>().is_err() {
                    return Ok(Validation::Invalid("Invalid port format".into()));
                }
                if !ifs.iter().any(|i| i.name == parts[2]) {
                    return Ok(Validation::Invalid("Invalid iface name".into()));
                }
            } else if parts.len() == 2 {
                if !ifs.iter().any(|i| i.name == parts[1]) {
                    return Ok(Validation::Invalid("Invalid iface name".into()));
                }
            }
            Ok(Validation::Valid)
        })
        .prompt();

    if host.is_ok() {
        let host = host.unwrap();
        machine.add_host(&host.split(":").collect::<Vec<&str>>()[0].to_string(), &host.split(":").collect::<Vec<&str>>()[1].to_string(), &host.split(":").collect::<Vec<&str>>()[2].to_string());
    }

    println!("{}", machine);

    Ok(())

}