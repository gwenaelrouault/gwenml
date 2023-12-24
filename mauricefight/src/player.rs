use std::collections::VecDeque;
use crate::game_events::{FighterEvent,ActionTempo,RunAction};
use crate::game_common::Direction;
use crate::game_inputs::InputState;
use sfml::SfBox;
use sfml::{
    graphics::{
        IntRect, RenderTarget,
        RenderWindow, Sprite, Transformable,
    },
    system::{Clock, Vector2f}
};



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

pub struct Player<'a> {
    pub position: Vector2f,
    pub speed: Vector2f,
    pub sprite: Sprite<'a>,
    pub state: State,
    pub input_state: InputState,
    pub actions: VecDeque<FighterEvent>,
    pub clock: SfBox<Clock>,
    pub running_action: RunAction,
    pub running_direction: Direction,
    pub ko : bool,
}

impl<'a> Player<'a> {
    fn update_position(&mut self) {
        self.position.x = self.position.x + self.speed.x;
        self.position.y = self.position.y + self.speed.y;
        self.sprite.set_position(self.position);
        let x_scale = if self.running_direction == Direction::Left { -0.7 } else { 0.7 };
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

    
    fn update_sprite(&mut self) {
        let current_player_sprite_rect = IntRect::new(
            (self.state.current_action.sprite_index + self.state.step)
                * self.state.current_action.sprite_len,
            0,
            self.state.current_action.sprite_len,
            self.state.current_action.sprite_len,
        );
        self.sprite.set_texture_rect(current_player_sprite_rect);
    }

    fn on_closed_current_action(&mut self) {
        if self.is_attacking() {
            self.do_something(FighterEvent::EndAttack);
        }
    }

    fn update_sprite_sequence(&mut self, new_sequence : bool) -> bool{
        let mut is_closed_current_action = false;
        if !new_sequence {        
            if self.clock.elapsed_time().as_milliseconds() >= self.state.current_action.delay {
                if self.state.is_done() && self.state.current_action.is_repeated() {
                    self.state.step = 0;
                } 
                else if !self.state.is_done() {
                     self.state.step = self.state.step + 1;
                }
                else {
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
        self.update_sprite();
        self.update_position();
        window.draw(&self.sprite);
    }

    pub fn do_something(&mut self, action: FighterEvent) {
        self.actions.push_back(action);
    }
}
