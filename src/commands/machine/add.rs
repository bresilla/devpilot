extern crate directories;
use directories::BaseDirs;
use clap::ArgMatches;
use crate::commands::machine::{Host, Machine, Machines};
use std::io::Result;
use figment::{providers::{Format, Toml, Env}, Figment};
use toml;
use std::env;
use hostname;


// #[derive(Debug, Deserialize, Serialize)]
// struct Machines {
//     machines: Vec<Machine>,
// }

pub fn handle(matches: ArgMatches){
    if let Some(proj_dirs)  = BaseDirs::new() {
        if !proj_dirs.data_local_dir().exists() {
            std::fs::create_dir_all(proj_dirs.data_local_dir()).expect("Could not create config directory");
        }

        let machines_file = proj_dirs.data_local_dir().join("machines.toml");
        if !machines_file.exists() || machines_file.metadata().unwrap().len() == 0 {
            std::fs::write(&machines_file, "").expect("Could not create config file");
            let hostn = hostname::get().unwrap().into_string().unwrap();
            let usrn = env::var("USER").unwrap_or_else(|_| String::from("root"));
            let machines = Machines {
                machines: vec![
                    Machine {
                        name: String::from(hostn.to_owned() + ".local"),
                        username: String::from(usrn),
                        hosts: vec![Host {
                            ip: String::from("127.0.0.1"),
                            port: String::from("22"),
                            iface: String::from("local"),
                            }],
                        key: None,
                        }
                    ]
                };


            let toml = toml::to_string(&machines).unwrap();
            std::fs::write(&machines_file, toml).expect("Could not write to config file");
        }
        let machines: Machines = Figment::new()
            .merge(Toml::file(machines_file))
            .extract().unwrap();
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