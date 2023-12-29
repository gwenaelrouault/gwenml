use crate::arena::Arena;
use crate::configuration::Configuration;
use crate::fighter_common::Fighter;
use crate::inputs::{InputProcessor, ResultEvent};
use crate::menu::Menu;
use sfml::SfBox;
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow, View},
    system::Vector2f,
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
    fighters: Vec<Box<dyn Fighter + 'a>>,
    selected_fighter: &'a str,
    display: DisplayState,
    menu: Menu<'a>,
    configuration: Configuration,
}

impl<'a> MauriceFight2dEngine<'a> {
    pub fn new(
        window: RenderWindow,
        view: SfBox<View>,
        arena: Arena<'a>,
        fighters: Vec<Box<dyn Fighter + 'a>>,
        selected_fighter: &'a str,
        menu: Menu<'a>,
        configuration: Configuration,
    ) -> Self {
        MauriceFight2dEngine {
            window,
            view,
            arena,
            fighters,
            selected_fighter,
            display: DisplayState::Menu,
            menu,
            configuration,
        }
    }

    fn draw_update_frame_arena(&mut self) {
        self.arena.draw(&mut self.window);
    }

    fn draw_update_frame_fighter(&mut self) {
        for fighter in &mut self.fighters {
            fighter.draw(&mut self.window);
        }
    }

    fn draw_update_frame(&mut self) {
        match self.display {
            DisplayState::Game => {
                self.draw_update_frame_arena();
                let selected = self
                    .fighters
                    .iter()
                    .find(|x| x.get_name().eq_ignore_ascii_case(self.selected_fighter));
                match selected {
                    Some(fighter) => {
                        self.view.move_(fighter.get_speed());
                    }
                    _ => {}
                }

                self.draw_update_frame_fighter();
                self.window.set_view(&self.view);
            }
            DisplayState::Menu => {
                self.menu.draw(&mut self.window, &self.configuration);
            }
        }
    }

    pub fn render_frame(&mut self) {
        self.window.clear(Color::BLACK);
        self.draw_update_frame();
        self.window.display();
    }

    pub fn process_input_event(&mut self, e: Event) -> bool {
        let mut end_game = false;
        match self.display {
            DisplayState::Game => {
                for fighter in &mut self.fighters {
                    if fighter.process_input_event(e) == ResultEvent::Exit {
                        self.display = DisplayState::Menu;
                    }
                }
            }
            DisplayState::Menu => match self.menu.process_event(e) {
                ResultEvent::Exit => {
                    end_game = true;
                }
                ResultEvent::Solo => {
                    self.display = DisplayState::Game;
                }
                _ => {}
            },
        }
        end_game
    }
}
