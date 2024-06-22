use clap::{Command, Arg, ArgAction};

pub fn cmd() -> Command {
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
                Arg::new("hostname")
                .short('h')
                .long("hostname")
                .aliases(&["hostname", "host"])
                .help("Hostname of the machine")
                .required(true)
                .value_name("HOSTNAME")
            )
            .arg(
                Arg::new("ssh")
                .short('s')
                .long("ssh")
                .aliases(&["ssh", "port"])
                .help("SSH port of the machine")
                .required(true)
                .value_name("SSH_PORT")
            )
        )
}