use clap::{Command, Arg, ArgAction};
use std::env;



pub fn cmd() -> Command {

    let _username = env::var("USER").unwrap_or_else(|_| String::from("root"));

    Command::new("machine")
        .about("Add or edit hostnames and ssh")
        .aliases(&["m", "machines", "host", "hosts"])
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
            .about("Add a new machine")
            .aliases(["a", "new"])
            .arg_required_else_help(true)
            .arg(
                Arg::new("name")
                .help("Name of the machine")
                .required(true)
                .value_name("MACHINE_NAME")
            )
            .arg(
                Arg::new("ip")
                .help("IP addresses of the machine")
                .required(true)
                .value_name("IP_ADDRESS")
                .num_args(1..=10)
                .action(ArgAction::Append)
            )
            .arg(
                Arg::new("username")
                .help("Username to use for ssh")
                .short('u')
                .long("username")
                .value_name("USERNAME")
                .default_value("root")
            )
            .arg(
                Arg::new("port")
                .help("Port to use for ssh")
                .short('p')
                .long("port")
                .value_name("PORT")
                .default_value("22")
            )
            .arg(
                Arg::new("key")
                .help("Path to the ssh key")
                .short('k')
                .long("key")
                .value_name("KEY_PATH")
                .default_value("~/.ssh/id_rsa")
            )
        )
}