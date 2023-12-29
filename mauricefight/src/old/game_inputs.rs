use crate::game_common::Direction;
use crate::game_events::{Attack,RunAction,FighterEvent};
use sfml::window::Event;


pub trait InputProcessor {
    fn process_event(&mut self, e: Event) -> bool;
}

#[derive(Copy, Clone, Debug)]
pub struct InputState {
    pub flag_crouch: bool,
    pub flag_move: bool,
    pub direction: Direction,
    pub flag_attack : bool,
    pub attack : Attack,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            flag_crouch: false,
            flag_move: false,
            direction: Direction::Right,
            flag_attack : false,
            attack : Attack::NoAttack,
        }
    }

    fn get_current_attack_from_input(&self, attack: Attack) -> (RunAction, Direction) {
        match attack {
            Attack::HighKick => (RunAction::HighKick, self.direction),
            Attack::MiddleKick => (RunAction::MiddleKick, self.direction),
            _ => (RunAction::Standing, self.direction)
        }
    }

    pub fn get_current_action_from_input(&mut self, evt: FighterEvent) -> (RunAction, Direction) {
        println!("INPUT - get_current_action_from_input {:?}", evt);
        match evt {
            FighterEvent::Crouch => {
                println!("KEY POP - CROUCH");
                self.flag_crouch = true;
            }
            FighterEvent::EndCrouch => {
                println!("KEY POP - CROUCH rel");
                self.flag_crouch = false;
            }
            FighterEvent::WalkingLeft => {
                println!("KEY POP - LEFT");
                self.flag_move = true;
                self.direction = Direction::Left;
            },
            FighterEvent::EndWalkingLeft => {
                println!("KEY POP - LEFT rel");
                self.flag_move = false;
            },
            FighterEvent::WalkingRight => {
                println!("KEY POP - RIGHT");
                self.flag_move = true;
                self.direction = Direction::Right;
            },
            FighterEvent::EndWalkingRight => {
                println!("KEY POP - RIGHT rel");
                self.flag_move = false;
            },
            FighterEvent::Attack1 => {
                println!("KEY POP - ATTACK1");
                self.flag_attack = true;
                self.attack = Attack::MiddleKick;
            },
            FighterEvent::Attack2 => {
                println!("KEY POP - ATTACK2");
                self.flag_attack = true;
                self.attack = Attack::HighKick;
            },
            FighterEvent::EndAttack => {
                println!("KEY POP - END ATTACK");
                self.flag_attack = false;
                self.attack = Attack::NoAttack;
            }
            _ => {}
        }
        match self {
            InputState {
                flag_crouch: true, ..
            } => (RunAction::Crouch, self.direction),
            InputState {
                flag_crouch: false,
                flag_move: true,
                flag_attack : false,
                ..
            } => (RunAction::Walking, self.direction),
            InputState {
                flag_crouch: false,
                flag_attack : true,
                ..
            } => self.get_current_attack_from_input(self.attack),
            _ => (RunAction::Standing, self.direction),
        }
    }

}