use crate::common::Direction;
use crate::configuration::resources::GameResources;
use crate::fighters::fighter_input::FighterInputEvent;
use crate::fighters::fighter_input::FighterInputState;
use crate::fighters::fighter_state::FighterAnimation;
use crate::fighters::fighter_state::FighterState;
use crate::fighters::fighter_state::State;
use crate::sprites::animated_sprite::AnimatedSprite;
use sfml::graphics::RenderWindow;

pub struct FighterHighKick {
    animation: FighterAnimation,
}

impl FighterHighKick {
    pub fn new(name: &str, resources: &GameResources, direction: Direction) -> Self {
        FighterHighKick {
            animation: FighterAnimation::new(
                resources,
                name,
                State::HighKick.to_string().as_str(),
                direction,
            ),
        }
    }
}

impl FighterState for FighterHighKick {
    fn get_animation_state(&self) -> &FighterAnimation {
        &self.animation
    }

    fn on_event(
        &mut self,
        event: FighterInputEvent,
        input_state: &FighterInputState,
    ) -> (State, Direction, bool) {
        (State::HighKick, input_state.direction, true)
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
            (State::HighKick, input_state.direction)
        }
    }
}
