use clap::ArgMatches;

pub fn handle(matches: ArgMatches){
    match matches.subcommand() {
        // Some(("get", args)) => {
        //     get::handle(args.clone());
        // }
        _ => unreachable!("UNREACHABLE"),
    }
}