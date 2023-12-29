use sfml::graphics::Sprite;
use sfml::graphics::RenderWindow;
use sfml::graphics::RenderTarget;

pub struct Arena<'a> {
    sprite: Sprite<'a>,
}

impl<'a> Arena<'a> {
    pub fn new(sprite: Sprite<'a>) -> Self {
        Arena { sprite }
    }

    pub fn draw(&mut self, window: &mut RenderWindow) {
        window.draw(&self.sprite);
    }
}
