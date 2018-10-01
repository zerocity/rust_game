use serde_json::{from_str, Value};
use std::collections::HashMap;
use std::fs;
use std::io::Error;

#[derive(Debug, Deserialize)]
pub struct Image {
    pub source: String,
    pub width: i32,
    pub height: i32,
}

pub type Properties = Option<Vec<HashMap<String, Value>>>;

#[derive(Debug, Deserialize, Clone)]
pub struct Tile {
    pub id: i32,
    #[serde(rename = "type")]
    pub tile_type: Option<String>,
    pub properties: Properties,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Tileset {
    pub image: String,
    pub imageheight: i32,
    pub imagewidth: i32,
    #[serde(rename = "type")]
    pub tileset_type: Option<String>,
    pub version: Option<f32>,

    pub tiledversion: Option<String>,
    pub name: String,
    pub tilewidth: i32,
    pub tileheight: i32,
    pub tilecount: i32,
    pub columns: i32,
    pub margin: i32,
    pub spacing: i32,
    pub tiles: Option<Vec<Tile>>,
}

#[derive(Debug, Deserialize)]
pub struct Layers {
    pub data: Vec<i32>,
    pub height: i32,
    pub id: i32,
    pub name: String,
    pub opacity: i32,
    #[serde(rename = "type")]
    pub layer_type: String,
    pub visible: bool,
    pub width: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Deserialize)]
pub struct SmallTileset {
    pub firstgid: i32,
    pub source: String,
}

#[derive(Debug, Deserialize)]
pub struct Tilemap {
    pub height: i32,
    pub infinite: bool,
    pub layers: Vec<Layers>,
    pub nextlayerid: i32,
    pub nextobjectid: i32,
    pub tileheight: i32,
    pub tilesets: Vec<SmallTileset>,
    pub tilewidth: i32,
    #[serde(rename = "type")]
    pub tilemap_type: String,
    pub version: f32,
    pub width: i32,
}

pub fn parse_tileset(path: String) -> Result<Tileset, Error> {
    let contents = fs::read_to_string(path)?;
    let project: Tileset = from_str(&contents).unwrap();
    Ok(project)
}

pub fn parse_tilemap(path: &str) -> Result<Tilemap, Error> {
    let contents = fs::read_to_string(path)?;
    let project: Tilemap = from_str(&contents).unwrap();
    Ok(project)
}
