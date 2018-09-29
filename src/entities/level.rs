use assets::sprite::{ManagedSprite, Sprite};
use assets::tileparser::Tilemap;

pub fn create_level(level: Tilemap) -> i32 {
    println!("{:?}", level);
    2
}

pub struct LevelMap {
    pub level: Tilemap,
    pub sprite: ManagedSprite,
}

impl LevelMap {
    pub fn new(level: Tilemap, sprite: ManagedSprite) -> Self {
        LevelMap { level, sprite }
    }

    pub get_grid(&self) -> Vec<Sprite>{
        let height = self.level.height;
        let width = self.level.width;



    }
}

#[test]
fn it_should_create_entities_for_level() {
    use assets::tileparser::parse_tilemap;
    let path = "resources/lvl1.test.json";
    let lvl = parse_tilemap(path).unwrap();
    create_level(lvl);
    assert_eq!(3, 3);
}
