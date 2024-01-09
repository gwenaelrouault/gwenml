use crate::common::Direction;
use crate::configuration::resources::GameResources;
use crate::common::InputProcessor;
use crate::common::ResultEvent;
use crate::sprites::animated_sprite::AnimatedSprite;
use sfml::graphics::RenderWindow;
use sfml::system::Vector2f;
use sfml::window::Event;
use sfml::window::Key;
use std::collections::HashMap;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum State {
    Idle,
    Crouch,
    Move,
    Ko,
}

pub struct FighterAnimation {
    pub sprite_index: i32,
    pub nb_frames: i32,
    pub delay: i32,
    pub direction: Direction,
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
            direction,
            speed: action_config.sequence.speed,
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::Move => write!(f, "walking"),
            State::Crouch => write!(f, "crouch"),
            _ => write!(f, "idle"),
        }
    }
}

trait FighterState {
    fn on_move(&mut self, direction: Direction) -> (State, Direction);

    fn on_end_move(&mut self) -> (State, Direction);

    fn on_frame_update(
        &mut self,
        sprite: &mut AnimatedSprite,
        window: &mut RenderWindow,
    ) -> (State, Direction);

    fn on_initialize(&mut self, sprite: &mut AnimatedSprite, direction: Direction);
}

struct FighterWaiting {
    animation: FighterAnimation,
}

impl FighterWaiting {
    pub fn new(name: &str, resources: &GameResources, direction: Direction) -> Self {
        FighterWaiting {
            animation: FighterAnimation::new(
                resources,
                name,
                State::Idle.to_string().as_str(),
                direction,
            ),
        }
    }
}

struct FighterCrouch {
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

struct FighterMove {
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

impl FighterState for FighterWaiting {
    fn on_move(&mut self, direction: Direction) -> (State, Direction) {
        (State::Move, direction)
    }

    fn on_end_move(&mut self) -> (State, Direction) {
        (State::Idle, self.animation.direction)
    }

    fn on_frame_update(&mut self, sprite: &mut AnimatedSprite, window: &mut RenderWindow) -> (State, Direction) {
        //println!("FRAME UPDATE");
        let frame_res = sprite.next_frame(self.animation.direction, window);
        if frame_res.0 {
            sprite.restart_animation();
        }
        (State::Idle, self.animation.direction)
    }

    fn on_initialize(&mut self, sprite: &mut AnimatedSprite, direction: Direction) {
        self.animation.direction = direction;
        sprite.reset_animation(
            self.animation.sprite_index,
            self.animation.delay,
            self.animation.nb_frames,
            direction,
            self.animation.speed,
        );
    }
}

impl FighterState for FighterMove {
    fn on_move(&mut self, direction: Direction) -> (State, Direction) {
        self.animation.direction = direction;
        (State::Move, self.animation.direction)
    }

    fn on_end_move(&mut self) -> (State, Direction) {
        (State::Idle, self.animation.direction)
    }

    fn on_frame_update(&mut self, sprite: &mut AnimatedSprite, window: &mut RenderWindow) -> (State, Direction) {
        let frame_res = sprite.next_frame(self.animation.direction, window);
        if frame_res.0 {
            sprite.restart_animation();
        }
        (State::Move, self.animation.direction)
    }

    fn on_initialize(&mut self, sprite: &mut AnimatedSprite, direction: Direction) {
        self.animation.direction = direction;
        sprite.reset_animation(
            self.animation.sprite_index,
            self.animation.delay,
            self.animation.nb_frames,
            self.animation.direction,
            self.animation.speed,
        );
    }
}

impl FighterState for FighterCrouch {
    fn on_move(&mut self, direction: Direction) -> (State, Direction) {
        self.animation.direction = direction;
        (State::Crouch, self.animation.direction)
    }

    fn on_end_move(&mut self) -> (State, Direction) {
        (State::Crouch, self.animation.direction)
    }

    fn on_frame_update(&mut self, sprite: &mut AnimatedSprite, window: &mut RenderWindow) -> (State, Direction) {
        let mut state = State::Crouch;
        let frame_res = sprite.next_frame(self.animation.direction, window);
        if frame_res.0 {
            state = State::Idle;
        }
        (state, self.animation.direction)
    }

    fn on_initialize(&mut self, sprite: &mut AnimatedSprite, direction: Direction) {
        self.animation.direction = direction;
        sprite.reset_animation(
            self.animation.sprite_index,
            self.animation.delay,
            self.animation.nb_frames,
            direction,
            self.animation.speed,
        );
    }
}

pub struct Fighter<'a> {
    name: String,
    previous_state: State,
    current_state: State,
    current_direction: Direction,
    states: HashMap<State, Box<dyn FighterState>>,
    sprite: AnimatedSprite<'a>,
    is_new_state: bool,
    pub selected: bool,
}

impl<'a> Fighter<'a> {
    pub fn new(name: &str, resources: &'a GameResources, sprite: &str, x: f32, y: f32, selected: bool) -> Self {
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
        }
    }

    pub fn on_frame_update(&mut self, window: &mut RenderWindow) {
        let mut _state = self.states.get_mut(&self.current_state);
        //println!("STATE {}", self.current_state);
        match _state {
            Some(s) => {
                if self.is_new_state {    
                    s.on_initialize(&mut self.sprite, self.current_direction);
                    self.is_new_state = false;
                }
                let next_state = s.on_frame_update(&mut self.sprite, window);
                if next_state.0 != self.current_state || next_state.1 != self.current_direction {
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

    fn consume_input(&mut self, e: sfml::window::Event)  -> (State, Direction) {
        let mut _state = self.states.get_mut(&self.current_state);
        match _state {
            Some(s) => {
                match e {
                    Event::KeyPressed {
                        code: Key::Right, ..
                    } => s.on_move(Direction::Right),
                    Event::KeyReleased {
                        code: Key::Right, ..
                    } => s.on_end_move(),
                    Event::KeyPressed {
                        code: Key::Left, ..
                    } => s.on_move(Direction::Left),
                    Event::KeyReleased {
                        code: Key::Left, ..
                    } => s.on_end_move(),
                    Event::KeyPressed {
                        code: Key::Down, ..
                    } => s.on_move(Direction::Right),
                    Event::KeyReleased {
                        code: Key::Down, ..
                    } => s.on_end_move(),
                    _ => (self.current_state,self.current_direction)
                }
            }
            _ => (self.current_state,self.current_direction)
        }

    }
}

impl<'a> InputProcessor for Fighter<'a> {
    fn process_event(&mut self, e: sfml::window::Event) -> ResultEvent {
        match e {
            Event::KeyPressed {
                code: Key::Escape, ..
            } => ResultEvent::Menu,
            _ => {
                let action_result = self.consume_input(e);
                if action_result.0 != self.current_state || action_result.1 != self.current_direction {
                    self.current_state = action_result.0;
                    self.current_direction = action_result.1;
                    println!("change : direction : {}", self.current_direction);
                    self.is_new_state = true;
                }
                ResultEvent::Solo
            }
        }        
    }
}
