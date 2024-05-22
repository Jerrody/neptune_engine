use utils::Id;

pub struct Texture {
    pub id: Id,
    pub data: Vec<u8>,
}

pub struct Model {
    pub id: Id,
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
}

pub struct Sound {
    pub id: Id,
    pub data: Vec<u8>,
}

pub struct Script {
    pub id: Id,
    pub code: String,
}

pub enum AssetType {
    Texture(Texture),
    Model(Model),
    Sound(Sound),
    Script(Script),
}
