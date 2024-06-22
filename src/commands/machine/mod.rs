use clap::ArgMatches;

mod add;


struct Host {
    ip: String,
    port: String,
}
struct Machine {
    name: String,
    username: String,
    hosts: Vec<Host>,
    key: String,
}

impl Machine {
    fn new() -> Machine {
        Machine {
            name: String::new(),
            username: String::new(),
            hosts: Vec::new(),
            key: String::new(),
        }
    }

    fn set_name(&mut self, name: &String) {
        self.name = name.clone();
    }

    fn set_username(&mut self, username: &String) {
        self.username = username.clone();
    }
    
    fn add_host(&mut self, ip : &String, port: &String) {
        self.hosts.push(Host {
            ip: ip.clone(),
            port: port.clone(),
        });
    }
    
    fn set_key(&mut self, key: &String) {
        self.key = key.clone();
    }
}

impl std::fmt::Display for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Machine: {}\nUsername: {}\nHosts: {}\nKey: {}", self.name, self.username, self.hosts.iter().map(|h| h.to_string()).collect::<Vec<String>>().join(", "), self.key)
    }
}

pub fn handle(matches: ArgMatches){
    match matches.subcommand() {
        Some(("add", args)) => {
            add::handle(args.clone());
        }
        _ => unreachable!("UNREACHABLE"),
    }
}