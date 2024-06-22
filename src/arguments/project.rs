use clap::{Command, Arg, ArgAction};

pub fn cmd() -> Command {
    Command::new("project")
        .about("Project related commands")
        .aliases(&["p", "projects", "proj"])
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("info")
            .about("Print information about a project")
            .aliases(["i", "show"])
            .arg_required_else_help(true)
            .arg(
                Arg::new("project_name")
                .help("Name of the project to get info")
                .required(true)
                .value_name("PROJECT_NAME")
            )
        )
        .subcommand(
            Command::new("list")
            .about("List all projects in the workspace")
            .aliases(["l", "ls"])
            .arg_required_else_help(true)
        )   
}