use macroquad::prelude::*;

#[derive(Copy, Clone, PartialEq)] // Ajout de PartialEq pour pouvoir faire : tile == Tile::Empty
enum Tile {
    Empty, // Ajout indispensable pour les cases "vides"
    WWall1, WWall2, NWall1, NWall2, SeAngle, SWall, SwAngle, EWall1, EWall2, S5, S4, Void,
    Floor1, Floor2, Floor3, Floor4, Floor5, Floor6, Floor7, Floor8,
    Spider1, Door, Spider2, LatDoor1, LatDoor2,
    NTorch, Skeleton, Chest, // Nouvelles tuiles ajoutées pour correspondre à vos grilles
}

impl Tile {
    fn coord(self) -> (f32, f32) {
        match self {
            Tile::Empty   => (0.0, 0.0), 
            Tile::WWall1  => (0.0, 0.0),
            Tile::WWall2  => (0.0, 1.0),
            Tile::NWall1  => (1.0, 0.0),
            Tile::NWall2  => (3.0, 0.0),
            Tile::SwAngle => (0.0, 4.0),
            Tile::S5 =>(0.0, 5.0),
            Tile::S4 =>(3.0, 5.0),
            Tile::SWall   => (1.0, 4.0),
            Tile::SeAngle => (5.0, 4.0),
            Tile::EWall1  => (5.0, 0.0),
            Tile::EWall2  => (5.0, 1.0),
            Tile::Floor1  => (6.0, 0.0),
            Tile::Floor2  => (7.0, 0.0),
            Tile::Floor3  => (8.0, 0.0),
            Tile::Floor4  => (9.0, 0.0),
            Tile::Floor5  => (6.0, 1.0),
            Tile::Floor6  => (7.0, 1.0),
            Tile::Floor7  => (8.0, 1.0),
            Tile::Floor8  => (9.0, 1.0),
            Tile::Spider1 => (4.0, 6.0),
            Tile::Spider2 => (5.0, 6.0),
            Tile::Door    => (7.0, 3.0),
            Tile::LatDoor1 => (7.0, 5.0),
            Tile::LatDoor2 => (7.0, 4.0),
            Tile::NTorch   => (0.0, 9.0),
            Tile::Skeleton => (7.0, 7.0),
            Tile::Chest    => (0.0, 8.0),
            Tile::Void     => (8.0, 7.0),
        }
    }
}

// Rendu d'une couche 2D
fn draw_layer(layer: &Vec<Vec<Tile>>, texture: &Texture2D, tw: f32, th: f32, zoom: f32) {
    for (row, line) in layer.iter().enumerate() {
        for (column, &tile) in line.iter().enumerate() {
            if tile == Tile::Empty { continue; } // On n'affiche rien si c'est vide
            
            let (x, y) = tile.coord();
            draw_texture_ex(
                texture,
                column as f32 * tw * zoom,
                row as f32 * th * zoom,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(x * tw, y * th, tw, th)),
                    dest_size: Some(Vec2::new(tw * zoom, th * zoom)),
                    ..Default::default()
                }
            );
        }
    }
}

fn fill_walls(walls_layer: &mut Vec<Vec<Tile>>, decor_layer: &mut Vec<Vec<Tile>>, map_walls: &Vec<&str>) {
    let mut line = 0;
    let max_cols = 1980 / 16;
    let max_rows = 1200 / 16;
    for row_str in map_walls {
        let mut col = 0;
        let mut walls_row = vec![];
        let mut decor_row = vec![];
        for token in row_str.split_whitespace() {
            let mut final_wall = Tile::Void;
            let mut final_deco = Tile::Empty;

            for index in token.split(',') {
                let tile_wall = match index {
                    "W"   => Tile::WWall1,
                    "N1"  => Tile::NWall1,
                    "N2"  => Tile::NWall2,
                    "D"   => Tile::Door,
                    "E"   => Tile::EWall1,
                    "SW"  => Tile::SwAngle,
                    "SE"  => Tile::SeAngle,
                    "S"   => Tile::SWall,
                    "F1"  => Tile::Floor1,
                    "F2"  => Tile::Floor2,
                    "F3"  => Tile::Floor3,
                    "F4"  => Tile::Floor4,
                    "F5"  => Tile::Floor5,
                    "F6"  => Tile::Floor6,
                    "F7"  => Tile::Floor7,
                    "F8"  => Tile::Floor8,
                    "S5"  => Tile::S5,
                    "S4"  => Tile::S4,
                    "."   => Tile::Void,
                    _     => Tile::Void,
                };
                let tile_deco = match index {
                    "T"     => Tile::NTorch,
                    "Sp2"   => Tile::Spider2,
                    "Sk"    => Tile::Skeleton,
                    "DL1"    => Tile::LatDoor1,
                    "DL2"    => Tile::LatDoor2,
                    "Chest" => Tile::Chest,
                    "Sp1"   => Tile::Spider1,
                    _       => Tile::Empty,
                };
                if tile_wall != Tile::Void {
                    final_wall = tile_wall;
                }
                if tile_deco != Tile::Empty {
                    final_deco = tile_deco;
                }
            }
            walls_row.push(final_wall);
            decor_row.push(final_deco);
            col += 1;
        }
        while col < max_cols { 
            walls_row.push(Tile::Void);
            decor_row.push(Tile::Empty);
            col += 1; 
        }
        
        walls_layer.push(walls_row);
        decor_layer.push(decor_row);
        line += 1; 
    }
    while line < max_rows {
        walls_layer.push(vec![Tile::Void; max_cols]);
        decor_layer.push(vec![Tile::Empty; max_cols]);
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

#[macroquad::main(window_conf)]
async fn main() {
    let texture = load_texture("assets/Dungeon_Tileset.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest); 
    
    let tile_width = texture.width() / 10.0;
    let tile_height = texture.height() / 10.0;
    
    let mut walls_layer = vec![];
    let mut decor_layer = vec![];

    let map_walls = vec![
        "W     N1    N1,T       D   N1,T N2   N2   E        .    .    .    W    N1   N1   N1     N1      N1   E",  
        "W     F7    F8         F7  F3   F6   F4   E        .    .    .    W    F1   F5   F4     F8      F6   E",  
        "W     F3    F6         F6  F2   F6   F2   E        .    .    .    W    F4   F1   F3     F1      F2   E",  
        "W     F4    F6         F7  F6   F1   F1   N1,DL2   N1   N1   N1   N1   F3   F2   F1     F3      F2   E"  ,
        "W     F3    F2         F7  F4   F4   F6   F1,DL1   F2   F4   F2   F1   F3   F2   F1     F1      F5   E",  
        "W     F5    F7         F5  F3   F3   F5   S5       S    S    S    S4   F5   F7   F5     F3      F3   E    W    N1   N1   N1   N1         E",
        "W     F3    F2         F1  F1   F5   F3   E        .    .    .    W    F3   F4   F2     F4,Sk   F1   E    W    F1   F1   F1   F1,Chest   E",
        "W     F4    F2         F8  F5   F3   F8   E        .    .    .    W    F7   F4   F1     F2      F8   N1   N1   D    S5   S    S          SE ",
        "SW    S     S4         F1  S5   S    S    SE       .    .    .    W    F8   F3   F2     F6      F7   F3   F1   F4   E",
        "W     N1    N1         F6  N1   N2   N2   E        .    .    .    W    F2   F5   F3     F8      F1   F7   F4   F2   E",
        "W     F3    F1         F4  F7   F5   F2   E        .    .    .    W    F6   F2   F7     F4      F3   F8   F1   F5   E",
        "W     F8    F4         F2  F1   F6   F7   E        .    .    .    W    F3   F8   F5,Sk  F2      F6   F4   F7   F1   E",
        "W     F5    F7         F1  F3   F2   F8   N1       N1   N1   N1   N1   F4   F6   F1     F3      F7   F2   F8   F5   E",
        "W     F2    F3         F6  F8   F1   F4   F7       F5   F2   F8   F3   F1   F6   F4     F2      F7   F5   F3   F8   E",
        "W     F1    F5         F3  F7   F4   F2   S5       S    S    S    S4   F6   F8   F3     F1      F5   F2   F4   F7   E",
        "W     F7    F4,Chest   F6  F2   F8   F3   E        .    .    .    W    F1   F5   F4     F7      F3   F8   F2   F6   E",
        "W     F3    F8         F1  F5   F7   F4   E        .    .    .    W    F2   F6   F3     F1      F8   F4   F5   F7   E",
        "SW    S     S          S   S    S    S    SE       .    .    .    SW   S    S    S      S       S    S    S    S    SE",
    ];

    fill_walls(&mut walls_layer, &mut decor_layer, &map_walls);
    let zoom = 4.0;
    loop {
        clear_background(BLACK);
        draw_layer(&walls_layer, &texture, tile_width, tile_height, zoom);
        draw_layer(&decor_layer, &texture, tile_width, tile_height, zoom);
        next_frame().await;
    }
}