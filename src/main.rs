use macroquad::prelude::*;

mod player;
mod assets;

use crate::player::Entity;
use crate::assets::*;


fn draw_asset(tw: f32, th: f32, zoom: f32, asset: &Texture2D, pos_map: (f32, f32), pos_asset: (f32, f32), flip: (bool, bool), rotation: f32){
    let (row, col) = pos_map;
    let (x, y) = pos_asset;
    let (flipx, flipy) = flip;
    draw_texture_ex(
        asset,
        col as f32 * tw * zoom,
        row as f32 * th * zoom,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(x * tw, y * th, tw, th)),
            flip_x: flipx,
            flip_y: flipy,
            rotation,
            dest_size: Some(Vec2::new(tw * zoom, th * zoom)),
            ..Default::default()
        }
    );
}

fn draw_layer( layer: &Vec<Vec<Vec<Tile>>>, texture: &Texture2D, tw: f32, th: f32, zoom: f32, entities:&mut Vec<(Entity, (Texture2D, (f32, f32)))>) {
    let mut not_printed = vec![];
    let mut always_last = vec![];
    for (row, line) in layer.iter().enumerate() {
        for (col, cell) in line.iter().enumerate() {
            for tile in cell {
                if *tile == Tile::Empty {
                    continue;
                }
                let (rotation, flipx, flipy) = Tile::rotate_tile(*tile);
                let (x, y, _z, order) = tile.coord();
                match order {
                    0.0 => {
                        draw_asset( tw, th, zoom, texture, (row as f32, col as f32), (x, y), (flipx, flipy), rotation);
                    }
                    1.0 => {
                        not_printed.push((row, col, x, y, rotation, (flipx, flipy)));
                    }

                    2.0 => {
                        always_last.push((row, col, x, y, rotation, (flipx, flipy)));
                    }
                    _ => {}
                }
            }
        }
    }
    for row in 0..layer.len() {
        for (joueur, (asset, coord)) in entities.iter() {
            let (px, py) = joueur.get_coord();
            if py.round() as usize == row {
                draw_asset(tw, th, zoom, asset, (py, px), *coord, (joueur.get_flip(), false), 0.0);
            }
        }
        for (tile_row, col, x, y, rotation, flip) in not_printed.iter() {
            if *tile_row == row {
                draw_asset(tw, th, zoom, texture, (*tile_row as f32, *col as f32), (*x, *y), *flip, *rotation);
            }
        }
    }
    for (row, col, x, y, rotation, flip) in always_last.iter() {
        draw_asset(tw, th, zoom, texture, (*row as f32, *col as f32), (*x, *y), *flip, *rotation,);
    }
}

fn fill_walls(map_objects: &mut Vec<Vec<Vec<Tile>>>, map_walls: &Vec<&str>) {
    let mut line = 0;
    let max_cols = 1980 / 16;
    let max_rows = 1200 / 16;
    for row_str in map_walls {
        let mut col = 0;
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

fn window_conf() -> Conf {
    Conf {
        window_title: "T-A-P".to_string(),
        window_width: 1980,
        window_height: 1200,
        ..Default::default()
    }
}

fn draw_button(x: f32, y: f32, width: f32, height: f32, text: &str) -> bool {
    let (mouse_x, mouse_y) = mouse_position();

    let hovered =
        mouse_x >= x &&
        mouse_x <= x + width &&
        mouse_y >= y &&
        mouse_y <= y + height;
    let color = if hovered {
        DARKGRAY
    } else {
        GRAY
    };

    draw_rectangle(x, y, width, height, color);
    let font_size = 30;
    let dimensions = measure_text(text, None, font_size, 1.0);

    draw_text(
        text,
        x + (width - dimensions.width) / 2.0,
        y + (height + dimensions.height) / 2.0,
        font_size as f32,
        WHITE,
    );
    hovered && is_mouse_button_pressed(MouseButton::Left)
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


fn draw_inventory(player: &mut Entity, map_objects: &mut Vec<Vec<Vec<Tile>>>) {
    let (x, y) = player.get_coord();
    let inventory_tab = player.get_inventory();
    let text_x = (x + 1.0) * 16.0 * 3.0;
    for (index, (name, qtt, cost)) in inventory_tab.iter().enumerate() {
        let text = format!("{} x{} - {}$", name, qtt, cost);
        let item_y = index as f32 * 30.0 + y * 16.0 * 3.0;
        draw_text(&text, text_x, item_y, 30.0, WHITE);
        let text_width = measure_text(&text, None, 30, 1.0).width;
        if draw_button(text_x + text_width + 10.0, item_y - 23.0, 50.0, 30.0, "DROP") {
            player.drop_inventory(name.to_string());
            map_objects[y as usize][x as usize].push(Tile::Pot)
        }
    }
}

fn activate_around(player: &mut Entity, map_objects: &mut Vec<Vec<Vec<Tile>>>) {
    let (px, py) = player.get_coord();
    let x = px.round() as isize;
    let y = py.round() as isize;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0{
                continue
            }
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || ny < 0 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if ny >= map_objects.len() || nx >= map_objects[ny].len() {
                continue;
            }
            for (index, tile) in map_objects[ny][nx].iter_mut().enumerate() {
                let new_tile = Tile::activate_tile(*tile);
                if new_tile != Tile::Void {
                    *tile = new_tile
                }
            }
        }
    }
}

fn grab_object(player: &mut Entity, map_objects: &mut Vec<Vec<Vec<Tile>>>) {
    let (px, py) = player.get_coord();
    let x = px.round() as usize;
    let y = py.round() as usize;
    let mut destroy = vec![];

    for (index, tile) in map_objects[y][x].iter().enumerate() {
        
        match tile {
            Tile::Pot => {
                player.add_inventory(10, "Potion".to_string());
                destroy.push((y, x, index));
            }
            _ => {}
        }
    }
    for (y, x, index) in destroy.into_iter().rev() {
        map_objects[y][x].remove(index);
    }
}

fn interact_with_others(player: &mut Entity, entities: &[(Entity, (Texture2D, (f32, f32)))]) {
    let (x, y) = player.get_coord();
    for (entity, _) in entities.iter() {
        let (ex, ey) = entity.get_coord();
        let dx = x - ex;
        let dy = y - ey;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist <= 1.0 {
            let text = entity.talk();
            let (x,y) = entity.get_coord();
            draw_text(&text, x * 16.0 * 3.0, y * 16.0 * 3.0, 30.0, WHITE);
        }
    }
}

fn key_player(player: &mut Entity, map: &mut Vec<Vec<Vec<Tile>>>, entities: &[(Entity, (Texture2D, (f32, f32)))]) {
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
    if is_key_pressed(KeyCode::I) {
        player.rev_print_inventory()
    }
    if is_key_pressed(KeyCode::O) {
        activate_around(player, map);
        interact_with_others(player, entities);
    }
    if is_key_pressed(KeyCode::T) {
        grab_object(player, map)
    }
    if player.can_print_inventory() {
        draw_inventory(player, map);
    }
}

async fn add_player(nom: String, coord: (f32, f32), asset_path: &str, asset_player: Characters, entities:&mut Vec<(Entity, (Texture2D, (f32, f32)))>, playable: bool) {
    let asset_coord = asset_player.coord();
    let mut joueur:Entity = Entity::nouvelle(nom, coord, playable);
    let joueur_asset = load_texture(asset_path).await.unwrap();
    joueur_asset.set_filter(FilterMode::Nearest);
    if !playable {
        joueur.add_dialogue("Bonjour !".to_string())
    }
    entities.push((joueur, (joueur_asset, asset_coord)))
}

#[macroquad::main(window_conf)]
async fn main() {
    let texture = load_texture("assets/Dungeon_Tileset.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest); 
    
    let tile_width = texture.width() / 10.0;
    let tile_height = texture.height() / 10.0;
    
    let mut map_objects:Vec<Vec<Vec<Tile>>> = vec![];
    let mut entities: Vec<(Entity, (Texture2D, (f32, f32)))> = vec![];
    add_player(
        "Pascal".to_string(),
        (1.0, 4.0),
        "assets/player.png",
        Characters::KnightKnife,
        &mut entities,
        true,
    ).await;

    add_player(
        "Bob".to_string(),
        (5.0, 6.0),
        "assets/player.png",
        Characters::PriestStick,
        &mut entities,
        false,
    ).await;

    let map_walls = vec![
        "W     N1    N1,T       F1,D N1,T N2       N2   E        .    .    .    W    N1   N1   N1     N1      N1   E",  
        "W     F7    F8         F7  F3   F6       F4   E        .    .    .    W    F1   F5   F4     F8      F6   E",  
        "W     F3    F6         F6  F2   F6       F2   E        .    .    .    W    F4   F1   F3     F1      F2   E",  
        "W     F4    F6         F7  F6   F1       F1   N1,DL2   N1   N1   N1   N1   F3   F2   F1     F3      F2   E"  ,
        "W     F3    F2         F7  F4   F4       F6   F1,DL1   F2   F4   F2   F1   F3   F2   F1     F1      F5   E    .    .    .    .    .          .  N1 N1 N1",  
        "W     F5    N1         F5  F3   F3       F5   S5       S    S    S    S4   F5   F7   F5     F3      F3   E    W    N1   N1   N1   N1         E  W  F1 F2",
        "W     F3    F2         F1  F1   F5       F3   E        .    .    .    W    F3   F4   F2     F4,Sk   F1   E    W    F1   F1   F1   F1,Chest   F1 F1 F2 F5",
        "W     F4    F2         F8  F5   F3       F8   E        .    .    .    W    F7   F4   F1     F2      F8   N1   F1,Dr   F1,D    S5   S    S          SE F1 F2 F5",
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
    fill_walls(&mut map_objects, &map_walls);
    let zoom = 3.0; 
    loop {
        clear_background(BLACK);
        draw_layer(&map_objects, &texture, tile_width, tile_height, zoom, &mut entities);
        let (player_slice, others) = entities.split_at_mut(1);
        let player = &mut player_slice[0].0;
        key_player(
            player,
            &mut map_objects,
            others,
        );
        next_frame().await;
    }
}