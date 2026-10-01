use macroquad::prelude::*;

mod player;
mod assets;
mod const_def;
mod pnj_behavs;

use crate::player::Entity;
use crate::assets::*;
use crate::const_def::*;
use crate::pnj_behavs::pnj_behaviour;
use std::time::Instant;


fn draw_asset(tw: f32, th: f32, asset: &Texture2D,
              pos_map: (f32, f32), pos_asset: (f32, f32),
              flip: (bool, bool), rotation: f32) {
    let (row, col) = pos_map;
    let (x, y) = pos_asset;
    let (flipx, flipy) = flip;
    draw_texture_ex(
        asset,
        col as f32 * tw * ZOOM,
        row as f32 * th * ZOOM,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(x * tw, y * th, tw, th)),
            flip_x: flipx,
            flip_y: flipy,
            rotation,
            dest_size: Some(Vec2::new(tw * ZOOM, th * ZOOM)),
            ..Default::default()
        }
    );
}

fn draw_layer(layer: &Vec<Vec<Vec<Tile>>>, texture: &Texture2D,
              tw: f32, th: f32, entities:&mut Vec<(Entity, (Texture2D, (f32, f32)))>) {
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
                        draw_asset(tw, th, texture,
                                   (row as f32, col as f32), (x, y),
                                   (flipx, flipy), rotation);
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
                let text = joueur.get_name();
                let text_width = measure_text(&text, None, FONT_SIZE as u16, 1.0).width;
                let text_x = px * tw * ZOOM + (tw * ZOOM - text_width) / 2.0;
                let text_y = py * th * ZOOM - 5.0;

                draw_text(&text, text_x, text_y, FONT_SIZE as f32, WHITE);
                draw_asset(tw, th, asset, (py, px), *coord, (joueur.get_flip(), false), 0.0);
            }
        }
        for (tile_row, col, x, y, rotation, flip) in not_printed.iter() {
            if *tile_row == row {
                draw_asset(tw, th, texture, (*tile_row as f32, *col as f32), (*x, *y), *flip, *rotation);
            }
        }
    }
    for (row, col, x, y, rotation, flip) in always_last.iter() {
        draw_asset(tw, th, texture, (*row as f32, *col as f32), (*x, *y), *flip, *rotation,);
    }
}

fn fill_walls(map_objects: &mut Vec<Vec<Vec<Tile>>>, map_walls: &Vec<&str>) {
    let mut line: usize = 0;
    let max_cols: usize = ((screen_width() + 1.0) / PIX_PER_CELL) as usize;
    let max_rows: usize = (screen_height() / PIX_PER_CELL) as usize;
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
    let dimensions = measure_text(text, None, FONT_SIZE as u16, 1.0);

    draw_text(
        text,
        x + (width - dimensions.width) / 2.0,
        y + (height + dimensions.height) / 2.0,
        FONT_SIZE as f32,
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

fn draw_inventory(player: &mut Entity,
                  map_objects: &mut Vec<Vec<Vec<Tile>>>) -> Vec<(String, Instant)> {
    let (x, y) = player.get_coord();
    let mut chat = vec![];
    let inventory_tab = player.get_inventory();
    let text_x = (x + 1.0) * PIX_PER_CELL * ZOOM;
    for (index, (name, qtt, cost)) in inventory_tab.iter().enumerate() {
        let text = format!("{} x{} - {}$", name, qtt, cost);
        let item_y = index as f32 * FONT_SIZE + y * PIX_PER_CELL * ZOOM + 10.0;
        draw_text(&text, text_x, item_y, FONT_SIZE, WHITE);
        let text_width = measure_text(&text, None, FONT_SIZE as u16, 1.0).width;
        if draw_button(text_x + text_width + 10.0, item_y - 50.0/3.0, 50.0, FONT_SIZE, "DROP") {
            let dropped = player.drop_inventory(name.to_string());
            let tile = Tile::get_tile_from_object(name);
            map_objects[y.round() as usize][x.round() as usize].push(tile);
            let text: String = format!("{} dropped 1x{}", player.get_name(), dropped);
            chat.push((text, Instant::now()))
        }
        if draw_button(text_x + text_width + 60.0, item_y - 50.0/3.0, 50.0, FONT_SIZE, "USE") {
            chat.push((player.use_items(name), Instant::now()))
        }
    }
    chat
}

fn activate_around(player: &mut Entity, map_objects: &mut Vec<Vec<Vec<Tile>>>) {
    let (px, py) = player.get_coord();
    let x: isize = px.round() as isize;
    let y: isize = py.round() as isize;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0{
                continue
            }
            let nx: isize = x + dx;
            let ny: isize = y + dy;
            if nx < 0 || ny < 0 {
                continue;
            }
            let nx: usize = nx as usize;
            let ny: usize = ny as usize;
            if ny >= map_objects.len() || nx >= map_objects[ny].len() {
                continue;
            }
            for (_index, tile) in map_objects[ny][nx].iter_mut().enumerate() {
                let new_tile: Tile = Tile::activate_tile(*tile);
                if new_tile != Tile::Void {
                    *tile = new_tile
                }
            }
        }
    }
}

fn grab_object(player: &mut Entity, map_objects: &mut Vec<Vec<Vec<Tile>>>) -> Vec<(String, Instant)> {
    let (px, py) = player.get_coord();
    let x = px.round() as usize;
    let y = py.round() as usize;
    let mut destroy = vec![];
    let mut chat: Vec<(String, Instant)> = vec![];

    for (index, tile) in map_objects[y][x].iter().enumerate() {
        let item = tile.get_object();
        if !item.is_empty() {
            let text = format!("{} grabbed the item {}", player.get_name(), item);
            chat.push((text, Instant::now()));
            player.add_inventory(10, item.to_string());
            destroy.push((y, x, index));
            break

        }
    }
    for (y, x, index) in destroy.into_iter().rev() {
        map_objects[y][x].remove(index);
    }
    chat
}

fn interact_with_others(player: &mut Entity,
                        entities: &mut [(Entity, (Texture2D, (f32, f32)))]) -> Vec<(String, Instant)> {
    let (x, y) = player.get_coord();
    let mut chats: Vec<(String, Instant)> = vec![];
    for (entity, _) in entities.iter_mut() {
        let (ex, ey) = entity.get_coord();
        let dx = x - ex;
        let dy = y - ey;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist <= 1.0 {
            let text: String = pnj_behaviour(entity, player);
            chats.push((text, Instant::now()));
        }
    }
    return chats
}

fn draw_chat(sentences: &mut Vec<(String, Instant)>, current_message: String) {
    let max_height = screen_height();
    if sentences.len() > 5 {
        sentences.remove(0);
    }
    if let Some((_last_msg, sendend)) = sentences.last() {
        if sendend.elapsed().as_secs() < 5 || !current_message.is_empty() {
            for (i, (text, _time)) in sentences.iter().rev().enumerate() {
                let y: f32 = max_height - (i + 1) as f32 * 40.0;
                let text_width: f32 = measure_text(text, None, 30, 1.0).width;
                draw_rectangle(15.0, y - 25.0, text_width + 10.0, 40.0, Color::new(0.0, 0.0, 0.0, 0.4));
                draw_text(text, 20.0, y, 30.0, WHITE);
            }
        }
    }
    draw_text(&current_message, 20.0, max_height - 10.0, 30.0, GRAY);
}


fn key_player(player: &mut Entity, map: &mut Vec<Vec<Vec<Tile>>>,
              entities: &mut [(Entity, (Texture2D, (f32, f32)))],
              current_message: &mut String) -> Vec<(String, Instant)> {
    let mut chats: Vec<(String, Instant)> = vec![];
    if player.get_chatting() {
        if is_key_pressed(KeyCode::Enter) {
            if !current_message.is_empty(){
                if current_message.starts_with('>') {
                    current_message.remove(0);
                }
                if !current_message.is_empty() {
                    let text: String = format!("{}: {}", player.get_name(), current_message);
                    chats.push((text, Instant::now()));
                }
                current_message.clear();
            }
            player.rev_chatting();
        } 
        else if is_key_pressed(KeyCode::Backspace) {
            if current_message.len() > 1 {
                current_message.pop();
            }
        }
        else if is_key_pressed(KeyCode::Escape) {
            current_message.clear();
            player.rev_chatting();
        } 
        else {
            while let Some(c) = get_char_pressed() {
                if !c.is_control() {
                    current_message.push(c);
                }
            }
        }
        return chats;
    }
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
        chats.extend(interact_with_others(player, entities));
    }
    if is_key_pressed(KeyCode::T) {
        chats.extend(grab_object(player, map));
    }
    if is_key_pressed(KeyCode::Enter) {
        player.rev_chatting();
        current_message.push_str(">");
        while get_char_pressed().is_some() {}

    }
    if player.can_print_inventory() {
        chats.extend(draw_inventory(player, map));
    }
    chats
}

async fn add_player(nom: String, coord: (f32, f32),
                    asset_path: &str, asset_player: Characters,
                    entities:&mut Vec<(Entity, (Texture2D, (f32, f32)))>,
                    playable: bool, hp: i32) {
    let asset_coord = asset_player.coord();
    let mut joueur:Entity = Entity::nouvelle(nom, coord, playable, hp);
    let joueur_asset = load_texture(asset_path).await.unwrap();
    joueur_asset.set_filter(FilterMode::Nearest);
    if joueur.get_name() == "Bob" {
        joueur.add_dialogue("Bonjour aventurier!".to_string());
        joueur.add_dialogue("Ce donjon est dangereux !".to_string());
        joueur.add_dialogue("La formule pour faire bouger ses jambes est 'wasd' !".to_string());
        joueur.add_dialogue("Utilises la rune O pour ouvrir les portes !".to_string());
        joueur.add_dialogue("Tu peux ramasser des objets par terre avec T".to_string());
        joueur.add_dialogue("Lance le pouvoir I pour regarder les objets acquis".to_string());
        joueur.add_dialogue("Je n'ai plus rien à t'apprendre".to_string());
    }
    if joueur.get_name() == "Vampire" {
        joueur.add_dialogue("BLBLBLBLBLBLBLBLBL".to_string());
        joueur.add_dialogue("Je reve de gouter de l'ail".to_string());
        joueur.add_dialogue("KSSSSSSSS".to_string());
        joueur.add_dialogue("c'est la peau d'un tueur".to_string());
    }
    entities.push((joueur, (joueur_asset, asset_coord)))
}

fn window_conf() -> Conf {
    Conf {
        window_title: "T-A-P".to_string(),
        fullscreen: true,
        ..Default::default()
    }
}

fn draw_hud(player: &Entity) {
    let name = player.get_name();
    let hp = player.get_hp();
    let hp_max = player.get_hp_max();
    let (x, y) = player.get_coord();
    let max_height = screen_height();
    let max_width = screen_width();
    let mut text = format!("player: {}", name);
    draw_text(&text, max_width - 400.0 ,max_height - 90.0, 30.0, GRAY);
    text = format!("hp: {}/{}", hp, hp_max);
    draw_text(&text, max_width - 400.0 ,max_height - 60.0, 30.0, GRAY);
    text = format!("room: {} (x: {}, y: {})", player.get_location(), x.round(), y.round());
    draw_text(&text, max_width - 400.0 ,max_height - 30.0, 30.0, GRAY);
}

//chats, entities et map_objects a share aux non hosts ?
#[macroquad::main(window_conf)]
async fn main() {
    let texture = load_texture("assets/Dungeon_Tileset.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);
    let mut chats: Vec<(String, Instant)> = vec![(String::new(), Instant::now())];
    let mut current_message: String = "".to_string();

    let tile_width = texture.width() / NB_OF_CELL;
    let tile_height = texture.height() / NB_OF_CELL;

    let mut map_objects:Vec<Vec<Vec<Tile>>> = vec![];
    let mut entities: Vec<(Entity, (Texture2D, (f32, f32)))> = vec![];

    add_player("GigaChad".to_string(), (1.0, 4.0), "assets/player.png", Characters::KnightKnife, &mut entities, true, 20).await;
    add_player("Bob".to_string(), (5.0, 6.0), "assets/player.png", Characters::PriestStick, &mut entities, false, 20,).await;
    add_player("Vampire".to_string(), (26.0, 9.0), "assets/player.png", Characters::VampireHair, &mut entities, false, 20,).await;
    add_player("Death".to_string(), (1.0, 16.0), "assets/player.png", Characters::SquelettonSickle, &mut entities, false, 20,).await;
    let map_walls = vec![
        "W     N1     N1,T       F1,D N1,T N2       N2   E        .    .    .    W    N1   N1   N1     N1      N1   E",  
        "W     F7     F8         F7  F3   F6       F4   E        .    .    .    W    F1   F5   F4     F8      F6   E",  
        "W     F3     F6         F6  F2   F6       F2   E        .    .    .    W    F4   F1   F3     F1      F2   E",  
        "W     F4     F6         F7  F6   F1       F1   N1,DL2   N1   N1   N1   N1   F3   F2   F1     F3      F2   E"  ,
        "W     F3,Pot F2         F7  F4   F4       F6   F1,DL1   F2   F4   F2   F1   F3   F2   F1     F1      F5   DW   N1    N1   N1   N1   N1         N1 N1 N1 N1  N1 E",  
        "W     F5,Pot N1         F5  F3   F3       F5   S5       S    S    S    S4   F5   F7   F5     F3      F3   DW   F1    F1   F1   F1   F1         F2 F3 F1 F2  F1 N1 E",
        "W     F3,Pot F2         F1  F1   F5       F3   E        .    .    .    W    F3   F4   F2     F4,Sk   F1   DW   F1    F1   F1   F1   F1,Chest   F1 F1 F2 F5  F2 F7 N1 E",
        "W     F4     F2         F8  F5   F3       F8   E        .    .    .    W    F7   F4   F1     F2      F8   N1   F1,Dr F1,D S5   S    S          S4 F1 F2 F5  F4 F1 F5 ",
        "SW    S      S4         F1  S5   S        S    SE       .    .    .    W    F8   F3   F2     F6      F7   F3   F1    F4   E    .    .          W  F4 F2 F1  F1 F5 S5 SE",
        "W     N1     N1         F6  N1   N2       N2   E        .    .    .    W    F2   F5   F3     F8      F1   F7   F4    F2   E    .    .          W F4 F2 F1  F1 S5 SE",
        "W     F3     F1         F4  F7   F5       F2   E        .    .    .    W    F6   F2   F7     F4      F3   F8   F1    F5   E    .    .          SW S S S  S SE",
        "W     F8     F4         F2  F1   F6       F7   E        F1   F1  F1    W    F3   F8   F5,Sk  F2      F6   F4   F7    F1   E",
        "W     F5     F7         F1  F3   F2,Pot   F8   N1       N1   N1   N1   N1   F4   F6   F1     F3      F7   F2   F8    F5   E",
        "W     F2     F3         F6  F8   F1       F4   F7       F5   F2   F8   F3   F1   F6   F4     F2      N1   N1   N1    N1   E",
        "W     F1     F5         F3,K1  F7   F4       F2   S5       S    S    S    S4   F6   F8   F3     F1      F5   F2   F4    F7   E",
        "W     F7     F4,Chest   F6  F2   F8       F3   E        .    .    .    W    F1   F5   F4     F7      F3   F8   F2    F6   E",
        "W     F3     F8         F1  F5   F7       F4   E        .    .    .    W    F2   F6   F3     F1      F8   F4   F5    F7   E",
        "SW    S      S          S   S    S        S    SE       .    .    .    SW   S    S    S      S       S    S    S     S    SE",
    ];

    fill_walls(&mut map_objects, &map_walls);
    loop {
        clear_background(BLACK);
        draw_layer(&map_objects, &texture, tile_width, tile_height, &mut entities);
        
        let (player_slice, others) = entities.split_at_mut(1);
        let player = &mut player_slice[0].0;
        chats.extend(key_player(player, &mut map_objects, others, &mut current_message));
        draw_chat(&mut chats, current_message.clone());
        draw_hud(player);
        if is_key_pressed(KeyCode::Escape) && !player.get_chatting() {
            return

        }
        next_frame().await;
    }
}
