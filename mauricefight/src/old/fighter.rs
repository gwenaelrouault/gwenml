use crate::animated_sprite::{AnimatedSprite, AnimationMode};
use crate::game_common::Direction;
use crate::configuration::CharacterConfiguration;
use crate::game_events::{ActionTempo, FighterEvent, RunAction};
use crate::game_inputs::{InputProcessor, InputState};
use sfml::window::{Event, Key};
use sfml::SfBox;
use sfml::{
    graphics::{IntRect, RenderTarget, RenderWindow, Sprite, Transformable},
    system::{Clock, Vector2f},
};
use std::collections::VecDeque;

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
trait CustomFighter {
    fn get_action_desc(&mut self, action: RunAction) -> ActionDesc;
}

impl std::fmt::Display for ActionDesc {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Action : {:?}, speed : {}", self.name, self.speed)
    }
}

#[derive(Copy, Clone)]
pub struct State {
    pub current_action: ActionDesc,
    pub step: i32,
}

impl State {
    pub fn is_done(&self) -> bool {
        self.step >= self.current_action.count - 1
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
                count: 4,
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

pub struct Fighter<'a> {
    sprite: AnimatedSprite<'a>,
    state: State,
    input_state: InputState,
    actions: VecDeque<FighterEvent>,
    running_action: RunAction,
    ko: bool,
}

impl<'a> Fighter<'a> {
    pub fn new(character : &CharacterConfiguration, sprite : Sprite, x: f32, y: f32, default_action : & str) -> Self {        
        Fighter {
            sprite: AnimatedSprite::new(
                sprite,
                character.sprite.size,
                character.sprite.display.scale,
                x,
                y,
                0.,
                0.,
                Direction::Right,
                2,
                AnimationMode::Repeated,
            ),
            state: State::new(ActionDesc::default),
            input_state : InputState::new(),
            actions : Vec::new(),
            running_action : RunAction::Blocking,
            ko : false
        }
    }

    fn update_position(&mut self) {
        self.position.x = self.position.x + self.speed.x;
        self.position.y = self.position.y + self.speed.y;
        self.sprite.set_position(self.position);
        let x_scale = if self.running_direction == Direction::Left {
            -0.7
        } else {
            0.7
        };
        self.sprite.set_scale(Vector2f::new(x_scale, 0.7));
    }

    fn is_attacking(&self) -> bool {
        self.running_action == RunAction::MiddleKick || self.running_action == RunAction::HighKick
    }

    fn get_action_desc(&mut self, action: RunAction) -> ActionDesc {
        match action {
            RunAction::Walking => ActionDesc {
                name: RunAction::Walking,
                count: 6,
                sprite_index: 0,
                sprite_len: 100,
                delay: 150,
                speed: 0.3,
                tempo: ActionTempo::Continu,
            },
            RunAction::Crouch => ActionDesc {
                name: RunAction::Crouch,
                count: 1,
                sprite_index: 20,
                sprite_len: 100,
                delay: 1000,
                speed: 0.,
                tempo: ActionTempo::Continu,
            },
            RunAction::MiddleKick => ActionDesc {
                name: RunAction::MiddleKick,
                count: 3,
                sprite_index: 9,
                sprite_len: 100,
                delay: 150,
                speed: 0.,
                tempo: ActionTempo::Immediate,
            },
            RunAction::HighKick => ActionDesc {
                name: RunAction::HighKick,
                count: 9,
                sprite_index: 24,
                sprite_len: 100,
                delay: 100,
                speed: 0.,
                tempo: ActionTempo::Immediate,
            },
            RunAction::Blocking => ActionDesc {
                name: RunAction::Blocking,
                count: 1,
                sprite_index: 37,
                sprite_len: 100,
                delay: 1000,
                speed: 0.,
                tempo: ActionTempo::Immediate,
            },
            RunAction::CrouchBlocking => ActionDesc {
                name: RunAction::CrouchBlocking,
                count: 1,
                sprite_index: 38,
                sprite_len: 100,
                delay: 1000,
                speed: 0.,
                tempo: ActionTempo::Immediate,
            },
            _ => ActionDesc {
                name: RunAction::Standing,
                count: 4,
                sprite_index: 5,
                sprite_len: 100,
                delay: 150,
                speed: 0.,
                tempo: ActionTempo::Infinite,
            },
        }
    }

    fn on_closed_current_action(&mut self) {
        if self.is_attacking() {
            self.do_something(FighterEvent::EndAttack);
        }
    }

    fn update_sprite_sequence(&mut self, new_sequence: bool) -> bool {
        let mut is_closed_current_action = false;
        if !new_sequence {
            if self.clock.elapsed_time().as_milliseconds() >= self.state.current_action.delay {
                if self.state.is_done() && self.state.current_action.is_repeated() {
                    self.state.step = 0;
                } else if !self.state.is_done() {
                    self.state.step = self.state.step + 1;
                } else {
                    println!("END ACTION {:?}", self.state.current_action);
                    is_closed_current_action = true;
                    self.on_closed_current_action();
                }
                self.clock.restart();
            }
        }
        is_closed_current_action
    }

    fn perform_action(&mut self) {
        let mut all_input_processed = false;
        while !all_input_processed {
            let to_perform = match self.actions.pop_front() {
                Some(action) => {
                    let action_to_perform = self.input_state.get_current_action_from_input(action);
                    println!("action {:?}", action_to_perform);
                    action_to_perform
                }
                None => {
                    all_input_processed = true;
                    (self.running_action.clone(), self.running_direction.clone())
                }
            };
            match to_perform {
                (_r, _d) if self.running_action != _r || self.running_direction != _d => {
                    self.running_direction = _d;
                    self.running_action = _r;
                    let desc = self.get_action_desc(self.running_action);
                    println!("NEW Action {}", desc);
                    self.state = State::new(&desc);
                    self.speed.x = match self.running_direction {
                        Direction::Left => -1. * self.state.current_action.speed,
                        _ => self.state.current_action.speed,
                    };
                    self.update_sprite_sequence(true);
                }
                _ => {
                    //println!("CURRENT Action {:?}", self.running_action);
                    self.update_sprite_sequence(false);
                }
            }
        }
    }

    pub fn draw(&mut self, window: &mut RenderWindow) {
        self.perform_action();
        self.sprite.next_frame(window);
    }

    pub fn do_something(&mut self, action: FighterEvent) {
        self.actions.push_back(action);
    }
}

impl<'a> InputProcessor for Fighter<'a> {
    fn process_event(&mut self, e: Event) -> bool {
        match e {
            Event::KeyPressed {
                code: Key::Right, ..
            } => {
                println!("KEY PUSH:RIGHT");
                self.do_something(FighterEvent::WalkingRight);
            }
            Event::KeyReleased {
                code: Key::Right, ..
            } => {
                println!("KEY PUSH:RIGHT rel");
                self.do_something(FighterEvent::EndWalkingRight);
            }
            Event::KeyPressed {
                code: Key::Left, ..
            } => {
                println!("KEY PUSH:LEFT");
                self.do_something(FighterEvent::WalkingLeft);
            }
            Event::KeyReleased {
                code: Key::Left, ..
            } => {
                println!("KEY PUSH:LEFT rel");
                self.do_something(FighterEvent::EndWalkingLeft);
            }
            Event::KeyPressed {
                code: Key::Down, ..
            } => {
                println!("KEY PUSH:DOWN");
                self.do_something(FighterEvent::Crouch);
            }
            Event::KeyReleased {
                code: Key::Down, ..
            } => {
                println!("KEY PUSH:DOWN rel");
                self.do_something(FighterEvent::EndCrouch);
            }
            Event::KeyPressed { code: Key::Up, .. } => {
                println!("KEY:UP");
                self.do_something(FighterEvent::Standing);
            }
            Event::KeyPressed { code: Key::A, .. } => {
                println!("KEY:MDDLE KICK");
                self.do_something(FighterEvent::Attack1);
            }
            Event::KeyPressed { code: Key::Z, .. } => {
                println!("KEY:HIGH KICK");
                self.do_something(FighterEvent::Attack2);
            }
            Event::KeyPressed { code: Key::E, .. } => {
                println!("KEY:BLOCKING");
                self.do_something(FighterEvent::Blocking);
            }
            _ => {}
        }
        false
    }
}
