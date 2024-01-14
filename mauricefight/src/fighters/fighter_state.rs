use crate::common::Direction;
use crate::fighters::fighter_input::FighterInputEvent;
use crate::fighters::fighter_input::FighterInputState;
use crate::configuration::resources::GameResources;
use crate::sprites::animated_sprite::AnimatedSprite;
use sfml::graphics::RenderWindow;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum State {
    Idle,
    Crouch,
    EndCrouch,
    Move,
    HighKick,
    LeftPunch,
    RightPunch,
    Ko,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::Move => write!(f, "walking"),
            State::Crouch => write!(f, "crouch"),
            State::EndCrouch => write!(f, "endcrouch"),
            State::HighKick => write!(f, "highkick"),
            State::LeftPunch => write!(f, "leftpunch"),
            State::RightPunch => write!(f, "rightpunch"),
            _ => write!(f, "idle"),
        }
    }
}

pub struct FighterAnimation {
    pub sprite_index: i32,
    pub nb_frames: i32,
    pub delay: i32,
    pub speed: f32,
}

impl FighterAnimation {
    pub fn new(
        resources: &GameResources,
        name: &str,
        action_name: &str,
        direction: Direction,
    ) -> Self {
        println!("perso : {}, action :{}", name, action_name);
        let action_config = resources
            .configuration
            .get_character(name)
            .unwrap()
            .get_action_configuration(action_name)
            .unwrap();
        FighterAnimation {
            sprite_index: action_config.sequence.index,
            nb_frames: action_config.sequence.nb_frames,
            delay: action_config.sequence.delay,
            speed: action_config.sequence.speed,
        }
    }
}

pub trait FighterState {
    fn on_event(&mut self, event : FighterInputEvent, input_state : &FighterInputState) -> (State, Direction, bool);

    fn get_animation_state(&self) -> &FighterAnimation;

    fn on_frame_update(
        &mut self,
        sprite: &mut AnimatedSprite, 
        input_state : &FighterInputState,
        window: &mut RenderWindow,
    ) -> (State, Direction);
}
