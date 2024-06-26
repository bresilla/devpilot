use crate::commands::{machine::Machines, TerminalSize};
extern crate directories;
use directories::ProjectDirs;
use clap::ArgMatches;
use figment::{providers::{Format, Toml}, Figment};
use std::io::{stdout, Result};
use crossterm::{
    event::{self, KeyCode, KeyEventKind}, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand
};
use ratatui::{
    prelude::{CrosstermBackend, Terminal, Style, Color},
    widgets::{Block, Borders},
};
use std::path::PathBuf;

pub fn handle(matches: ArgMatches, machines_file: PathBuf, terminal_size: TerminalSize){
    if let Some(proj_dirs) = ProjectDirs::from("com", "bresilla", "dotpilot") {
        proj_dirs.config_dir();
    }
    let mut machines: Machines = Figment::new()
        .merge(Toml::file(&machines_file))
        .extract().unwrap();

    if matches.get_flag("interactive") {
        if interactive(&mut machines).is_err() {
            eprintln!("Error: Could not start interactive mode");
        }
    } else if matches.get_flag("raw") {
        println!("{}", machines.to_listed());
    } else {
        println!("{}", machines.to_table(terminal_size));
    }
}


fn interactive(_machines: &mut Machines) -> Result<()> {    
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    loop{
        terminal.draw(|frame| {
                let area = frame.size();
                frame.render_widget(
                    Block::default().title("Machines").title_style(Style::default().fg(Color::Black).bg(Color::Yellow))
                    .borders(Borders::NONE)
                    .style(Style::default().bg(Color::Black)),
                    area

                );
        })?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}