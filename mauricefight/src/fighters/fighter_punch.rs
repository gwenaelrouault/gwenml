use crate::common::Direction;
use crate::configuration::resources::GameResources;
use crate::fighters::fighter_input::FighterInputEvent;
use crate::fighters::fighter_input::FighterInputState;
use crate::fighters::fighter_state::FighterAnimation;
use crate::fighters::fighter_state::FighterState;
use crate::fighters::fighter_state::State;
use crate::sprites::animated_sprite::AnimatedSprite;
use sfml::graphics::RenderWindow;

pub struct FighterLeftPunch {
    animation: FighterAnimation,
}

impl FighterLeftPunch {
    pub fn new(name: &str, resources: &GameResources, direction: Direction) -> Self {
        FighterLeftPunch {
            animation: FighterAnimation::new(
                resources,
                name,
                State::LeftPunch.to_string().as_str(),
                direction,
            ),
        }
    }
}

impl FighterState for FighterLeftPunch {

    fn get_animation_state(&self) -> &FighterAnimation {
        &self.animation
    }

    fn on_event(
        &mut self,
        event: super::fighter_input::FighterInputEvent,
        input_state: &FighterInputState,
    ) -> (State, Direction, bool) {
        (State::LeftPunch, input_state.direction, false)
    }

    fn on_frame_update(
        &mut self,
        sprite: &mut AnimatedSprite,
        input_state: &FighterInputState,
        window: &mut RenderWindow,
    ) -> (State, Direction) {
        //println!("FRAME UPDATE");
        let frame_res = sprite.next_frame(input_state.direction, window);
        if frame_res.0 {
            println!("----IDLE !");
            (State::Idle, input_state.direction)
        } else {
            println!("----END CROUCHE !");
            (State::LeftPunch, input_state.direction)
        }
    }
}

pub struct FighterRightPunch {
    animation: FighterAnimation,
}

impl FighterRightPunch {
    pub fn new(name: &str, resources: &GameResources, direction: Direction) -> Self {
        FighterRightPunch {
            animation: FighterAnimation::new(
                resources,
                name,
                State::RightPunch.to_string().as_str(),
                direction,
            ),
        }
    }
}

impl FighterState for FighterRightPunch {
    fn get_animation_state(&self) -> &FighterAnimation {
        &self.animation
    }

    fn on_event(
        &mut self,
        event: FighterInputEvent,
        input_state: &FighterInputState,
    ) -> (State, Direction, bool) {
        (State::LeftPunch, input_state.direction, true)
    }

    fn on_frame_update(
        &mut self,
        sprite: &mut AnimatedSprite,
        input_state: &FighterInputState,
        window: &mut RenderWindow,
    ) -> (State, Direction) {
        //println!("FRAME UPDATE");
        let frame_res = sprite.next_frame(input_state.direction, window);
        if frame_res.0 {
            println!("----IDLE !");
            (State::Idle, input_state.direction)
        } else {
            println!("----END CROUCHE !");
            (State::LeftPunch, input_state.direction)
        }
    }
}
