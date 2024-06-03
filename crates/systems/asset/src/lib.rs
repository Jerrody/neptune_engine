mod types;

use std::{collections::HashMap, sync::Arc};

use id::Id;
use surrealdb::{
    engine::local::{Db, SpeeDb}, method::UseDb, Surreal
};
use types::*;

pub struct Asset {
    pub asset_type: AssetType,
}

pub struct AssetManager<'a> {
    assets: HashMap<Id, Arc<Asset>>,
    db: UseDb<'a, Db>,
}

impl<'a> AssetManager<'a> {
    const DEFAULT_NAMESPACE: &'static str = "default_namespace";
    const EDITOR_DATABASE: &'static str = "default_database";

    pub async fn new() -> Self {
        let db = Surreal::new::<SpeeDb>("../../../../").await.unwrap();
        let db = db.use_ns(Self::DEFAULT_NAMESPACE)
            .use_db(Self::EDITOR_DATABASE).into_owned();

        Self {
            assets: Default::default(),
            db,
        }
    }
}
