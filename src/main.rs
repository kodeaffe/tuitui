mod database;
mod home;
mod footer;
mod menu;
mod bird;

use footer::render_footer;
use home::render_home;
use menu::{MenuItem, render_menu};
use bird::{add_random_bird, bird_count, remove_bird, render_birds};
use std::io;
use std::io::Stdout;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::event::{Event, Key};
use termion::input::TermRead;
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
    Frame,
    Terminal,
};


fn handle_event(
    rx: &mpsc::Receiver<Event>,
    bird_list_state: &mut ListState,
) -> Result<MenuItem, Box<dyn std::error::Error>> {
    match rx.recv()? {
        Event::Key(Key::Char('q')) => Ok(MenuItem::Quit),
        Event::Key(Key::Char('h')) => Ok(MenuItem::Home),
        Event::Key(Key::Char('b')) => Ok(MenuItem::Birds),
        Event::Key(Key::Char('a')) => {
            add_random_bird().expect("can add new random bird");
            Ok(MenuItem::Birds)
        }
        Event::Key(Key::Char('d')) => {
            remove_bird(bird_list_state).expect("can remove bird");
            Ok(MenuItem::Birds)
        }
        Event::Key(Key::Down) => {
            if let Some(selected) = bird_list_state.selected() {
                if selected >= bird_count() - 1 {
                    bird_list_state.select(Some(0));
                } else {
                    bird_list_state.select(Some(selected + 1));
                }
            }
            Ok(MenuItem::Birds)
        }
        Event::Key(Key::Up) => {
            if let Some(selected) = bird_list_state.selected() {
                if selected > 0 {
                    bird_list_state.select(Some(selected - 1));
                } else {
                    bird_list_state.select(Some(bird_count() - 1));
                }
            }
            Ok(MenuItem::Birds)
        }
        _ => Ok(MenuItem::None)
    }
}


fn render_layout(
    active_menu_item: MenuItem,
    bird_list_state: &mut ListState,
    rect: &mut Frame<'_, TermionBackend<RawTerminal<Stdout>>>,
)  {
    let size = rect.size();
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
        .split(size);
    rect.render_widget(render_menu(active_menu_item), chunks[0]);
    match active_menu_item {
        MenuItem::Birds => {
            let birds_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                )
                .split(chunks[1]);
            let (left, right) = render_birds(bird_list_state);
            rect.render_stateful_widget(left, birds_chunks[0], bird_list_state);
            rect.render_widget(right, birds_chunks[1]);
        },
        _ => rect.render_widget(render_home(), chunks[1]),
    }
    rect.render_widget(render_footer(), chunks[2]);
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            for c in std::io::stdin().events() {
                let evt = c.unwrap();
                tx.send(evt).expect("can send events");
            }
            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
    });

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut active_menu_item = MenuItem::Home;
    let mut bird_list_state = ListState::default();
    bird_list_state.select(Some(0));
    loop {
        terminal.draw(|rect| {
            render_layout(active_menu_item, &mut bird_list_state, rect);
        })?;
        match handle_event(&rx, &mut bird_list_state) {
            Ok(item) => {
                if item != MenuItem::None {
                    active_menu_item = item;
                }
            },
            Err(e) => { return Err(e); },
        }
        if active_menu_item == MenuItem::Quit {
            terminal.clear()?;
            break;
        }
    }
    Ok(())
}