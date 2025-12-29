// Auto-generated module declarations


use std::sync::OnceLock;

pub struct GameDB {
}

impl GameDB {
    pub fn load(data_dir: &str) -> anyhow::Result<Self> {

        Ok(Self {
        })
    }

    pub fn global() -> &'static GameDB {
        static DB: OnceLock<GameDB> = OnceLock::new();
        DB.get_or_init(|| {
            Self::load("data").expect("Failed to load game database")
        })
    }
}

static GAME_DATA: OnceLock<GameDB> = OnceLock::new();

pub fn init(data_dir: &str) -> anyhow::Result<()> {
    let db = GameDB::load(data_dir)?;
    GAME_DATA.set(db)
        .map_err(|_| anyhow::anyhow!("Game data already initialized"))
}

#[inline]
pub fn get() -> &'static GameDB {
    GAME_DATA.get().expect("Game data not initialized. Call init() first.")
}

#[inline]
pub fn try_get() -> Option<&'static GameDB> {
    GAME_DATA.get()
}