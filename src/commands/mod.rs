use clap::ArgMatches;


mod workspace;
mod project;
mod template;
mod machine;

pub fn handle(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("workspace", submatch)) => {
            workspace::handle(submatch.clone());
        }
        Some(("project", submatch)) => {
            project::handle(submatch.clone());
        }
        Some(("template", submatch)) => {
            template::handle(submatch.clone());
        }
        Some(("machine", submatch)) => {
            machine::handle(submatch.clone());
        }
        _ => unreachable!("UNREACHABLE"),
    };
}