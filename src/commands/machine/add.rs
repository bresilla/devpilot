use clap::ArgMatches;

use crate::commands::machine::Machine;

pub fn handle(matches: ArgMatches){
    
    let mut machine = Machine::new();

    let name = matches.get_one::<String>("name").unwrap();
    machine.set_name(name);

    let ip = matches.get_many::<String>("ip");
    for i in ip.unwrap() {
        machine.add_ip(i);
    }

    let username = matches.get_one::<String>("username").unwrap();
    machine.set_username(username);

    let port = matches.get_one::<String>("port").unwrap();
    machine.set_port(port);

    let key = matches.get_one::<String>("key").unwrap();
    machine.set_key(key);
    
    println!("{}", machine);

}