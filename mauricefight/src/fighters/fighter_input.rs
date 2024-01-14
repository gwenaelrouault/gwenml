use std::fmt;
use sfml::window::Event;
use crate::common::Direction;
use sfml::window::Key;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum FighterInputEvent {
    Nothing,
    Move,
    EndMove,
    Crouch,
    EndCrouch,
    LeftPunch,
    RightPunch,
    HighKick,
}

impl fmt::Display for FighterInputEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FighterInputEvent::Move => write!(f, "WalkingEvent"),
            FighterInputEvent::EndMove => write!(f, "EndWalkingEvent"),
            FighterInputEvent::Crouch => write!(f, "CrouchEvent"),
            FighterInputEvent::EndCrouch => write!(f, "EndCrouchEvent"),
            FighterInputEvent::HighKick => write!(f, "HighKickEvent"),
            FighterInputEvent::LeftPunch => write!(f, "LeftPunchEvent"),
            FighterInputEvent::RightPunch => write!(f, "RightPunchEvent"),
            _ => write!(f, "NO_EVENT"),
        }
    }
}

pub struct FighterInputState {
    pub direction : Direction,
    pub right_move : bool,
    pub left_move : bool,
    pub crouch : bool,
}

impl FighterInputState {
    pub fn new() -> Self {
        FighterInputState {
            direction : Direction::Right,
            right_move : false,
            left_move : false,
            crouch : false,
        }
    }

    pub fn on_input(&mut self, e: sfml::window::Event) -> FighterInputEvent {
        match e {
            Event::KeyPressed { code: Key::A, .. } => {
                FighterInputEvent::HighKick
            }
            Event::KeyPressed { code: Key::B, .. } => {
                FighterInputEvent::LeftPunch
            }
            Event::KeyPressed { code: Key::C, .. } => {
                FighterInputEvent::RightPunch
            }
            Event::KeyPressed {
                code: Key::Right, ..
            } => {
                self.right_move = true;
                self.direction = Direction::Right;
                FighterInputEvent::Move
            }
            Event::KeyReleased {
                code: Key::Right, ..
            } => {
                self.right_move = false;
                FighterInputEvent::EndMove
            }
            Event::KeyPressed {
                code: Key::Left, ..
            } => {
                self.left_move = true;
                self.direction = Direction::Left;
                FighterInputEvent::Move
            }
            Event::KeyReleased {
                code: Key::Left, ..
            } => {
                self.left_move = false;
                FighterInputEvent::EndMove
            }
            Event::KeyPressed {
                code: Key::Down, ..
            } => {
                self.crouch = true;
                FighterInputEvent::Crouch
            }
            Event::KeyReleased {
                code: Key::Down, ..
            } => {
                self.crouch = false;
                FighterInputEvent::EndCrouch
            }
            _ => FighterInputEvent::Nothing
        }
    }
}

