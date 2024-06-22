use clap::ArgMatches;

mod add;

struct Machine {
    name: String,
    ip: Vec<String>,
    username: String,
    port: String,
    key: String,
}

impl Machine {
    fn new() -> Machine {
        Machine {
            name: String::new(),
            ip: Vec::new(),
            username: String::new(),
            port: String::new(),
            key: String::new(),
        }
    }

    fn set_name(&mut self, name: &String) {
        self.name = name.clone();
    }
    
    fn add_ip(&mut self, ip: &String) {
        self.ip.push(ip.clone());
    }
    
    fn set_username(&mut self, username: &String) {
        self.username = username.clone();
    }

    fn set_port(&mut self, port: &String) {
        self.port = port.clone();
    }

    fn set_key(&mut self, key: &String) {
        self.key = key.clone();
    }
}

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Machine: {}\n", self.name)?;
        write!(f, "IPs: ")?;
        for i in &self.ip {
            write!(f, "{} ", i)?;
        }
        write!(f, "\n")?;
        write!(f, "Username: {}\n", self.username)?;
        write!(f, "Port: {}\n", self.port)?;
        write!(f, "Key: {}\n", self.key)
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