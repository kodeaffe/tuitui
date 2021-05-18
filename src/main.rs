mod database;
mod home;
mod footer;
mod menu;
mod bird;

use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use footer::render_footer;
use home::render_home;
use menu::{MenuItem, render_menu};
use bird::{add_random_bird, bird_count, remove_bird, render_birds};
use std::io;
use std::io::Stdout;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
    Frame,
    Terminal,
};
use crossterm::event::KeyEvent;


enum Event<I> {
    Input(I),
    Tick,
}

fn handle_event(
    rx: &mpsc::Receiver<Event<KeyEvent>>,
    bird_list_state: &mut ListState,
) -> Result<MenuItem, Box<dyn std::error::Error>> {
    match rx.recv()? {
        Event::Input(event) => match event.code {
            KeyCode::Char('q') => {

                Ok(MenuItem::Quit)
            }
            KeyCode::Char('h') => Ok(MenuItem::Home),
            KeyCode::Char('b') => Ok(MenuItem::Birds),
            KeyCode::Char('a') => {
                add_random_bird().expect("can add new random bird");
                Ok(MenuItem::Birds)
            }
            KeyCode::Char('d') => {
                remove_bird(bird_list_state).expect("can remove bird");
                Ok(MenuItem::Birds)
            }
            KeyCode::Down => {
                if let Some(selected) = bird_list_state.selected() {
                    if selected >= bird_count() - 1 {
                        bird_list_state.select(Some(0));
                    } else {
                        bird_list_state.select(Some(selected + 1));
                    }
                }
                Ok(MenuItem::Birds)
            }
            KeyCode::Up => {
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
        },
        Event::Tick => Ok(MenuItem::None),
    }
}


fn render_layout(
    active_menu_item: MenuItem,
    bird_list_state: &mut ListState,
    rect: &mut Frame<'_, CrosstermBackend<Stdout>>,
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
    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
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
            disable_raw_mode()?;
            terminal.show_cursor()?;
            terminal.clear()?;
            break;
        }
    }
    Ok(())
}