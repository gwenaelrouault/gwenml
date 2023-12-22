use std::collections::VecDeque;
use sfml::SfBox;
use sfml::{
    graphics::{
        IntRect, RenderTarget,
        RenderWindow, Sprite, Transformable,
    },
    system::{Clock, Vector2f}
};

#[derive(Copy, Clone, Debug)]
pub enum Action {
    Standing,
    WalkingRight,
    WalkingLeft,
    EndWalkingRight,
    EndWalkingLeft,
    Crouch,
    EndCrouch,
    Attack1,
    Attack2,
    EndAttack,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RunAction {
    Standing,
    Walking,
    Crouch,
    CrouchPunch,
    Punch,
    MiddleKick,
    HighKick,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
}
#[derive(Copy, Clone, Debug)]
pub enum ActionTempo {
    Infinite,
    Continu,
    Immediate,
}
#[derive(Copy, Clone, Debug)]
pub enum Attack {
    NoAttack,
    Punch,
    HighKick,
    MiddleKick,
}

#[derive(Copy, Clone, Debug)]
pub struct ActionDesc {
    pub name: RunAction,
    pub count: i32,
    pub sprite_index: i32,
    pub sprite_len: i32,
    pub delay: i32,
    pub speed: f32,
    pub tempo: ActionTempo,
}

impl ActionDesc {
    pub fn is_repeated(&self) -> bool {
        match self.tempo {
            ActionTempo::Infinite | ActionTempo::Continu => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for ActionDesc {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Action : {:?}, speed : {}", self.name, self.speed)
    }
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
}

#[derive(Copy, Clone)]
pub struct State {
    pub current_action: ActionDesc,
    pub step: i32,
}

impl State {
    pub fn is_done(&self) -> bool {
        self.step >= self.current_action.count - 1
    }

    pub fn new(action: &ActionDesc) -> Self {
        Self {
            current_action: action.clone(),
            step: 0,
        }
    }

    pub fn default() -> Self {
        Self {
            current_action: ActionDesc {
                name: RunAction::Standing,
                count: 4,
                sprite_index: 5,
                sprite_len: 100,
                delay: 150,
                speed: 0.,
                tempo: ActionTempo::Infinite,
            },
            step: 0,
        }
    }
}

pub struct Player<'a> {
    pub position: Vector2f,
    pub speed: Vector2f,
    pub sprite: Sprite<'a>,
    pub state: State,
    pub input_state: InputState,
    pub actions: VecDeque<Action>,
    pub clock: SfBox<Clock>,
    pub running_action: RunAction,
    pub running_direction: Direction,
    pub ko : bool,
}

impl<'a> Player<'a> {
    fn update_position(&mut self) {
        self.position.x = self.position.x + self.speed.x;
        self.position.y = self.position.y + self.speed.y;
        self.sprite.set_position(self.position);
        let x_scale = if self.running_direction == Direction::Left { -0.7 } else { 0.7 };
        self.sprite.set_scale(Vector2f::new(x_scale, 0.7));
    }

    fn is_attacking(&self) -> bool {
        self.running_action == RunAction::MiddleKick || self.running_action == RunAction::HighKick
    }

    fn get_action_desc(&mut self, action: RunAction) -> ActionDesc {
        match action {
            RunAction::Walking => ActionDesc {
                name: RunAction::Walking,
                count: 6,
                sprite_index: 0,
                sprite_len: 100,
                delay: 100,
                speed: 0.3,
                tempo: ActionTempo::Continu,
            },
            RunAction::Crouch => ActionDesc {
                name: RunAction::Crouch,
                count: 1,
                sprite_index: 20,
                sprite_len: 100,
                delay: 1000,
                speed: 0.,
                tempo: ActionTempo::Continu,
            },
            RunAction::MiddleKick => ActionDesc {
                name: RunAction::MiddleKick,
                count: 3,
                sprite_index: 9,
                sprite_len: 100,
                delay: 100,
                speed: 0.,
                tempo: ActionTempo::Immediate,
            },
            RunAction::HighKick => ActionDesc {
                name: RunAction::HighKick,
                count: 8,
                sprite_index: 24,
                sprite_len: 100,
                delay: 50,
                speed: 0.,
                tempo: ActionTempo::Immediate,
            },
            _ => ActionDesc {
                name: RunAction::Standing,
                count: 4,
                sprite_index: 5,
                sprite_len: 100,
                delay: 150,
                speed: 0.,
                tempo: ActionTempo::Infinite,
            },
        }
    }

    fn get_current_attack_from_input(&self, attack: Attack) -> (RunAction, Direction) {
        match attack {
            Attack::HighKick => (RunAction::HighKick, self.input_state.direction),
            Attack::MiddleKick => (RunAction::MiddleKick, self.input_state.direction),
            _ => (RunAction::Standing, self.input_state.direction)
        }
    }

    fn get_current_action_from_input(&mut self, action: Action) -> (RunAction, Direction) {
        println!("INPUT - get_current_action_from_input {:?}", action);
        match action {
            Action::Crouch => {
                println!("KEY POP - CROUCH");
                self.input_state.flag_crouch = true;
            }
            Action::EndCrouch => {
                println!("KEY POP - CROUCH rel");
                self.input_state.flag_crouch = false;
            }
            Action::WalkingLeft => {
                println!("KEY POP - LEFT");
                self.input_state.flag_move = true;
                self.input_state.direction = Direction::Left;
            },
            Action::EndWalkingLeft => {
                println!("KEY POP - LEFT rel");
                self.input_state.flag_move = false;
            },
            Action::WalkingRight => {
                println!("KEY POP - RIGHT");
                self.input_state.flag_move = true;
                self.input_state.direction = Direction::Right;
            },
            Action::EndWalkingRight => {
                println!("KEY POP - RIGHT rel");
                self.input_state.flag_move = false;
            },
            Action::Attack1 => {
                println!("KEY POP - ATTACK1");
                self.input_state.flag_attack = true;
                self.input_state.attack = Attack::MiddleKick;
            },
            Action::Attack2 => {
                println!("KEY POP - ATTACK1");
                self.input_state.flag_attack = true;
                self.input_state.attack = Attack::HighKick;
            },
            Action::EndAttack => {
                self.input_state.flag_attack = false;
                self.input_state.attack = Attack::NoAttack;
            }
            _ => {}
        }
        match self.input_state {
            InputState {
                flag_crouch: true, ..
            } => (RunAction::Crouch, self.input_state.direction),
            InputState {
                flag_crouch: false,
                flag_move: true,
                flag_attack : false,
                ..
            } => (RunAction::Walking, self.input_state.direction),
            InputState {
                flag_crouch: false,
                flag_attack : true,
                ..
            } => self.get_current_attack_from_input(self.input_state.attack),
            _ => (RunAction::Standing, self.input_state.direction),
        }
    }

    fn update_sprite(&mut self) {
        let current_player_sprite_rect = IntRect::new(
            (self.state.current_action.sprite_index + self.state.step)
                * self.state.current_action.sprite_len,
            0,
            self.state.current_action.sprite_len,
            self.state.current_action.sprite_len,
        );
        self.sprite.set_texture_rect(current_player_sprite_rect);
    }

    fn on_closed_current_action(&mut self) {
        if self.is_attacking() {
            self.do_something(Action::EndAttack);
        }
    }

    fn update_sprite_sequence(&mut self, new_sequence : bool) -> bool{
        let mut is_closed_current_action = false;
        if !new_sequence {        
            if self.clock.elapsed_time().as_milliseconds() >= self.state.current_action.delay {
                if self.state.is_done() && self.state.current_action.is_repeated() {
                    self.state.step = 0;
                } 
                else if !self.state.is_done() {
                     self.state.step = self.state.step + 1;
                }
                else {
                    println!("END ACTION {:?}", self.state.current_action);
                    is_closed_current_action = true;
                    self.on_closed_current_action();
                }
                self.clock.restart();    
            }
        }
        is_closed_current_action
    }

    fn perform_action(&mut self) {
        let mut all_input_processed = false;
        while !all_input_processed {
            let to_perform = match self.actions.pop_front() {
                Some(action) => {
                    let action_to_perform = self.get_current_action_from_input(action);
                    println!("action {:?}", action_to_perform);
                    action_to_perform
                }
                None => {
                    all_input_processed = true;
                    (self.running_action.clone(), self.running_direction.clone())
                }
            };
            match to_perform {
                (_r, _d) if self.running_action != _r || self.running_direction != _d => {
                    self.running_direction = _d;
                    self.running_action = _r;
                    let desc = self.get_action_desc(self.running_action);
                    println!("NEW Action {}", desc);
                    self.state = State::new(&desc);
                    self.speed.x = match self.running_direction {
                        Direction::Left => -1. * self.state.current_action.speed,
                        _ => self.state.current_action.speed,
                    };
                    self.update_sprite_sequence(true);
                }
                _ => {
                    println!("CURRENT Action {:?}", self.running_action);
                    self.update_sprite_sequence(false);
                }
            }
        }
    }

    pub fn draw(&mut self, window: &mut RenderWindow) {
        self.perform_action();
        self.update_sprite();
        self.update_position();
        window.draw(&self.sprite);
    }

    pub fn do_something(&mut self, action: Action) {
        self.actions.push_back(action);
    }
}
