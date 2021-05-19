mod database;
mod home;
mod event;
mod footer;
mod menu;
mod bird;

use footer::render_footer;
use home::render_home;
use menu::{MenuItem, render_menu};
use bird::{add_random_bird, bird_count, remove_bird, render_birds};
use event::{Event, Events};
use std::io;
use std::io::Stdout;
use termion::{
    event::Key,
    raw::{IntoRawMode, RawTerminal},
};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
    Frame,
    Terminal,
};


fn render_layout(
    active_menu_item: MenuItem,
    bird_list_state: &mut ListState,
    rect: &mut Frame<'_, TermionBackend<RawTerminal<Stdout>>>,
)  {
    let area = rect.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(2),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(area);
    rect.render_widget(render_menu(active_menu_item), chunks[0]);
    match active_menu_item {
        MenuItem::Birds => {
            let birds_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                )
                .split(chunks[1]);
            let selected = bird_list_state.selected()
                .expect("there is always a selected bird");
            let (left, right) = render_birds(selected);
            rect.render_stateful_widget(left, birds_chunks[0], bird_list_state);
            rect.render_widget(right, birds_chunks[1]);
        },
        _ => rect.render_widget(render_home(), chunks[1]),
    }
    rect.render_widget(render_footer(), chunks[2]);
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    let events = Events::new();
    let mut active_menu_item = MenuItem::Home;
    let mut bird_list_state = ListState::default();
    bird_list_state.select(Some(0));

    loop {
        terminal.draw(|rect| {
            render_layout(active_menu_item, &mut bird_list_state, rect);
        })?;
        match events.next()? {
            Event::Input(input) => {
                match input {
                    Key::Char('q') => break,
                    Key::Char('h') => active_menu_item = MenuItem::Home,
                    Key::Char('b') => active_menu_item = MenuItem::Birds,
                    Key::Char('a') => {
                        add_random_bird().expect("can add new random bird");
                        active_menu_item = MenuItem::Birds;
                    },
                    Key::Char('d') => {
                        if let Some(selected) = bird_list_state.selected() {
                            remove_bird(selected).expect("can remove bird");
                            if selected > 0 {
                                bird_list_state.select(Some(selected - 1));
                            }
                            active_menu_item = MenuItem::Birds;
                        }
                    },
                    Key::Down => {
                        if let Some(selected) = bird_list_state.selected() {
                            if selected >= bird_count() - 1 {
                                bird_list_state.select(Some(0));
                            } else {
                                bird_list_state.select(Some(selected + 1));
                            }
                        }
                        active_menu_item = MenuItem::Birds;
                    },
                    Key::Up => {
                        if let Some(selected) = bird_list_state.selected() {
                            if selected > 0 {
                                bird_list_state.select(Some(selected - 1));
                            } else {
                                bird_list_state.select(Some(bird_count() - 1));
                            }
                        }
                        active_menu_item = MenuItem::Birds;
                    },
                    _ => {}
                }
            },
            Event::Tick => {},
        }
    }
    terminal.clear()?;
    Ok(())
}