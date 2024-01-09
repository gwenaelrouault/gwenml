use crate::common::Direction;
use crate::inputs::ResultEvent;
use crate::{
    sprites::animated_sprite::AnimationMode,
    configuration::configuration::CharacterConfiguration,
    fighter_common::{Character, Fighter},
};
use sfml::graphics::Sprite;
use sfml::window::{Event, Key};
use std::collections::VecDeque;
use std::fmt;

#[derive(Copy, Clone, Debug)]
struct InputState {
    flag_crouch: bool,
    flag_move: bool,
    direction: Direction,
    flag_attack: bool,
}

enum JeanJacquesInput {
    Idle,
    WalkingRight,
    EndWalkingRight,
    WalkingLeft,
    EndWalkingLeft,
    Crouch,
    EndCrouch,
    MiddleKick,
    Punch,
    Exit,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum JeanJacquesRun {
    Idle,
    Walking,
    Crouch,
    MiddleKick,
    Punch,
    CrouchPunch,
}

impl fmt::Display for JeanJacquesRun {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JeanJacquesRun::Walking => write!(f, "walking"),
            JeanJacquesRun::Crouch => write!(f, "crouch"),
            JeanJacquesRun::MiddleKick=> write!(f, "middle_kick"),
            JeanJacquesRun::Punch=> write!(f, "punch"),
            JeanJacquesRun::CrouchPunch=> write!(f, "crouch_punch"),
            _ => write!(f, "idle"),
        }
    }
}

pub struct JeanJacques<'a> {
    character: Character<'a>,
    input_state: InputState,
    events_queue: VecDeque<JeanJacquesInput>,
    current_action: JeanJacquesRun,
    current_direction: Direction,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            flag_crouch: false,
            flag_move: false,
            direction: Direction::Right,
            flag_attack: false,
        }
    }

    pub fn on_end_action(&mut self) {
        self.flag_attack = false;
    }
}

impl<'a> JeanJacques<'a> {
    pub fn new(
        configuration: &CharacterConfiguration,
        sprite: Sprite<'a>,
        x: f32,
        y: f32,
        default_action: &str,
        mode: AnimationMode,
        is_human: bool,
    ) -> Self {
        JeanJacques {
            character: Character::new(configuration, &sprite, x, y, default_action, mode, is_human),
            input_state: InputState::new(),
            events_queue: VecDeque::new(),
            current_action: JeanJacquesRun::Idle,
            current_direction: Direction::Right,
        }
    }

    fn get_running_action_from_current_state(&self) -> (JeanJacquesRun, Direction) {
        match self.input_state {
            InputState {
                flag_crouch: true,
                flag_attack: false, ..
            } => (JeanJacquesRun::Crouch, self.input_state.direction),
            InputState {
                flag_crouch: true,
                flag_attack: true, ..
            } => (JeanJacquesRun::CrouchPunch, self.input_state.direction),
            InputState {
                flag_crouch: false,
                flag_move: true,
                flag_attack: false,
                ..
            } => (JeanJacquesRun::Walking, self.input_state.direction),
            _ => (JeanJacquesRun::Idle, self.input_state.direction),
        }
    }

    fn update_state(&mut self, e: JeanJacquesInput) {
        match e {
            JeanJacquesInput::WalkingLeft => {
                println!("KEY POP - LEFT");
                self.input_state.flag_move = true;
                self.input_state.direction = Direction::Left;
            }
            JeanJacquesInput::EndWalkingLeft => {
                println!("KEY POP - LEFT rel");
                self.input_state.flag_move = false;
            }
            JeanJacquesInput::WalkingRight => {
                println!("KEY POP - RIGHT");
                self.input_state.flag_move = true;
                self.input_state.direction = Direction::Right;
            }
            JeanJacquesInput::EndWalkingRight => {
                println!("KEY POP - RIGHT rel");
                self.input_state.flag_move = false;
            },
            JeanJacquesInput::Crouch => {
                println!("KEY POP - RIGHT rel");
                self.input_state.flag_crouch = true;
            },
            JeanJacquesInput::EndCrouch => {
                println!("KEY POP - RIGHT rel");
                self.input_state.flag_crouch = false;
            },
            JeanJacquesInput::Punch => {
                println!("KEY POP - RIGHT rel");
                self.input_state.flag_attack = true;
            },
            _ => {}
        }
    }

}

impl<'a> Fighter for JeanJacques<'a> {
    fn get_name(&self) -> &str {
        &self.character.configuration.name.as_str()
    }

    fn get_speed(&self) -> sfml::system::Vector2f {
        self.character.get_speed()
    }
    

    fn draw(&mut self, window: &mut sfml::graphics::RenderWindow) {
        let to_perform = match self.events_queue.pop_front() {
            Some(action) => {
                self.update_state(action);
                let action_to_perform = self.get_running_action_from_current_state();
                println!("action {:?}", action_to_perform);
                action_to_perform
            }
            None => (self.current_action.clone(), self.current_direction.clone()),
        };
        match to_perform {
            // new action to perform
            (_r, _d) if self.current_action != _r || self.current_direction != _d => {
                println!("---------------NEW ACTION {}->{}", self.current_action,_r);
                self.current_direction = _d;
                self.current_action = _r;
                let mode = if self.current_action == JeanJacquesRun::Idle || self.current_action == JeanJacquesRun::Walking || self.current_action == JeanJacquesRun::Crouch {AnimationMode::Repeated} else  {AnimationMode::OneShot};
                self.character.start_action(self.current_action.to_string().as_str(), mode, self.current_direction, window);
            }
            // continue current action
            _ => {
                if self.character.on_draw(window) {
                    self.input_state.on_end_action();
                } 
            }
        }
    }

    fn process_input_event(&mut self, evt: sfml::window::Event) -> ResultEvent {
        ResultEvent::Solo
    }
}
