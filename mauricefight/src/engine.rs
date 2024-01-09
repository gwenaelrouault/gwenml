use crate::arena::Arena;
use crate::configuration::resources::{self, GameResources};
use crate::fighters::fighter::Fighter;
use crate::gui::menu::Menu;
use crate::common::{InputProcessor, ResultEvent};
use sfml::SfBox;
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow, View},
    window::Event,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DisplayState {
    Menu,
    Game,
}

pub struct MauriceFight2dEngine<'a> {
    pub window: RenderWindow,
    view: SfBox<View>,
    arena: Arena<'a>,
    fighters: Vec<Box<Fighter<'a>>>,
    display: DisplayState,
    menu: Menu<'a>,
}

impl<'a> MauriceFight2dEngine<'a> {
    pub fn new(
        window: RenderWindow,
        view: SfBox<View>,
        arena: Arena<'a>,
        resources: &'a GameResources,
    ) -> Self {
        MauriceFight2dEngine {
            window,
            view,
            arena,
            fighters: Vec::new(),
            display: DisplayState::Menu,
            menu: Menu::new(resources),
        }
    }

    fn draw_update_frame_arena(&mut self) {
        self.arena.draw(&mut self.window);
    }

    fn draw_update_frame(&mut self, resources: &GameResources) {
        match self.display {
            DisplayState::Game => {
                self.draw_update_frame_arena();
                for fighter in self.fighters.iter_mut() {
                    //println!("FIGHTER : {}", fighter.get_name());
                    fighter.on_frame_update(&mut self.window);
                    if fighter.selected {
                        self.view.move_(fighter.get_speed())
                    }
                }
            }
            DisplayState::Menu => {
                self.menu.draw(&mut self.window, resources);
            }
        }
    }

    pub fn render_frame(&mut self, resources: &GameResources) {
        self.window.clear(Color::BLACK);
        self.draw_update_frame(resources);
        self.window.set_view(&self.view);
        self.window.display();
    }

    fn load_solo_level(&mut self, resources: &'a GameResources) {
        self.fighters.clear();
        self.fighters.push(Box::new(Fighter::new(
            "Maurice", &resources, "Maurice", 120., 150., true,
        )));
    }

    pub fn process_input_event(&mut self, e: Event, resources: &'a GameResources) -> bool {
        let mut end_game = false;
        match self.display {
            DisplayState::Game => {
                for fighter in &mut self.fighters {
                    if fighter.selected {
                        if fighter.as_mut().process_event(e) == ResultEvent::Menu {
                            self.display = DisplayState::Menu;
                        }
                    }
                }
            }
            DisplayState::Menu => match self.menu.process_event(e) {
                ResultEvent::Exit => {
                    end_game = true;
                }
                ResultEvent::Solo => {
                    self.display = DisplayState::Game;
                    self.load_solo_level(resources);
                }
                _ => {}
            },
        }
        end_game
    }
}
