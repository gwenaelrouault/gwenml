use crate::common::Direction;
use crate::common::InputProcessor;
use crate::common::ResultEvent;
use crate::{configuration::resources::GameResources, sprites::animated_sprite::AnimatedSprite};
use sfml::window::{Event, Key};
use 
   sfml::{
        graphics::{
            RenderTarget, RenderWindow, Sprite,IntRect,
            Transformable,
        },
        system::Vector2f,
    };

#[derive(Copy, Clone, Debug, PartialEq)]
enum MenuAction {
    Nothing,
    Cancel,
    Solo,
    Multi,
    Params
}

pub struct Cursor<'a> {
    sprite : AnimatedSprite<'a>,
    selected : i32,
    step : f32,
    min : Vector2f,
    max : Vector2f,
}

impl<'a> Cursor<'a> {
    pub fn new(resources : &'a GameResources) -> Self {
        Cursor {
            sprite : AnimatedSprite::new(
                resources,
                "cursor", 
                resources.configuration.gui.cursor.sprite.size, 
                resources.configuration.gui.cursor.sprite.display.scale, 
                75.,
                50.,
                0.,
                0.,
                Direction::Right,
                0,
                resources.configuration.gui.cursor.delay,
                resources.configuration.gui.cursor.sprite.nb_frames),
            selected : 1,
            step : 30.,
            min : Vector2f::new(0., 48.),
            max : Vector2f::new(75., 48. * 3.),
        }
    }

    fn draw(&mut self, window : &mut RenderWindow) {
        let frame_res = self.sprite.next_frame(Direction::Right, window);
        if frame_res.0 {
            self.sprite.restart_animation();
        }
    }

    fn move_up(&mut self) {
        let y = self.sprite.nav.position.y - self.step;
        if y  >= self.min.y {
            self.sprite.nav.position.y = y;
            self.selected = self.selected - 1;
        }
    }

    fn move_down(&mut self) {
        let y = self.sprite.nav.position.y + self.step;
        if y <= self.max.y {
            self.sprite.nav.position.y = y;
            self.selected = self.selected + 1;
        }
    }
}

pub struct Menu<'a> {
    background : Sprite<'a>,
    fonts : Sprite<'a>,
    cursor : Cursor<'a>,
}

impl<'a> Menu<'a> {
    pub fn new(resources : &'a GameResources) -> Self { 
        Menu {
            background : resources.get_sprite("background"),
            fonts : resources.get_sprite("fonts"),
            cursor : Cursor::new(resources)
        }
    }

    pub fn on_up(&mut self) {
        self.cursor.move_up();
    }

    pub fn on_down(&mut self) {
        self.cursor.move_down();
    }

    fn select(&mut self) -> MenuAction {
        match self.cursor.selected {
            1 => MenuAction::Solo,
            2 => MenuAction::Multi,
            3 => MenuAction::Params,
            4 => MenuAction::Cancel,
            _ => MenuAction::Nothing,
        }
    }

    pub fn draw(&mut self, window : &mut RenderWindow, resources : &GameResources) {
        self.fonts.set_scale(Vector2f::new(0.5, 0.5));
        window.draw(&self.background);
        self.cursor.draw(window);
        self.print(window, "mode solo", Vector2f::new(100., 50.), resources, 10.);
        self.print(window, "multijoueur", Vector2f::new(100., 80.), resources, 10.);
        self.print(window, "parametres", Vector2f::new(100., 110.), resources, 10.);
        self.print(window, "quitter", Vector2f::new(100., 140.), resources, 10.);
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
        resources : &GameResources,
        tab : f32) {
        for c in text.chars() { 
            match self.get_sprite_letter_index_from_char(c) {
                Some(index) => {
                    let rect = IntRect::new(
                        index * resources.configuration.gui.fonts.sprite.size,
                        0,
                        resources.configuration.gui.fonts.sprite.size,
                        resources.configuration.gui.fonts.sprite.size,
                    );
                    //println!("POSITION {}", position.x);
                    self.fonts.set_texture_rect(rect);
                    self.fonts.set_position(position);
                   
                    window.draw(&self.fonts);
                },
                None => {
                    //WHITESPACE
                }
            }
            position.x = position.x + tab;
        }
    }
}

impl<'a> InputProcessor for Menu<'a> {
    fn process_event(&mut self, e: Event) -> ResultEvent {
        let mut res = ResultEvent::Menu;
        match e {
            Event::KeyPressed {
                code: Key::Down, ..
            } => {
                println!("KEY PUSH:DOWN");
                self.on_down();
            }
            Event::KeyPressed {
                code: Key::Up, ..
            } => {
                println!("KEY PUSH:DOWN");
                self.on_up();
            }
            Event::KeyPressed {
                code: Key::Enter, ..
            } => {
                println!("KEY PUSH:Enter");
                let action = self.select();
                match action {
                    MenuAction::Cancel => {
                        res = ResultEvent::Exit
                    }
                    MenuAction::Solo => {
                        res = ResultEvent::Solo
                    }
                    _ => {}
                }
            }
            Event::KeyPressed {
                code: Key::Escape, ..
            } => {
                println!("KEY PUSH:ESC");
                res = ResultEvent::Exit
            }
            _ => {}
        }
        res
    }
}