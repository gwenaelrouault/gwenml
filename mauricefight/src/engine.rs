use crate::arena::Arena;
use crate::game_configuration::GameConfiguration;
use crate::game_events::FighterEvent;
use crate::game_inputs::InputProcessor;
use crate::menu::Menu;
use crate::player::Player;
use sfml::SfBox;
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow, View},
    system::{Clock, Vector2f},
    window::{Event, Key},
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
    player: Player<'a>,
    timer: SfBox<Clock>,
    display: DisplayState,
    menu: Menu<'a>,
    configuration: GameConfiguration,
}

impl<'a> MauriceFight2dEngine<'a> {
    pub fn new(
        window: RenderWindow,
        view: SfBox<View>,
        arena: Arena<'a>,
        player: Player<'a>,
        menu: Menu<'a>,
        configuration: GameConfiguration,
    ) -> Self {
        let timer = Clock::start();
        MauriceFight2dEngine {
            window,
            view,
            arena,
            player,
            timer,
            display: DisplayState::Menu,
            menu,
            configuration,
        }
    }

    fn draw_update_frame_arena(&mut self) {
        self.arena.draw(&mut self.window);
    }

    fn draw_update_frame_player(&mut self) {
        self.player.draw(&mut self.window);
    }

    fn draw_update_frame(&mut self) {
        match self.display {
            DisplayState::Game => {
                self.draw_update_frame_arena();
                self.view.move_(self.player.speed);
                self.draw_update_frame_player();
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

    pub fn process_input_event(&mut self, e: Event) {
        match self.display {
            DisplayState::Game => {
                self.player.process_event(e);
            },
            DisplayState::Menu => {
                self.menu.process_event(e);
            }
        }
    }
}
