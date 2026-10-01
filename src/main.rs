mod gui;
mod player;
mod assets;
mod const_def;
mod pnj_behavs;

use crate::assets::*;
use crate::const_def::*;
use crate::player::Entity;
use crate::gui::run_gui;
use std::{fs::File, io::Read};
use std::collections::HashMap;
use serde::{Deserialize};
use macroquad::prelude::*;
use std::time::Instant;

#[derive(Debug, Deserialize)]
struct Game {
	world: World,
}

#[derive(Debug, Deserialize)]
struct World {
	locations: HashMap<String, Location>,
}

#[derive(Debug, Deserialize)]
struct Location {
	name: String,
	description: String,
	exits: HashMap<String, String>,

	#[serde(default)]
	spawns: Vec<Spawn>,

	#[serde(default)]
	items: Vec<String>,

	seed: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Spawn {
    npc_type: String,

    count: u32,
}

fn fill_walls(map_objects: &mut Vec<Vec<Vec<Tile>>>, map_walls: &Vec<String>) {
    let mut line: usize = 0;
    let max_rows: usize = map_walls.len();
    let max_cols: usize = map_walls
        .iter()
        .map(|row| row.split_whitespace().count())
        .max()
        .unwrap_or(0);
    for row_str in map_walls {
        let mut col: usize = 0;
        let mut walls_row: Vec<Vec<Tile>> = vec![];
        for token in row_str.split_whitespace() {
            let mut cell_infos: Vec<Tile> = vec![];
            for index in token.split(',') {
                let tile_wall = Tile::str_to_tile(index);
                if tile_wall != Tile::Empty {
                    cell_infos.push(tile_wall)
                }
            }
            walls_row.push(cell_infos);
            col += 1;
        }
        while col < max_cols { 
            walls_row.push(vec![Tile::Void]);
            col += 1; 
        }
        
        map_objects.push(walls_row);
        line += 1; 
    }
    while line < max_rows {
        map_objects.push(vec![vec![Tile::Void]; max_cols]);
        line += 1;
    }
}

async fn add_entity(nom: String, coord: (f32, f32),
                    asset_path: &str, asset_player: Characters,
                    entities:&mut Vec<(Entity, (Texture2D, (f32, f32)))>,
                    playable: bool, hp: i32) {
    let asset_coord: (f32, f32) = asset_player.coord();
    let joueur: Entity = Entity::nouvelle(nom, coord, playable, hp);
    let joueur_asset: Texture2D = load_texture(asset_path).await.unwrap();
    joueur_asset.set_filter(FilterMode::Nearest);
    entities.push((joueur, (joueur_asset, asset_coord)))
}

fn window_conf() -> Conf {
    Conf {
        window_title: "T-A-P".to_string(),
        fullscreen: true,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut fichier: File = File::open("world.yaml")?;
    let mut x: String = String::new();
    fichier.read_to_string(&mut x)?;

 
    let config: Game = yaml_serde::from_str(&x)?;
    let mut coord: (f32, f32) = (1.0, 4.0);
	let mut player: Entity = Entity::nouvelle("GigaChad".to_string(), coord, true, 20);
	let asset_coord: (f32, f32) = Characters::KnightKnife.coord();
	let player_asset: Texture2D = load_texture("assets/player.png").await.unwrap();
    player_asset.set_filter(FilterMode::Nearest);


    let mut current_location_name: String = "start".to_string();
    let mut ran: u8 = 8;

    loop {
        let location: &Location = match config.world.locations.get(&current_location_name) {
            Some(location) => location,
            None => {
                eprintln!("Location inconnue : {}", current_location_name);
                break;
            }
        };
		let mut chats: Vec<(String, Instant)> = vec![(String::new(), Instant::now())];
		let mut map_objects: Vec<Vec<Vec<Tile>>> = vec![];
        fill_walls(&mut map_objects, &location.seed);
        let (x, y) = player.get_coord();
        match ran {
            NORTH => {
                player.set_coord((x, (map_objects.len() - 2) as f32));
                println!("exited north from ({},{}), arrived in {:#?}", x,y,player.get_coord())
            }
            SOUTH => {
                player.set_coord((x, 1.0));
            }
            WEST => {
                player.set_coord(((map_objects[0].len() - 2) as f32, y));
            }
            EAST => {
                player.set_coord((1.0, y));
            }
            _ => {
                player.set_coord((3.0, 4.0));
            }
        }
		let mut entities: Vec<(Entity, (Texture2D, (f32, f32)))> = vec![];
		add_entity("Bob".to_string(), (5.0, 6.0), "assets/player.png", Characters::PriestStick, &mut entities, false, 20).await;
		add_entity("Vampire".to_string(), (26.0, 9.0), "assets/player.png", Characters::VampireHair, &mut entities, false, 20).await;
		add_entity("Death".to_string(), (1.0, 16.0), "assets/player.png", Characters::SquelettonSickle, &mut entities, false, 20).await;
		
        
        println!("row {}    col {}", map_objects.len(), map_objects[0].len());

		(ran, coord) = run_gui((&mut player, (player_asset.clone(), asset_coord)),
                               &mut chats, &mut entities, map_objects).await;
        println!("{}", ran);
		match ran {
            NORTH => {
                if let Some(next_location) = location.exits.get("north") {
                    current_location_name = next_location.clone();
                }
            }
            SOUTH => {
                if let Some(next_location) = location.exits.get("south") {
                    current_location_name = next_location.clone();
                }
            }
            EAST => {
                if let Some(next_location) = location.exits.get("east") {
                    current_location_name = next_location.clone();
                }
            }
            WEST => {
                if let Some(next_location) = location.exits.get("west") {
                    current_location_name = next_location.clone();
                }
            }
            QUIT => {
                return Ok(());
            }
            _ => {}
		}
        player.set_coord(coord)
	}

    Ok(())
}
