use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use std::env;
use hostname;
use directories::BaseDirs;
use std::path::PathBuf;

mod add;
mod list;

#[derive(Debug, Deserialize, Serialize)]
struct Host {
    ip: String,
    port: String,
    iface: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Machine {
    name: String,
    username: String,
    key: Option<String>,
    hosts: Vec<Host>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Machines {
    machines: Vec<Machine>,
}


impl Machine {
    fn new() -> Machine {
        Machine {
            name: String::new(),
            username: String::new(),
            key: None,
            hosts: Vec::new(),
        }
    }

    fn set_name(&mut self, name: &String) {
        self.name = name.clone();
    }

    fn set_username(&mut self, username: &String) {
        self.username = username.clone();
    }
    
    fn add_host(&mut self, ip : &String, port: &String, iface: &String) {
        self.hosts.push(Host {
            ip: ip.clone(),
            port: port.clone(),
            iface: iface.clone(),
        });
    }
    
    fn set_key(&mut self, key: &String) {
        self.key = Some(key.clone());
    }
}

impl std::fmt::Display for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.ip, self.port, self.iface)
    }
}

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Machine: {}\nUsername: {}\nHosts: {}\nKey: {}", self.name, self.username, self.hosts.iter().map(|h| h.to_string()).collect::<Vec<String>>().join(", "), self.key.as_ref().unwrap_or(&String::from("None")))
    }
}

// impl Machines {
//     fn new() -> Machines {
//         Machines {
//             machines: Vec::new(),
//         }
//     }
// }

impl std::fmt::Display for Machines {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.machines.iter().map(|m| m.to_string()).collect::<Vec<String>>().join("\n"))
    }
}

pub fn handle(matches: ArgMatches){

    let mut machines_file: PathBuf = PathBuf::new();
    if let Some(proj_dirs)  = BaseDirs::new() {
        if !proj_dirs.data_local_dir().exists() {
            std::fs::create_dir_all(proj_dirs.data_local_dir()).expect("Could not create config directory");
        }
        machines_file = proj_dirs.data_local_dir().join("machines.toml");
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
    }

    if machines_file == PathBuf::new() {
        eprintln!("Error: Could not create config file");
        return;
    }

    match matches.subcommand() {
        Some(("add", args)) => {
            add::handle(args.clone(), machines_file);
        }
        Some(("list", args)) => {
            list::handle(args.clone(), machines_file);
        }
        _ => unreachable!("UNREACHABLE"),
    }
}