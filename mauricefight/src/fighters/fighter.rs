use crate::common::Direction;
use crate::common::InputProcessor;
use crate::common::ResultEvent;
use crate::configuration::resources::GameResources;
use crate::fighters::fighter_crouch::FighterCrouch;
use crate::fighters::fighter_crouch::FighterEndCrouch;
use crate::fighters::fighter_input::FighterInputState;
use crate::fighters::fighter_move::FighterMove;
use crate::fighters::fighter_state::FighterState;
use crate::fighters::fighter_state::State;
use crate::fighters::fighter_waiting::FighterWaiting;
use crate::sprites::animated_sprite::AnimatedSprite;
use sfml::graphics::RenderWindow;
use sfml::system::Vector2f;
use sfml::window::Event;
use sfml::window::Key;
use std::collections::HashMap;
use super::fighter_high_kick::FighterHighKick;
use crate::fighters::fighter_punch::FighterLeftPunch;
use crate::fighters::fighter_punch::FighterRightPunch;

pub struct Fighter<'a> {
    name: String,
    previous_state: State,
    current_state: State,
    current_direction: Direction,
    states: HashMap<State, Box<dyn FighterState>>,
    sprite: AnimatedSprite<'a>,
    is_new_state: bool,
    pub selected: bool,
    input_state: FighterInputState,
}

impl<'a> Fighter<'a> {
    pub fn new(
        name: &str,
        resources: &'a GameResources,
        sprite: &str,
        x: f32,
        y: f32,
        selected: bool,
    ) -> Self {
        let mut states: HashMap<State, Box<dyn FighterState>> = HashMap::new();
        states.insert(
            State::Idle,
            Box::new(FighterWaiting::new(name, resources, Direction::Right)),
        );
        states.insert(
            State::Crouch,
            Box::new(FighterCrouch::new(name, resources, Direction::Right)),
        );
        states.insert(
            State::Move,
            Box::new(FighterMove::new(name, resources, Direction::Right)),
        );
        states.insert(
            State::EndCrouch,
            Box::new(FighterEndCrouch::new(name, resources, Direction::Right)),
        );
        states.insert(
            State::HighKick,
            Box::new(FighterHighKick::new(name, resources, Direction::Right)),
        );
        states.insert(
            State::LeftPunch,
            Box::new(FighterLeftPunch::new(name, resources, Direction::Right)),
        );
        states.insert(
            State::RightPunch,
            Box::new(FighterRightPunch::new(name, resources, Direction::Right)),
        );
        let configuration = resources.configuration.get_character(name).unwrap();
        let default_action_config = configuration.get_action_configuration("idle").unwrap();
        Fighter {
            name: name.to_string(),
            previous_state: State::Idle,
            current_state: State::Idle,
            current_direction: Direction::Right,
            states,
            sprite: AnimatedSprite::new(
                resources,
                sprite,
                configuration.sprite.size,
                configuration.sprite.display.scale,
                x,
                y,
                0.,
                0.,
                Direction::Right,
                default_action_config.sequence.index,
                default_action_config.sequence.delay,
                default_action_config.sequence.nb_frames,
            ),
            is_new_state: true,
            selected,
            input_state: FighterInputState::new(),
        }
    }

    pub fn on_frame_update(&mut self, window: &mut RenderWindow) {
        let mut _state = self.states.get_mut(&self.current_state);
        //println!("STATE {}", self.current_state);
        match _state {
            Some(s) => {
                if self.is_new_state {
                    let animation_state = s.get_animation_state();
                    self.sprite.reset_animation(
                        animation_state.sprite_index,
                        animation_state.delay, 
                        animation_state.nb_frames, 
                        self.current_direction, 
                        animation_state.speed, 
                    );
                    self.is_new_state = false;
                }
                let next_state = s.on_frame_update(&mut self.sprite, &self.input_state, window);
                if next_state.0 != self.current_state || next_state.1 != self.current_direction {
                    println!(
                        "change : state : {}, direction : {}",
                        self.current_state, self.current_direction
                    );
                    self.current_state = next_state.0;
                    self.current_direction = next_state.1;
                    self.is_new_state = true;
                }
            }
            _ => {}
        }
    }

    pub fn get_speed(&self) -> Vector2f {
        self.sprite.nav.speed.clone()
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

impl<'a> InputProcessor for Fighter<'a> {
    fn process_event(&mut self, e: sfml::window::Event) -> ResultEvent {
        match e {
            Event::KeyPressed {
                code: Key::Escape, ..
            } => ResultEvent::Menu,
            _ => {
                let input_event = self.input_state.on_input(e);
                let mut _state = self.states.get_mut(&self.current_state);
                match _state {
                    Some(s) => {
                        let action_result = s.on_event(input_event, &self.input_state);
                        if action_result.0 != self.current_state
                            || action_result.1 != self.current_direction
                        {
                            self.current_state = action_result.0;
                            self.current_direction = action_result.1;
                            println!(
                                "change : state : {}, direction : {}",
                                self.current_state, self.current_direction
                            );
                            self.is_new_state = true;
                        }
                        ResultEvent::Solo
                    }
                    _ => ResultEvent::Solo
                }
            }
        }
    }
}
