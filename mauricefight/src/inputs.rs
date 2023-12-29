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
