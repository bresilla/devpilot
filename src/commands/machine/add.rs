use clap::ArgMatches;
use crate::commands::machine::Machine;

pub fn handle(matches: ArgMatches){
    
    let mut machine = Machine::new();

    if matches.get_flag("interactive") {
        if interactive(machine).is_err() {
            eprintln!("Error: Could not start interactive mode");
        }
        return;
    }

    let name = matches.get_one::<String>("name").unwrap();
    machine.set_name(name);

    let ip = matches.get_many::<String>("ip");
    for i in ip.unwrap() {
        machine.add_ip(i);
    }

    let username = matches.get_one::<String>("username").unwrap();
    machine.set_username(username);

    let port = matches.get_one::<String>("port").unwrap();
    machine.set_port(port);

    let key = matches.get_one::<String>("key").unwrap();
    machine.set_key(key);
    
    println!("{}", machine);

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
use std::io::{stdout, Result};

fn interactive(machine: Machine) -> Result<()> {    
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    loop{
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                // Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                //     .white()
                //     .on_blue(),
                // area,
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