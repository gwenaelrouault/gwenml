use crate::game_configuration::GameConfiguration;
use 
   sfml::{
        graphics::{
            RenderTarget, RenderWindow, Sprite,IntRect,
            Transformable,
        },
        system::Vector2f,
    };

struct Cursor<'a> {
    sprite : Sprite<'a>,
    position : Vector2f,
    step : f32,
    min : Vector2f,
    max : Vector2f,
}

impl<'a> Cursor<'a> {

    pub fn new(sprite : Sprite<'a>, configuration : &GameConfiguration) -> Self {
        Cursor {
            sprite,
            position : Vector2f::new(75., 48.),
            step : 30.,
            min : Vector2f::new(0., 48.),
            max : Vector2f::new(75., 48. * 3.),
        }
    }

    fn draw(&mut self, window : &mut RenderWindow) {
        self.sprite.set_position(self.position);
        self.sprite.set_scale(Vector2f::new(0.3, 0.3));
        window.draw(&self.sprite);
    }

    fn move_up(&mut self) {
        let y = self.position.y - self.step;
        if y  >= self.min.y {
            self.position.y = y;
        }
    }

    fn move_down(&mut self) {
        let y = self.position.y + self.step;
        if y <= self.max.y {
            self.position.y = y;
        }
    }

    fn select(&mut self) {
        
    }
}

pub struct Menu<'a> {
    background : Sprite<'a>,
    letters : Sprite<'a>,
    cursor : Cursor<'a>,
}

impl<'a> Menu<'a> {
    pub fn new(sprite_bg : Sprite<'a>, sprite_letters : Sprite<'a>, skull : Sprite<'a>, configuration : &GameConfiguration) -> Self { 
        Menu {
            background : sprite_bg,
            letters : sprite_letters,
            cursor : Cursor::new(skull, configuration)
        }
    }

    pub fn on_up(&mut self) {
        self.cursor.move_up();
    }

    pub fn on_down(&mut self) {
        self.cursor.move_down();
    }

    pub fn draw(&mut self, window : &mut RenderWindow, configuration : &GameConfiguration) {
        self.letters.set_scale(Vector2f::new(0.5, 0.5));
        window.draw(&self.background);
        self.cursor.draw(window);
        self.print(window, "mode solo", Vector2f::new(100., 50.), configuration, 10.);
        self.print(window, "multijoueur", Vector2f::new(100., 80.), configuration, 10.);
        self.print(window, "parametres", Vector2f::new(100., 110.), configuration, 10.);
        self.print(window, "quitter", Vector2f::new(100., 140.), configuration, 10.);
    }

    fn get_sprite_letter_index_from_char(&self, c : char) -> Option<i32> {
        match c {
            'a' => Option::from(0),
            'b' => Option::from(1),
            'c' => Option::from(2),
            'd' => Option::from(3),
            'e' => Option::from(4),
            'f' => Option::from(5),
            'g' => Option::from(6),
            'h' => Option::from(7),
            'i' => Option::from(8),
            'j' => Option::from(9),
            'k' => Option::from(10),
            'l' => Option::from(11),
            'm' => Option::from(12),
            'n' => Option::from(13),
            'o' => Option::from(14),
            'p' => Option::from(15),
            'q' => Option::from(16),
            'r' => Option::from(17),
            's' => Option::from(18),
            't' => Option::from(19),
            'u' => Option::from(20),
            'v' => Option::from(21),
            'w' => Option::from(22),
            'x' => Option::from(23),
            'y' => Option::from(24),
            'z' => Option::from(25),
            _=> Option::None,
        }
    }

    fn print(&mut self, 
        window : &mut RenderWindow, 
        text : &str, 
        mut position : Vector2f, 
        configuration : &GameConfiguration,
        tab : f32) {
        for c in text.chars() { 
            match self.get_sprite_letter_index_from_char(c) {
                Some(index) => {
                    let rect = IntRect::new(
                        index * configuration.texture_pack.size_letter,
                        0,
                        configuration.texture_pack.size_letter,
                        configuration.texture_pack.size_letter,
                    );
                    //println!("POSITION {}", position.x);
                    self.letters.set_texture_rect(rect);
                    self.letters.set_position(position);
                   
                    window.draw(&self.letters);
                },
                None => {
                    //WHITESPACE
                }
            }
            position.x = position.x + tab;
        }
    }
}