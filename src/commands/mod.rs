use clap::ArgMatches;

pub fn handle(matches: ArgMatches) {
    match matches.subcommand() {
        // Some(("action", submatch)) => {
        //     action::handle(submatch.clone());
        // }
        // Some(("topic", submatch)) => {
        //     topic::handle(submatch.clone());
        // }
        // Some(("service", submatch)) => {
        //     service::handle(submatch.clone());
        // }
        // Some(("param", submatch)) => {
        //     param::handle(submatch.clone());
        // }
        // Some(("node", submatch)) => {
        //     node::handle(submatch.clone());
        // }
        // Some(("interface", submatch)) => {
        //     interface::handle(submatch.clone());
        // }
        _ => unreachable!("UNREACHABLE"),
    };
}