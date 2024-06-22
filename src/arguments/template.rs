use clap::{Command, Arg, ArgAction};

pub fn cmd() -> Command {
    Command::new("template")
        .about("Project template management")
        .aliases(&["t", "templates", "temp"])
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
            .about("Add a new template")
            .aliases(["a", "new"])
            .arg_required_else_help(true)
            .arg(
                Arg::new("name")
                .help("Name of the template")
                .required(true)
                .value_name("TEMPLATE_NAME")
            )
            .arg(
                Arg::new("description")
                .short('d')
                .long("description")
                .aliases(&["description", "desc"])
                .help("Description of the template")
                .required(true)
                .value_name("DESCRIPTION")
            )
            .arg(
                Arg::new("path")
                .short('p')
                .long("path")
                .aliases(&["path", "p"])
                .help("Path to the template")
                .required(true)
                .value_name("TEMPLATE_PATH")
            )
        )
}