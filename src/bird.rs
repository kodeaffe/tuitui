use chrono::prelude::*;
use rand::{distributions::Alphanumeric, prelude::*};
use serde::{Deserialize, Serialize};
use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, Row, Table
    },
};

use crate::database::{Error, read_db, write_db};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bird {
    id: usize,
    name: String,
    species: String,
    age: usize,
    created_at: DateTime<Utc>,
}


pub fn render_birds<'a>(selected: usize) -> (List<'a>, Table<'a>) {
    let birds = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Birds")
        .border_type(BorderType::Plain);
    let bird_list = read_db::<Bird>().expect("can fetch bird list");
    let items: Vec<_> = bird_list
        .iter()
        .map(|bird| {
            ListItem::new(Spans::from(vec![Span::styled(
                bird.name.clone(),
                Style::default(),
            )]))
        })
        .collect();
    let list = List::new(items).block(birds).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );
    let selected_bird: Bird;
    if let Some(bird) = bird_list.get(selected) {
        selected_bird = bird.clone();
    } else {
        return (list, Table::new(vec![]));
    }

    let bird_detail = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_bird.id.to_string())),
        Cell::from(Span::raw(selected_bird.name)),
        Cell::from(Span::raw(selected_bird.species)),
        Cell::from(Span::raw(selected_bird.age.to_string())),
        Cell::from(Span::raw(selected_bird.created_at.to_string())),
    ])])
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "ID",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Name",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Species",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Age",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Created At",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Detail")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(5),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(5),
        Constraint::Percentage(20),
    ]);

    (list, bird_detail)
}


pub fn remove_bird(selected: usize) -> Result<(), Error> {
    let mut bird_list = read_db::<Bird>()?;
    match bird_list.get(selected) {
        Some(_) => {
            bird_list.remove(selected);
            write_db::<Bird>(&bird_list)?;
        },
        _ => (),
    }
    Ok(())
}


pub fn add_random_bird() -> Result<Vec<Bird>, Error> {
    let mut rng = rand::thread_rng();
    let species = match rng.gen_range(0, 4) {
        0 => "Kea",
        1 => "Kiwi",
        2 => "Fantail",
        3 => "Moa",
        _ => "Tui",
    };
    let random_bird = Bird {
        id: rng.gen_range(0, 9999999),
        name: rng.sample_iter(Alphanumeric).take(10).collect(),
        species: species.to_owned(),
        age: rng.gen_range(1, 15),
        created_at: Utc::now(),
    };
    let mut bird_list = read_db::<Bird>()?;
    bird_list.push(random_bird);
    write_db::<Bird>(&bird_list)?;
    Ok(bird_list)
}


pub fn bird_count() -> usize {
    match read_db::<Bird>() {
        Ok(bird_list) => bird_list.len(),
        _ => 0,
    }
}