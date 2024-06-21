use clap::{Command, builder::styling, arg};
use colored::Colorize;

pub fn letter_str(letter: &str) -> String {
    let mut wrapped = "[".bright_green().to_string();
    wrapped.push_str(&letter.bright_green().italic().to_string());
    wrapped.push_str(&"]".bright_green().to_string());
    wrapped.push_str(&"  ".to_string());
    wrapped
}

pub fn command_str(word: &str) -> String {
    word.bright_green().bold().to_string()
}

pub fn descriptin_str(word: &str) -> String {
    word.bright_white().to_string()
}

const ABOUT_STR: &str = "ultimate tool for managing development workflows";

pub fn cli(logo: bool) -> Command {
    let _logo_1: String ="
       ██         ".bright_blue().to_string().to_owned()+"
       ██         ".bright_blue().to_string().as_str()+"
   ██████ ██████  ".bright_blue().to_string().as_str()+"
  ██   ██ ██   ██ ".bright_blue().to_string().as_str()+"
   ██████ ██████  ".bright_blue().to_string().as_str()+"
          ██      ".bright_blue().to_string().as_str()+"
          ██      ".bright_blue().to_string().as_str()+ "\n";
    let logo_str: String = if logo {_logo_1 } else { String::new() };
    let help_str: String = " ".to_string().to_owned()+"
Usage:".bright_blue().bold().to_string().as_str()+"  dp".bright_green().bold().to_string().as_str()+" <COMMAND>".green().to_string().as_str()+"
      ".bright_blue().bold().to_string().as_str()+"  dp".bright_green().bold().to_string().as_str()+" <C>".green().to_string().as_str()+"

Commands:".bright_blue().bold().to_string().as_str()+"
  "+ &command_str("workspace") + "   "+&letter_str("w")+ &descriptin_str("Workspace related commands") + "
  "+ &command_str("project") + "     "+&letter_str("p")+ &descriptin_str("Project and code creation") + "
  "+ &command_str("hostname") + "    "+&letter_str("h")+ &descriptin_str("Add or edit hostnames and ssh") + "
  "+ &command_str("template") + "    "+&letter_str("t")+ &descriptin_str("Project template management") + "
  ";

    let styles = styling::Styles::styled()
        .header(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .error(styling::AnsiColor::Red.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Green.on_default());

    Command::new("dp")
        .styles(styles)
        .about(&ABOUT_STR) 
        .author("bresilla <trim.bresilla@gmail.com>")
        .version("0.1.0")
        .long_about(ABOUT_STR)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .disable_help_subcommand(true)
        .override_help(logo_str + &help_str)
        // .subcommand(action::cmd())
        // .subcommand(topic::cmd())
        // .subcommand(service::cmd())
        // .subcommand(param::cmd())
        // .subcommand(node::cmd())
        // .subcommand(interface::cmd())
        // .subcommand(frame::cmd())
        // .subcommand(run::cmd())
        // .subcommand(launch::cmd())
        // .subcommand(work::cmd())
        // .subcommand(bag::cmd())
        // .subcommand(daemon::cmd())
        // .subcommand(middleware::cmd())
        .arg(arg!(--about "about"))
}