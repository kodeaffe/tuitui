#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
    Home,
    Birds,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Birds => 1,
        }
    }
}