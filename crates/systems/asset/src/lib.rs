mod types;

use std::{collections::HashMap, sync::Arc};

use types::*;
use utils::Id;

pub struct Asset {
    pub asset_type: AssetType,
}

pub struct AssetManager {
    assets: HashMap<Id, Arc<Asset>>,
    db: 
}

impl AssetManager {
    pub fn new() -> Self {
        let db = 
        
        Self {
            assets: Default::default(),

        }
    }
}
