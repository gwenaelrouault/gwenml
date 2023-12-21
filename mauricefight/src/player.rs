use std::collections::VecDeque;
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
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum Action {
    Standing,
    WalkingRight,
    WalkingLeft,
    EndWalkingRight,
    EndWalkingLeft,
    Crouch,
    EndCrouch,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RunAction {
    Standing,
    Walking,
    Crouch,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
}
#[derive(Copy, Clone, Debug)]
pub enum ActionTempo {
    Infinite,
    Continu,
    Immediate,
}

#[derive(Copy, Clone, Debug)]
pub struct ActionDesc {
    pub name: RunAction,
    pub count: i32,
    pub sprite_index: i32,
    pub sprite_len: i32,
    pub delay: i32,
    pub speed: f32,
    pub tempo: ActionTempo,
}

impl ActionDesc {
    pub fn is_repeated(&self) -> bool {
        match self.tempo {
            ActionTempo::Infinite | ActionTempo::Continu => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for ActionDesc {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Action : {:?}, speed : {}", self.name, self.speed)
    }
}

#[derive(Copy, Clone)]
pub struct InputState {
    pub flag_crouch: bool,
    pub flag_move: bool,
    pub direction: Direction,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            flag_crouch: false,
            flag_move: false,
            direction: Direction::Right,
        }
    }
}

#[derive(Copy, Clone)]
pub struct State {
    pub current_action: ActionDesc,
    pub step: i32,
}

impl State {
    pub fn is_done(&self) -> bool {
        self.step >= self.current_action.count
    }

    pub fn new(action: &ActionDesc) -> Self {
        Self {
            current_action: action.clone(),
            step: 0,
        }
    }

    pub fn default() -> Self {
        Self {
            current_action: ActionDesc {
                name: RunAction::Standing,
                count: 3,
                sprite_index: 5,
                sprite_len: 100,
                delay: 150,
                speed: 0.,
                tempo: ActionTempo::Infinite,
            },
            step: 0,
        }
    }
}

pub struct Player<'a> {
    pub position: Vector2f,
    pub speed: Vector2f,
    pub sprite: Sprite<'a>,
    pub state: State,
    pub input_state: InputState,
    pub actions: VecDeque<Action>,
    pub clock: SfBox<Clock>,
    pub running_action: RunAction,
    pub running_direction: Direction,
}

impl<'a> Player<'a> {
    fn update_position(&mut self) {
        self.position.x = self.position.x + self.speed.x;
        self.position.y = self.position.y + self.speed.y;
        self.sprite.set_position(self.position);
        let x_scale = if self.speed.x < 0. { -0.7 } else { 0.7 };
        self.sprite.set_scale(Vector2f::new(x_scale, 0.7));
    }

    fn get_action_desc(&mut self, action: RunAction) -> ActionDesc {
        match action {
            RunAction::Walking => ActionDesc {
                name: RunAction::Walking,
                count: 6,
                sprite_index: 0,
                sprite_len: 100,
                delay: 100,
                speed: 0.25,
                tempo: ActionTempo::Continu,
            },
            RunAction::Crouch => ActionDesc {
                name: RunAction::Crouch,
                count: 1,
                sprite_index: 20,
                sprite_len: 100,
                delay: 100,
                speed: 0.,
                tempo: ActionTempo::Continu,
            },
            _ => ActionDesc {
                name: RunAction::Standing,
                count: 3,
                sprite_index: 5,
                sprite_len: 100,
                delay: 100,
                speed: 0.,
                tempo: ActionTempo::Infinite,
            },
        }
    }

    fn get_current_action_from_input(&mut self, action: Action) -> (RunAction, Direction) {
        println!("INPUT - get_current_action_from_input {:?}", action);
        match action {
            Action::Crouch => self.input_state.flag_crouch = true,
            Action::EndCrouch => self.input_state.flag_crouch = false,
            Action::WalkingLeft => {
                println!("KEY POP - LEFT");
                self.input_state.flag_move = true;
                self.input_state.direction = Direction::Left;
            },
            Action::EndWalkingLeft => {
                println!("KEY POP - LEFT rel");
                self.input_state.flag_move = false;
            },
            Action::WalkingRight => {
                println!("KEY POP - RIGHT");
                self.input_state.flag_move = true;
                self.input_state.direction = Direction::Right;
            },
            Action::EndWalkingRight => {
                println!("KEY POP - RIGHT rel");
                self.input_state.flag_move = false;
            },
            _ => {}
        }
        match self.input_state {
            InputState {
                flag_crouch: true, ..
            } => (RunAction::Crouch, self.input_state.direction),
            InputState {
                flag_crouch: false,
                flag_move: true,
                ..
            } => (RunAction::Walking, self.input_state.direction),
            _ => (RunAction::Standing, self.input_state.direction),
        }
    }

    fn perform_action(&mut self) {
        let mut all_input_processed = false;
        while !all_input_processed {
            let to_perform = match self.actions.pop_front() {
                Some(action) => {
                    let action_to_perform = self.get_current_action_from_input(action);
                    println!("action {:?}", action_to_perform);
                    action_to_perform
                }
                None => {
                    all_input_processed = true;
                    (self.running_action.clone(), self.running_direction.clone())
                }
            };
            //println!("CURRENT {:?} / NEW {:?}", self.running_action, to_perform);
            match to_perform {
                (_r, _d) if self.running_action != _r || self.running_direction != _d => {
                    self.running_direction = _d;
                    self.running_action = _r;
                    let desc = self.get_action_desc(self.running_action);
                    println!("aNEW ction {}", desc);
                    self.state = State::new(&desc);
                    self.speed.x = match self.running_direction {
                        Direction::Left => -1. * self.state.current_action.speed,
                        _ => self.state.current_action.speed,
                    }
                }
                _ => {
                    if self.clock.elapsed_time().as_milliseconds()
                        >= self.state.current_action.delay
                    {
                        if self.state.is_done() && self.state.current_action.is_repeated() {
                            self.state.step = 0;
                        } else {
                            self.state.step = self.state.step + 1;
                        }
                        self.clock.restart();
                    }
                }
            }
        }
        let current_player_sprite_rect = IntRect::new(
            (self.state.current_action.sprite_index + self.state.step)
                * self.state.current_action.sprite_len,
            0,
            self.state.current_action.sprite_len,
            self.state.current_action.sprite_len,
        );
        self.sprite.set_texture_rect(current_player_sprite_rect);
    }

    pub fn draw(&mut self, window: &mut RenderWindow) {
        self.perform_action();
        self.update_position();
        window.draw(&self.sprite);
    }

    pub fn do_something(&mut self, action: Action) {
        self.actions.push_back(action);
    }
}
