use sfml::SfBox;
use 
    sfml::{
        audio::{Sound, SoundBuffer, SoundSource},
        graphics::{
            CircleShape, Color, Font, RectangleShape, RenderTarget, RenderWindow, Shape, Text,Texture,Sprite,IntRect,View,FloatRect,Image,
            Transformable,
        },
        system::{Clock, Time, Vector2f},
        window::{ContextSettings, Event,  Key, Style}
};
pub struct Player<'a> {
    pub position : Vector2f,
    pub speed : Vector2f,
    pub sprite : Sprite<'a>,
}

impl<'a> Player<'a> {
    fn update_position(&mut self) {
        self.position.x = self.position.x +  self.speed.x;
        self.position.y = self.position.y + self.speed.y;
        self.sprite.set_position(self.position);
    }

    pub fn draw(&mut self, window : &mut RenderWindow) {
        self.update_position();
        window.draw(&self.sprite);
    }
}