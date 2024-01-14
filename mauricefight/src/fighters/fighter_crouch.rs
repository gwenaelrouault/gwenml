use crate::common::Direction;
use crate::configuration::resources::GameResources;
use crate::fighters::fighter_input::FighterInputEvent;
use crate::fighters::fighter_input::FighterInputState;
use crate::fighters::fighter_state::FighterAnimation;
use crate::fighters::fighter_state::FighterState;
use crate::fighters::fighter_state::State;
use crate::sprites::animated_sprite::AnimatedSprite;
use sfml::graphics::RenderWindow;

pub struct FighterCrouch {
    animation: FighterAnimation,
}

impl FighterCrouch {
    pub fn new(name: &str, resources: &GameResources, direction: Direction) -> Self {
        FighterCrouch {
            animation: FighterAnimation::new(
                resources,
                name,
                State::Crouch.to_string().as_str(),
                direction,
            ),
        }
    }
}

impl FighterState for FighterCrouch {
    fn get_animation_state(&self) -> &FighterAnimation {
        &self.animation
    }

    fn on_event(
        &mut self,
        event: FighterInputEvent,
        input_state: &FighterInputState,
    ) -> (State, Direction, bool) {
        match event {
            FighterInputEvent::EndCrouch => (State::EndCrouch, input_state.direction, true),
            _ => (State::Crouch, input_state.direction, false),
        }
    }

    fn on_frame_update(
        &mut self,
        sprite: &mut AnimatedSprite,
        input_state: &FighterInputState,
        window: &mut RenderWindow,
    ) -> (State, Direction) {
        sprite.next_frame(input_state.direction, window);
        (State::Crouch, input_state.direction)
    }
}

pub struct FighterEndCrouch {
    animation: FighterAnimation,
}

impl FighterEndCrouch {
    pub fn new(name: &str, resources: &GameResources, direction: Direction) -> Self {
        FighterEndCrouch {
            animation: FighterAnimation::new(
                resources,
                name,
                State::EndCrouch.to_string().as_str(),
                direction,
            ),
        }
    }
}

impl FighterState for FighterEndCrouch {
    fn get_animation_state(&self) -> &FighterAnimation {
        &self.animation
    }

    fn on_event(
        &mut self,
        event: FighterInputEvent,
        input_state: &FighterInputState,
    ) -> (State, Direction, bool) {
        match event {
            FighterInputEvent::Crouch => (State::Crouch, input_state.direction, true),
            _ => (State::EndCrouch, input_state.direction, false),
        }
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
            (State::EndCrouch, input_state.direction)
        }
    }
}
