use std::fmt;
use sfml::window::Event;
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ResultEvent {
    Exit,
    Solo,
    Menu
}

pub trait InputProcessor {
    fn process_event(&mut self, e: Event) -> ResultEvent;
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}