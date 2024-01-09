use crate::sprites::animated_sprite::AnimatedSprite;
use crate::sprites::animated_sprite::AnimationMode;
use crate::common::Direction;
use crate::configuration::configuration::ActionConfiguration;
use crate::configuration::configuration::CharacterConfiguration;
use crate::inputs::ResultEvent;
use sfml::graphics::FloatRect;
use sfml::graphics::RectangleShape;
use sfml::graphics::RenderStates;
use sfml::graphics::RenderTarget;
use sfml::graphics::Sprite;
use sfml::window::Event;
use sfml::{graphics::RenderWindow, system::Vector2f};

struct HitBox {
    body: FloatRect,
    head: FloatRect,
    legs: FloatRect,
}

impl HitBox {
    pub fn update(&mut self, config: &ActionConfiguration, frame: usize, position : &Vector2f) {
        println!("ACTION_CONFIG : {:?}", config);
        self.body.left = config.sequence.frames[frame].body.left + position.x;
        self.body.top = config.sequence.frames[frame].body.top + position.y;
        self.body.width = config.sequence.frames[frame].body.width;
        self.body.height = config.sequence.frames[frame].body.height;

        self.head.left = config.sequence.frames[frame].head.left + position.x;
        self.head.top = config.sequence.frames[frame].head.top + position.y;
        self.head.width = config.sequence.frames[frame].head.width;
        self.head.height = config.sequence.frames[frame].head.height;

        self.legs.left = config.sequence.frames[frame].legs.left + position.x;
        self.legs.top = config.sequence.frames[frame].legs.top + position.y;
        self.legs.width = config.sequence.frames[frame].legs.width;
        self.legs.height = config.sequence.frames[frame].legs.height;
    }

    pub fn new(config: &ActionConfiguration, x :f32, y : f32) -> Self {
        HitBox {
            body: FloatRect::new(
                config.sequence.frames[0].body.left + x,
                config.sequence.frames[0].body.top + y,
                config.sequence.frames[0].body.width,
                config.sequence.frames[0].body.height,
            ),
            head: FloatRect::new(
                config.sequence.frames[0].head.left + x,
                config.sequence.frames[0].head.top + y,
                config.sequence.frames[0].head.width,
                config.sequence.frames[0].head.height,
            ),
            legs: FloatRect::new(
                config.sequence.frames[0].legs.left + x,
                config.sequence.frames[0].legs.top + y,
                config.sequence.frames[0].legs.width,
                config.sequence.frames[0].legs.height,
            ),
        }
    }
    pub fn draw(&mut self, window: &mut RenderWindow) {
        let hitbox_body = RectangleShape::from_rect(self.body);
        let hitbox_head = RectangleShape::from_rect(self.head);
        let hitbox_legs = RectangleShape::from_rect(self.legs);
        window.draw_rectangle_shape(&hitbox_body, &RenderStates::DEFAULT);
        window.draw_rectangle_shape(&hitbox_head, &RenderStates::DEFAULT);
        window.draw_rectangle_shape(&hitbox_legs, &RenderStates::DEFAULT);
    }

}

pub struct Character {
    pub configuration: CharacterConfiguration,
    current_action_configuration: ActionConfiguration,
    sprite: AnimatedSprite,
    pub is_human: bool,
    hit_box: HitBox,
}

impl Character {
    pub fn new(
        character: &CharacterConfiguration,
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
            current_action_configuration: default_action_config.clone(),
            sprite: AnimatedSprite::new(
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
            hit_box: HitBox::new(default_action_config, x, y),
        }
    }

    pub fn start_action(
        &mut self,
        action: &str,
        mode: AnimationMode,
        direction: Direction,
        window: &mut RenderWindow,
    ) {
        println!("===========>START ACTION {}", action);
        self.current_action_configuration = self.configuration.get_action_configuration(action).unwrap().clone();
        
        self.hit_box.update(&self.current_action_configuration, 0, &self.sprite.nav.position);
        self.sprite.start_animation(
            self.current_action_configuration.sequence.index,
            self.current_action_configuration.sequence.delay,
            self.current_action_configuration.sequence.nb_frames,
            mode,
            direction,
            self.current_action_configuration.sequence.speed,
            window,
        );
    }

    pub fn on_draw(&mut self, window: &mut RenderWindow) -> bool {
        let next_frame_result = self.sprite.next_frame(window);
        let frame_index = next_frame_result.1 as usize;
        println!("HITBOX {}", frame_index);
        self.hit_box.update(&self.current_action_configuration, frame_index, &self.sprite.nav.position);
        //self.hit_box.draw(window);
        next_frame_result.0
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
