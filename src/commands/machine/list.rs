extern crate directories;
use directories::ProjectDirs;
use clap::ArgMatches;
use crate::commands::machine::Machine;
use std::io::{stdout, Result};


pub fn handle(matches: ArgMatches){

    if let Some(proj_dirs) = ProjectDirs::from("com", "bresilla", "dotpilot") {
        proj_dirs.config_dir();
    }
    
    let mut machine = Machine::new();
    
    if matches.get_flag("interactive") {
        if interactive(&mut machine).is_err() {
            eprintln!("Error: Could not start interactive mode");
        }
        return;
    }
    

}

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Terminal, Style, Color},
    widgets::{Block, Borders},
};


fn interactive(machine: &mut Machine) -> Result<()> {    
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    machine.set_name(&String::from("test"));

    loop{
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Block::default().title("Machine").title_style(Style::default().fg(Color::Yellow))
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::DarkGray)),
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