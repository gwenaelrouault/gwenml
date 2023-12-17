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

#[derive(Copy, Clone)]
pub enum Action {
    STANDING,
    WALKING,
    ATTACK1,
}

pub struct ActionDesc {
    pub name : Action,
    pub count : i32,
    pub sprite_index : i32,
    pub sprite_len : i32,
    pub delay : i32,
}

pub struct State {
    pub current_action : ActionDesc,
    pub step : i32,
}

impl State {
    pub fn is_done(&self) -> bool {
        self.step >= self.current_action.count
    }

    pub fn new(action: &ActionDesc) -> Self {
        Self {
            current_action : ActionDesc {
                name : action.name.clone(),
                count : action.count,
                sprite_index : action.sprite_index,
                sprite_len : action.sprite_len,
                delay : action.delay
            },
            step : 0,
        }
    }

    pub fn default() -> Self {
        Self {
            current_action : ActionDesc {
                name : Action::STANDING,
                count : 3,
                sprite_index : 0,
                sprite_len : 150,
                delay : 150,
            },
            step : 0,
        }
    }
}

pub struct Player<'a> {
    pub position : Vector2f,
    pub speed : Vector2f,
    pub sprite : Sprite<'a>,
    pub state : State,
    pub actions : Vec<Action>,
    pub clock : SfBox<Clock>,
}

impl<'a> Player<'a> {
    fn update_position(&mut self) {
        self.position.x = self.position.x +  self.speed.x;
        self.position.y = self.position.y + self.speed.y;
        self.sprite.set_position(self.position);
        self.sprite.set_scale(Vector2f::new(-0.7, 0.7));
    }

    fn get_action_desc(&mut self, action : Action) -> ActionDesc {
        ActionDesc {
            name  : Action::STANDING,
            count : 3,
            sprite_index : 0,
            sprite_len : 150,
            delay : 150,
        }
    }

    fn perform_action(&mut self) {
        if self.state.is_done() {
            match(self.actions.pop()) {
                Some(action) => {
                    let desc = self.get_action_desc(action);
                    self.state = State::new(&desc);
                }
                None => {
                    self.state = State::default();
                }
            }
        }
        let current_player_sprite_rect = IntRect::new(
            self.state.step * self.state.current_action.sprite_len , 
            self.state.current_action.sprite_index * self.state.current_action.sprite_len, 
            self.state.current_action.sprite_len, 
            self.state.current_action.sprite_len);
        self.sprite.set_texture_rect(current_player_sprite_rect);   
        if self.clock.elapsed_time().as_milliseconds() >= self.state.current_action.delay {
            self.state.step = self.state.step + 1;
            self.clock.restart();
        }
        
    }

    pub fn draw(&mut self, window : &mut RenderWindow) {
        self.update_position();
        self.perform_action();
        window.draw(&self.sprite);
    }

    pub fn do_something(&mut self, action : Action) {
        self.actions.push(action);
    }
}