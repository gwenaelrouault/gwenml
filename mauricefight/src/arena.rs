use sfml::SfBox;

use {
    rand::{thread_rng, Rng},
    sfml::{
        audio::{Sound, SoundBuffer, SoundSource},
        graphics::{
            CircleShape, Color, Font, RectangleShape, RenderTarget, RenderWindow, Shape, Text,Texture,Sprite,IntRect,View,FloatRect,Image,
            Transformable,
        },
        system::{Clock, Time, Vector2f},
        window::{ContextSettings, Event,  Key, Style},
    },
    std::{env, f32::consts::PI},
};

pub struct Arena<'a> {
    sprite : Sprite<'a>
}

impl<'a> Arena<'a> {
    pub fn new(sprite : Sprite<'a>) -> Self { 
        Arena {
            sprite
        }
    }

    pub fn draw(&mut self, window : &mut RenderWindow) {
        window.draw(&self.sprite);
    }
}