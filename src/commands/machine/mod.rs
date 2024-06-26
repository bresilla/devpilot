use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use tabled::Tabled;
use std::env;
use hostname;
use directories::BaseDirs;
use std::path::PathBuf;
use std::iter;
use std::borrow::Cow;

mod add;
mod list;

#[derive(Debug, Deserialize, Serialize)]
struct Host {
    ip: String,
    port: String,
    iface: String,
}

impl std::fmt::Display for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.ip, self.port, self.iface)
    }
}

impl Clone for Host {
    fn clone(&self) -> Self {
        Host {
            ip: self.ip.clone(),
            port: self.port.clone(),
            iface: self.iface.clone(),
        }
    }
}

impl Tabled for Host {
    const LENGTH: usize = 42;
    fn headers() -> Vec<Cow<'static, str>> {
        vec!["IP".into(), "Port".into(), "Interface".into()]
    }

    fn fields(&self) -> Vec<Cow<'_, str>> { 
        vec![self.ip.clone().into(), self.port.clone().into(), self.iface.clone().into()]
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Machine {
    name: String,
    username: String,
    key: Option<String>,
    hosts: Vec<Host>,
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

impl Clone for Machine {
    fn clone(&self) -> Self {
        Machine {
            name: self.name.clone(),
            username: self.username.clone(),
            key: self.key.clone(),
            hosts: self.hosts.clone(),
        }
    }
}

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Machine: {}\nUsername: {}\nHosts: {}\nKey: {}", self.name, self.username, self.hosts.iter().map(|h| h.to_string()).collect::<Vec<String>>().join(", "), self.key.as_ref().unwrap_or(&String::from("None")))
    }
}

impl Tabled for Machine {
    const LENGTH: usize = 42;
    fn headers() -> Vec<Cow<'static, str>> {
        vec!["Name".into(), "Username".into(), "Hosts".into(), "Key".into()]
    }

    fn fields(&self) -> Vec<Cow<'_, str>> { 
        vec![self.name.clone().into(), self.username.clone().into(), self.hosts.iter().map(|h| h.to_string()).collect::<Vec<String>>().join(", ").into(), self.key.as_ref().unwrap_or(&String::from("None")).clone().into()]
    }
}



#[derive(Debug, Deserialize, Serialize)]
struct Machines {
    machines: Vec<Machine>,
}

impl Machines {
    fn _new() -> Machines {
        Machines {
            machines: Vec::new(),
        }
    }
}

// impl iter::Iterator for Machines {
//     type Item = Machine;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.machines.pop()
//     }
// }

impl iter::IntoIterator for Machines {
    type Item = Machine;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.machines.into_iter()
    }
}

impl Clone for Machines {
    fn clone(&self) -> Self {
        Machines {
            machines: self.machines.clone(),
        }
    }
}

impl std::fmt::Display for Machines {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.machines.iter().map(|m| m.to_string()).collect::<Vec<String>>().join("\n"))
    }
}

impl Tabled for Machines {
    const LENGTH: usize = 42;
    fn headers() -> Vec<Cow<'static, str>> {
        vec!["Name".into(), "Username".into(), "Hosts".into(), "Key".into()]
    }

    fn fields(&self) -> Vec<Cow<'_, str>> { 
        vec![self.machines.iter().map(|m| m.name.clone()).collect::<Vec<String>>().join(", ").into(), 
        self.machines.iter().map(|m| m.username.clone()).collect::<Vec<String>>().join(", ").into(), 
        self.machines.iter().map(|m| m.hosts.iter().map(|h| h.to_string()).collect::<Vec<String>>().join(", ")).collect::<Vec<String>>().join(", ").into(), 
        self.machines.iter().map(|m| m.key.as_ref().unwrap_or(&String::from("None")).clone()).collect::<Vec<String>>().join(", ").into()]
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
                        name: String::from(hostn.to_owned()),
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