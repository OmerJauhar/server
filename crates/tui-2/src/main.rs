use std::{error::Error, io, vec};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
// use itertools::Itertools;
use ratatui::{layout::Constraint::*, prelude::*, widgets::*};
// use tui::widgets::Paragraph;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}
fn ui<B: Backend>(frame: &mut Frame<B>) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
        Percentage(100)
        // Percentage(90)
        ])
        .split(frame.size());
    frame.render_widget(Block::default().title(" OJ's Server ").title_alignment(Alignment::Center).borders(ratatui::widgets::Borders::ALL).border_type(BorderType::Rounded), main_layout[0]);
    // frame.render_widget(Block::default().borders(ratatui::widgets::Borders::ALL).border_type(BorderType::Rounded), main_layout[2])
    let center_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Percentage(2),
        Percentage(8),
        Percentage(80),
        Percentage(10)
    ])
    .split(main_layout[0]);

    frame.render_widget(Block::default().borders(ratatui::widgets::Borders::ALL), center_layout[1]);

    let search_layer = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(
        vec![
            Percentage(80),
            Percentage(20)
        ]
    )
    .split(center_layout[1]);

    frame.render_widget(Block::default().borders(ratatui::widgets::Borders::ALL), search_layer[1]);

    let rand_value : u64 = 23 ; 

    let main_layer = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Percentage(20),
        Percentage(60),
        Percentage(20)
    ])
    .split(center_layout[2]);

    let directories = [ListItem::new("Item 1"), ListItem::new("Item 2"), ListItem::new("Item 3")];
    let directories_list = List::new(directories)
    .block(Block::default().title("Directories").borders(Borders::ALL))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    .highlight_symbol(">>");


    let files = [ListItem::new("meow")];
    let files_list = List::new(files)
    .block(Block::default().title("Files").borders(Borders::ALL))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    .highlight_symbol(">>");

    frame.render_widget(directories_list, main_layer[0]);
    frame.render_widget(files_list, main_layer[1]);
    frame.render_widget(Block::default().borders(ratatui::widgets::Borders::ALL), main_layer[2]);

    let about_layer = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Percentage(80), //about layer ka 80 us hum pate dei 
        Percentage(20)
    ])
    .split(center_layout[3]);

    frame.render_widget(Gauge::default()
    .block(Block::default().title("Progress").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Blue))
        .ratio(0.50)
        // .label(label)
        .use_unicode(true), about_layer[0]);

    frame.render_widget(ratatui::widgets::Paragraph::new("h: help    a: about\n     q: Exit    ctrl+c ")
    .block(Block::default().borders(Borders::ALL))
    .style(Style::default().fg(Color::Gray).bg(Color::Black))
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true }),about_layer[1]);
    
}