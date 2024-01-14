use crate::common::Direction;
use crate::configuration::resources::GameResources;
use crate::fighters::fighter_input::FighterInputEvent;
use crate::fighters::fighter_input::FighterInputState;
use crate::fighters::fighter_state::FighterAnimation;
use crate::fighters::fighter_state::FighterState;
use crate::fighters::fighter_state::State;
use crate::sprites::animated_sprite::AnimatedSprite;
use sfml::graphics::RenderWindow;

pub struct FighterMove {
    animation: FighterAnimation,
}

impl FighterMove {
    pub fn new(name: &str, resources: &GameResources, direction: Direction) -> Self {
        FighterMove {
            animation: FighterAnimation::new(
                resources,
                name,
                State::Move.to_string().as_str(),
                direction,
            ),
        }
    }
}

impl FighterState for FighterMove {
    fn get_animation_state(&self) -> &FighterAnimation {
        &self.animation
    }

    fn on_event(
        &mut self,
        event: FighterInputEvent,
        input_state: &FighterInputState,
    ) -> (State, Direction, bool) {
        match event {
            FighterInputEvent::EndMove => (State::Idle, input_state.direction, true),
            FighterInputEvent::Crouch => (State::Crouch, input_state.direction, true),
            FighterInputEvent::HighKick => (State::HighKick, input_state.direction, true),
            FighterInputEvent::LeftPunch => (State::LeftPunch, input_state.direction, true),
            FighterInputEvent::RightPunch => (State::RightPunch, input_state.direction, true),
            _ => (State::Move, input_state.direction, false),
        }
    }

    fn on_frame_update(
        &mut self,
        sprite: &mut AnimatedSprite,
        input_state: &FighterInputState,
        window: &mut RenderWindow,
    ) -> (State, Direction) {
        let frame_res = sprite.next_frame(input_state.direction, window);
        if frame_res.0 {
            sprite.restart_animation();
        }
        (State::Move, input_state.direction)
    }
}
