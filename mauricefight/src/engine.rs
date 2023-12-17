use std::collections::HashMap;
use crate::arena::Arena;
use crate::player::Player;
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

pub struct MauriceFight2dEngine<'a> {
    pub window: RenderWindow,
    pub view: SfBox<View>,
    pub arena: Arena<'a>,
    pub player: Player<'a>,
    pub timer : SfBox<Clock>
}

impl<'a> MauriceFight2dEngine<'a> {
    fn draw_update_frame_arena(&mut self) {
        self.arena.draw(&mut self.window);
    }

    fn draw_update_frame_player(&mut self) {
        self.player.draw(&mut self.window);
    }

    fn draw_update_frame(&mut self) {
        self.draw_update_frame_arena();
       
        self.view.move_(self.player.speed);
        self.draw_update_frame_player();
        self.window.set_view(&self.view);
    }

    pub fn render_frame(&mut self) {
        self.window.clear(Color::BLACK);
        self.draw_update_frame();
        self.window.display();
    }

}
