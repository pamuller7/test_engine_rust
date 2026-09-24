use macroquad::prelude::*;

mod player;

use crate::player::Entity;

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Empty,
    WWall1, WWall2, NWall1, NWall2, SeAngle, SWall, SwAngle, EWall1, EWall2, S5, S4, Void,
    Floor1, Floor2, Floor3, Floor4, Floor5, Floor6, Floor7, Floor8,
    Spider1, Door, Spider2, LatDoor1, LatDoor2,
    NTorch, Skeleton, Chest, Pot
}

// tile coord: (x, y, blocking or not, printed after or before the player
//                                     0-> printed 1st layer
//                                     1-> depending on player pos
//                                     2-> printed last layer)
impl Tile {
    fn coord(self) -> (f32, f32, f32, f32) {
        match self {
            Tile::Empty   => (0.0, 0.0, 1.0, 0.0),

            //walls
            Tile::WWall1  => (0.0, 0.0, 0.0, 2.0),
            Tile::WWall2  => (0.0, 1.0, 0.0, 2.0),
            Tile::NWall1  => (1.0, 0.0, 0.0, 1.0),
            Tile::NWall2  => (3.0, 0.0, 0.0, 1.0),
            Tile::SwAngle => (0.0, 4.0, 0.0, 2.0),
            Tile::S5 =>(0.0, 5.0, 0.0, 2.0),
            Tile::S4 =>(3.0, 5.0, 0.0, 2.0),
            Tile::SWall   => (1.0, 4.0, 0.0, 1.0),
            Tile::SeAngle => (5.0, 4.0, 0.0, 2.0),
            Tile::EWall1  => (5.0, 0.0, 0.0, 2.0),
            Tile::EWall2  => (5.0, 1.0, 0.0, 2.0),

            //floors
            Tile::Floor1  => (6.0, 0.0, 1.0, 0.0),
            Tile::Floor2  => (7.0, 0.0, 1.0, 0.0),
            Tile::Floor3  => (8.0, 0.0, 1.0, 0.0),
            Tile::Floor4  => (9.0, 0.0, 1.0, 0.0),
            Tile::Floor5  => (6.0, 1.0, 1.0, 0.0),
            Tile::Floor6  => (7.0, 1.0, 1.0, 0.0),
            Tile::Floor7  => (8.0, 1.0, 1.0, 0.0),
            Tile::Floor8  => (9.0, 1.0, 1.0, 0.0),

            Tile::Void     => (8.0, 7.0, 1.0, 0.0), // bg color

            //decorations
            Tile::Spider1 => (4.0, 6.0, 1.0, 0.0),
            Tile::Spider2 => (5.0, 6.0, 1.0, 0.0),
            Tile::Door    => (7.0, 3.0, 0.0, 1.0),
            Tile::LatDoor1 => (7.0, 5.0, 0.0, 2.0),
            Tile::LatDoor2 => (7.0, 4.0, 0.0, 2.0),
            Tile::NTorch   => (0.0, 9.0, 1.0, 0.0),
            Tile::Skeleton => (7.0, 7.0, 1.0, 0.0),
            Tile::Chest    => (0.0, 8.0, 0.0, 0.0),
            Tile::Pot      => (9.0, 8.0, 1.0, 0.0)
        }
    }
}



fn draw_asset(tw: f32, th: f32, zoom: f32, asset: &Texture2D, pos_map: (f32, f32), pos_asset: (f32, f32), flip: bool){
    let (row, col) = pos_map;
    let (x, y) = pos_asset;
    draw_texture_ex(
        asset,
        col as f32 * tw * zoom,
        row as f32 * th * zoom,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(x * tw, y * th, tw, th)),
            flip_x: flip,
            dest_size: Some(Vec2::new(tw * zoom, th * zoom)),
            ..Default::default()
        }
    );
}

fn draw_layer(layer: &Vec<Vec<Vec<Tile>>>, texture: &Texture2D, tw: f32, th: f32, zoom: f32, player: &Entity, joueur_asset: &Texture2D,
) {
    let (px, py) = player.get_coord();
    let player_row = py.round() as usize;
    let mut not_printed = vec![];
    for (row, line) in layer.iter().enumerate() {
        for (col, cell) in line.iter().enumerate() {
            for tile in cell {
                if *tile == Tile::Empty {
                    continue;
                }
                let (x, y, _z, order) = tile.coord();
                if order == 0.0{
                    draw_asset(tw, th, zoom, texture, (row as f32, col as f32), (x, y), false);
                }
                else {
                    if row >= player_row || order == 2.0 {
                        not_printed.push((row, col, x, y));
                        continue;
                    }
                    draw_asset(tw, th, zoom, texture, (row as f32, col as f32), (x, y), false);
                }
            }
        }
    }
    draw_asset(tw, th, zoom, joueur_asset, (py, px), (4.0, 0.0), player.get_flip());
    for (row, col, x, y) in not_printed.iter() {
        draw_asset(tw, th, zoom, texture, (*row as f32, *col as f32), (*x, *y), false);
    }
}


fn fill_walls(walls_layer: &mut Vec<Vec<Vec<Tile>>>, map_walls: &Vec<&str>) {
    let mut line = 0;
    let max_cols = 1980 / 16;
    let max_rows = 1200 / 16;
    for row_str in map_walls {
        let mut col = 0;
        let mut walls_row: Vec<Vec<Tile>> = vec![];
        for token in row_str.split_whitespace() {
            let mut cell_infos: Vec<Tile> = vec![];
            for index in token.split(',') {
                let tile_wall = match index {
                    "W"     => Tile::WWall1,
                    "N1"    => Tile::NWall1,
                    "N2"    => Tile::NWall2,
                    "D"     => Tile::Door,
                    "E"     => Tile::EWall1,
                    "SW"    => Tile::SwAngle,
                    "SE"    => Tile::SeAngle,
                    "S"     => Tile::SWall,
                    "F1"    => Tile::Floor1,
                    "F2"    => Tile::Floor2,
                    "F3"    => Tile::Floor3,
                    "F4"    => Tile::Floor4,
                    "F5"    => Tile::Floor5,
                    "F6"    => Tile::Floor6,
                    "F7"    => Tile::Floor7,
                    "F8"    => Tile::Floor8,
                    "S5"    => Tile::S5,
                    "S4"    => Tile::S4,
                    "."     => Tile::Void,
                    "T"     => Tile::NTorch,
                    "Sp2"   => Tile::Spider2,
                    "Sk"    => Tile::Skeleton,
                    "DL1"   => Tile::LatDoor1,
                    "DL2"   => Tile::LatDoor2,
                    "Chest" => Tile::Chest,
                    "Sp1"   => Tile::Spider1,
                    "Pot"   => Tile::Pot,
                    _       => Tile::Empty,
                };
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
        
        walls_layer.push(walls_row);
        line += 1; 
    }
    while line < max_rows {
        walls_layer.push(vec![vec![Tile::Void]; max_cols]);
        line += 1;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "T-A-P".to_string(),
        window_width: 1980,
        window_height: 1200,
        ..Default::default()
    }
}


fn can_moove(player: &mut Entity, map: &Vec<Vec<Vec<Tile>>>, movement: &str) -> bool {
    let deplacement:(f32, f32) = match movement {
        "N" => (0.0, -0.1),
        "S" => (0.0, 0.1),
        "E" => (0.1, 0.0),
        "W" => (-0.1, 0.0),
        _   => (0.0, 0.0)
    };
    let (x, y) = player.get_coord();
    let (x1, y1) = deplacement;
    let new_col = ((x + x1).round()) as usize;
    let new_line = ((y + y1).round()) as usize;
    for tile in map[new_line][new_col].iter(){
        let (_, _, info, _) = tile.coord();
        if info == 0.0{
            return false
        }
    }
    return true
}

fn moove_player(player: &mut Entity, map: &Vec<Vec<Vec<Tile>>>) {
    if is_key_down(KeyCode::W) {
        if can_moove(player, map, "N"){
            player.moove("N")
        }
    }
    if is_key_down(KeyCode::A) {
        if can_moove(player, map, "W"){
            player.moove("W")
        }
    }
    if is_key_down(KeyCode::S) {
        if can_moove(player, map, "S"){
            player.moove("S")
        }
    }
    if is_key_down(KeyCode::D) {
        if can_moove(player, map, "E"){
            player.moove("E")
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let texture = load_texture("assets/Dungeon_Tileset.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest); 
    
    let tile_width = texture.width() / 10.0;
    let tile_height = texture.height() / 10.0;
    
    let mut walls_layer:Vec<Vec<Vec<Tile>>> = vec![];

    let map_walls = vec![
        "W     N1    N1,T       D   N1,T N2       N2   E        .    .    .    W    N1   N1   N1     N1      N1   E",  
        "W     F7    F8         F7  F3   F6       F4   E        .    .    .    W    F1   F5   F4     F8      F6   E",  
        "W     F3    F6         F6  F2   F6       F2   E        .    .    .    W    F4   F1   F3     F1      F2   E",  
        "W     F4    F6         F7  F6   F1       F1   N1,DL2   N1   N1   N1   N1   F3   F2   F1     F3      F2   E"  ,
        "W     F3    F2         F7  F4   F4       F6   F1,DL1   F2   F4   F2   F1   F3   F2   F1     F1      F5   E    .    .    .    .    .          .  N1 N1 N1",  
        "W     F5    F7         F5  F3   F3       F5   S5       S    S    S    S4   F5   F7   F5     F3      F3   E    W    N1   N1   N1   N1         E  W  F1 F2",
        "W     F3    F2         F1  F1   F5       F3   E        .    .    .    W    F3   F4   F2     F4,Sk   F1   E    W    F1   F1   F1   F1,Chest   F1 F1 F2 F5",
        "W     F4    F2         F8  F5   F3       F8   E        .    .    .    W    F7   F4   F1     F2      F8   N1   N1   D    S5   S    S          SE F1 F2 F5",
        "SW    S     S4         F1  S5   S        S    SE       .    .    .    W    F8   F3   F2     F6      F7   F3   F1   F4   E    .    .          .  S  S  S",
        "W     N1    N1         F6  N1   N2       N2   E        .    .    .    W    F2   F5   F3     F8      F1   F7   F4   F2   E",
        "W     F3    F1         F4  F7   F5       F2   E        .    .    .    W    F6   F2   F7     F4      F3   F8   F1   F5   E",
        "W     F8    F4         F2  F1   F6       F7   E        .    .    .    W    F3   F8   F5,Sk  F2      F6   F4   F7   F1   E",
        "W     F5    F7         F1  F3   F2,Pot   F8   N1       N1   N1   N1   N1   F4   F6   F1     F3      F7   F2   F8   F5   E",
        "W     F2    F3         F6  F8   F1       F4   F7       F5   F2   F8   F3   F1   F6   F4     F2      F7   F5   F3   F8   E",
        "W     F1    F5         F3  F7   F4       F2   S5       S    S    S    S4   F6   F8   F3     F1      F5   F2   F4   F7   E",
        "W     F7    F4,Chest   F6  F2   F8       F3   E        .    .    .    W    F1   F5   F4     F7      F3   F8   F2   F6   E",
        "W     F3    F8         F1  F5   F7       F4   E        .    .    .    W    F2   F6   F3     F1      F8   F4   F5   F7   E",
        "SW    S     S          S   S    S        S    SE       .    .    .    SW   S    S    S      S       S    S    S    S    SE",
    ];

    let mut joueur:Entity = Entity::nouvelle("Pascal".to_string(), (1.0,4.0));
    let joueur_asset = load_texture("assets/Dungeon_Character_2.png").await.unwrap();
    joueur_asset.set_filter(FilterMode::Nearest); 
    fill_walls(&mut walls_layer, &map_walls);
    let zoom = 3.0;
    loop {
        clear_background(BLACK);
        draw_layer(&walls_layer, &texture, tile_width, tile_height, zoom, &joueur, &joueur_asset);

        moove_player(&mut joueur, &walls_layer);
        next_frame().await;
    }
}