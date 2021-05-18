use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
};


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MenuItem {
    Home,
    Birds,
    None,
    Quit,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Birds => 1,
            _ => 0,
        }
    }
}


pub fn render_menu<'a>(active_item: MenuItem) -> Tabs<'a> {
    let menu_titles = vec!["Home", "Birds", "Add", "Delete", "Quit"];
    let menu = menu_titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();
    let tabs = Tabs::new(menu)
        .select(active_item.into())
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("|"));
    tabs
}
