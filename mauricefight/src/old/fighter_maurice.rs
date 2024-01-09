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

#[derive(Copy, Clone, Debug, PartialEq)]
enum MauriceRun {
    Idle,
    Walking,
    Crouch,
    MiddleKick,
    Punch,
    CrouchPunch,
}

impl fmt::Display for MauriceRun {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MauriceRun::Walking => write!(f, "walking"),
            MauriceRun::Crouch => write!(f, "crouch"),
            MauriceRun::MiddleKick=> write!(f, "middle_kick"),
            MauriceRun::Punch=> write!(f, "punch"),
            MauriceRun::CrouchPunch=> write!(f, "crouch_punch"),
            _ => write!(f, "idle"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum MauriceInput {
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

pub struct Maurice<'a> {
    character: Character<'a>,
    input_state: InputState,
    events_queue: VecDeque<MauriceInput>,
    current_action: MauriceRun,
    current_direction: Direction,
}

impl<'a> Maurice<'a> {
    pub fn new(
        configuration: &CharacterConfiguration,
        sprite: Sprite<'a>,
        x: f32,
        y: f32,
        default_action: &str,
        mode: AnimationMode,
        is_human: bool,
    ) -> Self {
        Maurice {
            character: Character::new(configuration, &sprite, x, y, default_action, mode, is_human),
            input_state: InputState::new(),
            events_queue: VecDeque::new(),
            current_action: MauriceRun::Idle,
            current_direction: Direction::Right,
        }
    }

    fn get_action_from_input(&self, e: Event) -> MauriceInput {
        match e {
            Event::KeyPressed {
                code: Key::Right, ..
            } => MauriceInput::WalkingRight,
            Event::KeyReleased {
                code: Key::Right, ..
            } => MauriceInput::EndWalkingRight,
            Event::KeyPressed {
                code: Key::Left, ..
            } => MauriceInput::WalkingLeft,
            Event::KeyReleased {
                code: Key::Left, ..
            } => MauriceInput::EndWalkingLeft,
            Event::KeyPressed {
                code: Key::Down, ..
            } => MauriceInput::Crouch,
            Event::KeyReleased {
                code: Key::Down, ..
            } => MauriceInput::EndCrouch,
            Event::KeyPressed {
                code: Key::Escape, ..
            } => MauriceInput::Exit,
            Event::KeyPressed {
                code: Key::A, ..
            } => MauriceInput::Punch,
            Event::KeyPressed {
                code: Key::B, ..
            } => MauriceInput::MiddleKick,
            _ => MauriceInput::Idle,
        }
    }

    fn update_state(&mut self, e: MauriceInput) {
        match e {
            MauriceInput::WalkingLeft => {
                println!("KEY POP - LEFT");
                self.input_state.flag_move = true;
                self.input_state.direction = Direction::Left;
            }
            MauriceInput::EndWalkingLeft => {
                println!("KEY POP - LEFT rel");
                self.input_state.flag_move = false;
            }
            MauriceInput::WalkingRight => {
                println!("KEY POP - RIGHT");
                self.input_state.flag_move = true;
                self.input_state.direction = Direction::Right;
            }
            MauriceInput::EndWalkingRight => {
                println!("KEY POP - RIGHT rel");
                self.input_state.flag_move = false;
            },
            MauriceInput::Crouch => {
                println!("KEY POP - RIGHT rel");
                self.input_state.flag_crouch = true;
            },
            MauriceInput::EndCrouch => {
                println!("KEY POP - RIGHT rel");
                self.input_state.flag_crouch = false;
            },
            MauriceInput::Punch => {
                println!("KEY POP - RIGHT rel");
                self.input_state.flag_attack = true;
            },
            _ => {}
        }
    }

    fn get_running_action_from_current_state(&self) -> (MauriceRun, Direction) {
        match self.input_state {
            InputState {
                flag_crouch: true,
                flag_attack: false, ..
            } => (MauriceRun::Crouch, self.input_state.direction),
            InputState {
                flag_crouch: true,
                flag_attack: true, ..
            } => (MauriceRun::CrouchPunch, self.input_state.direction),
            InputState {
                flag_crouch: false,
                flag_move: true,
                flag_attack: false,
                ..
            } => (MauriceRun::Walking, self.input_state.direction),
            _ => (MauriceRun::Idle, self.input_state.direction),
        }
    }

    // call for frame update
    fn on_draw_frame(&mut self, window: &mut sfml::graphics::RenderWindow) {
        // update state from inputs and get current action to perform
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
                let mode = if self.current_action == MauriceRun::Idle || self.current_action == MauriceRun::Walking || self.current_action == MauriceRun::Crouch {AnimationMode::Repeated} else  {AnimationMode::OneShot};
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

}

impl<'a> Fighter for Maurice<'a> {
    fn get_name(&self) -> &str {
        &self.character.configuration.name.as_str()
    }

    fn get_speed(&self) -> sfml::system::Vector2f {
        self.character.get_speed()
    }

    fn draw(&mut self, window: &mut sfml::graphics::RenderWindow) {
        self.on_draw_frame(window);
    }

    fn process_input_event(&mut self, evt: sfml::window::Event) -> ResultEvent {
        let action = self.get_action_from_input(evt);
        let res = if action == MauriceInput::Exit {
            ResultEvent::Exit
        } else {
            ResultEvent::Solo
        };
        self.events_queue.push_back(action);
        res
    }
}
