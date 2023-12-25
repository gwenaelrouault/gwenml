use std::collections::HashMap;
use crate::game_configuration::GameConfiguration;
use crate::arena::Arena;
use crate::player::Player;
use crate::menu::Menu;
use crate::game_events::FighterEvent;
use sfml::SfBox;
use sfml::{
    audio::{Sound, SoundBuffer, SoundSource},
    graphics::{
        CircleShape, Color, FloatRect, Font, Image, IntRect, RectangleShape, RenderTarget,
        RenderWindow, Shape, Sprite, Text, Texture, Transformable, View,
    },
    system::{Clock, Time, Vector2f},
    window::{ContextSettings, Event, Key, Style},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DisplayState {
    Menu,
    Game
}

pub struct MauriceFight2dEngine<'a> {
    pub window: RenderWindow,
    view: SfBox<View>,
    arena: Arena<'a>,
    player: Player<'a>,
    timer : SfBox<Clock>,
    display : DisplayState,
    menu : Menu<'a>,
    configuration : GameConfiguration,
}

impl<'a> MauriceFight2dEngine<'a> {
    pub fn new(
        window: RenderWindow, 
        view: SfBox<View>,
        arena: Arena<'a>, 
        player: Player<'a>, 
        menu : Menu<'a>, 
        configuration : GameConfiguration) -> Self {
        let timer = Clock::start();
        MauriceFight2dEngine {
            window,
            view,
            arena,
            player,
            timer,
            display : DisplayState::Menu,
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
            },
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

    pub fn process_input_event(&mut self, e : Event) {
        match e {
            Event::KeyPressed {
                code: Key::Right,..
            } => {
                println!("KEY PUSH:RIGHT");
                self.player.do_something(FighterEvent::WalkingRight);
            },
            Event::KeyReleased {
                code: Key::Right,..
            } => {
                println!("KEY PUSH:RIGHT rel");
                self.player.do_something(FighterEvent::EndWalkingRight);
            },
            Event::KeyPressed {
                code: Key::Left,..
            } => {
                println!("KEY PUSH:LEFT");
                self.player.do_something(FighterEvent::WalkingLeft);
            },
            Event::KeyReleased {
                code: Key::Left,..
            } => {
                println!("KEY PUSH:LEFT rel");
                self.player.do_something(FighterEvent::EndWalkingLeft);
            },
            Event::KeyPressed {
                code: Key::Down,..
            } => {
                println!("KEY PUSH:DOWN");
                self.player.do_something(FighterEvent::Crouch);
            },
            Event::KeyReleased {
                code: Key::Down,..
            } => {
                println!("KEY PUSH:DOWN rel");
                self.player.do_something(FighterEvent::EndCrouch);
            },
            Event::KeyPressed {
                code: Key::Up,..
            } => {
                println!("KEY:UP");
                self.player.do_something(FighterEvent::Standing);
            },
            Event::KeyPressed {
                code: Key::A,..
            } => {
                println!("KEY:MDDLE KICK");
                self.player.do_something(FighterEvent::Attack1);
            },
            Event::KeyPressed {
                code: Key::Z,..
            } => {
                println!("KEY:HIGH KICK");
                self.player.do_something(FighterEvent::Attack2);
            },
            Event::KeyPressed {
                code: Key::E,..
            } => {
                println!("KEY:BLOCKING");
                self.player.do_something(FighterEvent::Blocking);
            },
            _ => {}
        }
    }

}
