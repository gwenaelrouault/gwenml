use crate::animated_sprite::AnimatedSprite;
use crate::animated_sprite::AnimationMode;
use crate::configuration::CharacterConfiguration;
use crate::inputs::InputProcessor;
use crate::inputs::ResultEvent;
use crate::common::Direction;
use crate::fighter_common::{FighterInputEvent,Fighter};

use sfml::window::{Event, Key};
use sfml::graphics::Sprite;
use sfml::graphics::RenderWindow;
use std::collections::VecDeque;


pub struct Character<'a, T : Fighter> {
    pub name : &'a str,
    pub sprite: AnimatedSprite<'a>,
    events: VecDeque<FighterInputEvent>,
    custom_behaviour : T,
}

impl<'a, T> Character<'a, T> 
where T : Fighter
{
    pub fn new(character : &CharacterConfiguration,
        custom : T, 
        sprite : Sprite<'a>, 
        x: f32, 
        y: f32, 
        default_action : & str, 
        mode : AnimationMode) -> Self {
        let default_action_config = character.get_action_configuration(default_action).unwrap();
        println!("Action : {:?}", default_action_config);
        Character {
            name : &character.name,
            sprite: AnimatedSprite::new(
                sprite,
                character.sprite.size,
                character.sprite.display.scale,
                x,
                y,
                0.,
                0.,
                Direction::Right,
                default_action_config.sequence.index,
                default_action_config.sequence.delay,
                default_action_config.sequence.nb_frames,
                mode,
            ),
            events : VecDeque::new(),
            custom_behaviour : custom,
        }
    }

    pub fn do_something(&mut self, action: FighterInputEvent) {
        self.events.push_back(action);
    }

    pub fn draw(&mut self, window: &mut RenderWindow) {
        self.perform_action();
        self.sprite.next_frame(window);
    }

    fn perform_action(&mut self) {
       self.custom_behaviour.process_input_event(self.events.pop_front());
    }

}

impl<'a, T> InputProcessor for Character<'a, T> 
where T : Fighter
{
    fn process_event(&mut self, e: Event) -> ResultEvent {
        let mut res = ResultEvent::Solo;
        match e {
            Event::KeyPressed {
                code: Key::Right, ..
            } => {
                println!("KEY PUSH:RIGHT");
                self.do_something(FighterInputEvent::WalkingRight);
            }
            Event::KeyReleased {
                code: Key::Right, ..
            } => {
                println!("KEY PUSH:RIGHT rel");
                self.do_something(FighterInputEvent::EndWalkingRight);
            }
            Event::KeyPressed {
                code: Key::Left, ..
            } => {
                println!("KEY PUSH:LEFT");
                self.do_something(FighterInputEvent::WalkingLeft);
            }
            Event::KeyReleased {
                code: Key::Left, ..
            } => {
                println!("KEY PUSH:LEFT rel");
                self.do_something(FighterInputEvent::EndWalkingLeft);
            }
            Event::KeyPressed {
                code: Key::Down, ..
            } => {
                println!("KEY PUSH:DOWN");
                self.do_something(FighterInputEvent::Crouch);
            }
            Event::KeyReleased {
                code: Key::Down, ..
            } => {
                println!("KEY PUSH:DOWN rel");
                self.do_something(FighterInputEvent::EndCrouch);
            }
            Event::KeyPressed { code: Key::Up, .. } => {
                println!("KEY:UP");
                self.do_something(FighterInputEvent::Standing);
            }
            Event::KeyPressed { code: Key::A, .. } => {
                println!("KEY:MDDLE KICK");
                self.do_something(FighterInputEvent::Attack1);
            }
            Event::KeyPressed { code: Key::Z, .. } => {
                println!("KEY:HIGH KICK");
                self.do_something(FighterInputEvent::Attack2);
            }
            Event::KeyPressed { code: Key::E, .. } => {
                println!("KEY:BLOCKING");
                self.do_something(FighterInputEvent::Blocking);
            }
            Event::KeyPressed { code: Key::Escape, .. } => {
                println!("KEY:ESC");
                res = ResultEvent::Exit;
            }
            _ => {}
        }
        res
    }
}