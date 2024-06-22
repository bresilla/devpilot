use clap::{Command, Arg, ArgAction};

pub fn cmd() -> Command {
    Command::new("workspace")
        .about("Workspace related commands")
        .aliases(&["w", "workspaces", "ws"])
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("info")
            .about("Print information about a workspace")
            .aliases(["i", "show"])
            .arg_required_else_help(true)
            .arg(
                Arg::new("workspace_name")
                .help("Name of the workspace to get info")
                .required(true)
                .value_name("WORKSPACE_NAME")
            )
        )
        .subcommand(
            Command::new("component")
            .about("add or remove components from a workspace")
            .aliases(["c", "components", "comp"])
            .arg_required_else_help(true)
            .arg(
                Arg::new("workspace_name")
                .help("Name of the workspace to add or remove components")
                .required(true)
                .value_name("WORKSPACE_NAME")
            )
        )
}