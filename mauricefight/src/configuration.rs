use serde_derive::Deserialize;

#[derive(Deserialize,Debug,Clone)]
pub struct SpriteSequenceConfiguration {
    pub index : i32,
    pub nb_frames : i32,
    pub delay : i32,
    pub speed : f32,
}

#[derive(Deserialize,Debug,Clone)]
pub struct DisplayConfiguration {
    pub scale : f32,
    pub x_origin : f32,
    pub y_origin : f32,
}

#[derive(Deserialize,Debug,Clone)]
pub struct SpriteConfiguration {
    pub img : String,
    pub nb_frames : i32,
    pub size : i32,
    pub display : DisplayConfiguration,
}

#[derive(Deserialize,Debug,Clone)]
pub struct CursorConfiguration {
    pub sprite : SpriteConfiguration,
    pub delay : i32,
}

#[derive(Deserialize,Debug,Clone)]
pub struct FontsConfiguration {
    pub sprite : SpriteConfiguration,
}

#[derive(Deserialize,Debug, Clone)]
pub struct GuiConfiguration {
    pub cursor : CursorConfiguration,
    pub fonts : FontsConfiguration,
}

#[derive(Deserialize,Debug,Clone)]
pub struct LevelConfiguration {
    pub name : String,
    pub sprite : SpriteConfiguration,
}

#[derive(Deserialize,Debug,Clone)]
pub struct ActionConfiguration {
    pub name : String,
    pub sequence : SpriteSequenceConfiguration,
}

impl std::fmt::Display for ActionConfiguration {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Action : {:?}, ", self.name)
    }
}


#[derive(Deserialize,Debug, Clone)]
pub struct CharacterConfiguration {
    pub name : String,
    pub sprite : SpriteConfiguration,
    pub actions : Vec<ActionConfiguration>,
}

impl CharacterConfiguration {
    pub fn get_action_configuration(&self, name : &str) -> Option<&ActionConfiguration> {
        self.actions.iter().find(|x| x.name.eq_ignore_ascii_case(name))
    }
}

#[derive(Deserialize,Debug)]
pub struct TexturesConfiguration {
    pub sprite : SpriteConfiguration,
}

#[derive(Deserialize,Debug)]
pub struct Configuration {
    pub levels : Vec<LevelConfiguration>,
    pub characters : Vec<CharacterConfiguration>,
    pub gui : GuiConfiguration,
    pub textures : TexturesConfiguration,
}
