use crate::animated_sprite::AnimatedSprite;
use crate::animated_sprite::AnimationMode;
use crate::common::Direction;
use crate::configuration::CharacterConfiguration;
use crate::inputs::ResultEvent;
use sfml::graphics::Sprite;
use sfml::window::Event;
use sfml::{graphics::RenderWindow, system::Vector2f};

pub struct Character<'a> {
    pub configuration: CharacterConfiguration,
    sprite: AnimatedSprite<'a>,
    pub is_human: bool,
}

impl<'a> Character<'a> {
    pub fn new(
        character: &CharacterConfiguration,
        sprite: Sprite<'a>,
        x: f32,
        y: f32,
        default_action: &str,
        mode: AnimationMode,
        is_human: bool,
    ) -> Self {
        let default_action_config = character.get_action_configuration(default_action).unwrap();
        println!("Action : {:?}", default_action_config);
        Character {
            configuration: character.clone(),
            sprite: AnimatedSprite::new(
                sprite,
                character.sprite.size,
                character.sprite.display.scale,
                x,
                y,
                0.,
                0.,
                Direction::Right,
                default_action_config.sequence.index,
                default_action_config.sequence.delay,
                default_action_config.sequence.nb_frames,
                mode,
            ),
            is_human,
        }
    }

    pub fn start_action(
        &mut self,
        action: &str,
        mode: AnimationMode,
        direction: Direction,
        window: &mut RenderWindow,
    ) {
        println!("START ACTION");
        let action_config = self.configuration.get_action_configuration(action).unwrap();
        self.sprite.start_animation(
            action_config.sequence.index,
            action_config.sequence.delay,
            action_config.sequence.nb_frames,
            mode,
            direction,
            action_config.sequence.speed,
            window,
        );
    }

    pub fn on_draw(&mut self, window: &mut RenderWindow) -> bool {
        self.sprite.next_frame(window)
    }

    pub fn get_speed(&self) -> sfml::system::Vector2f {
        self.sprite.nav.speed
    }
}

pub trait Fighter {
    fn process_input_event(&mut self, evt: Event) -> ResultEvent;

    fn draw(&mut self, window: &mut RenderWindow);

    fn get_speed(&self) -> Vector2f;

    fn get_name(&self) -> &str;
}
