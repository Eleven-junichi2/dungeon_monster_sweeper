use std::io;

use rand::{self, Rng};

struct Player {
    pos: Coordinate,
    strength: u8,
}

struct Enemy {
    pos: Coordinate,
    strength: u8,
}

struct Coordinate {
    x: usize,
    y: usize,
}

fn main() {
    let floor_progress: u128 = 0;
    println!("--- Dungeon Monster Sweeper ---");
    println!("HighScore: 未実装");
    println!("Press any key to continue");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("--- How to play ---");
    println!("Press \"escape key\" to exit from the game");
    println!("Input \"x y\" of your destination (For example, \"12 2\" means go to (12, 2))");
    println!("Press any key to continue");
    io::stdin().read_line(&mut input).unwrap();
    // マップ生成
    // 生成したマップを表示
    // 現在の階を表示
    // 操作方法を表示
    let mut rng = rand::thread_rng();
    let player = Player {
        pos: Coordinate {
            x: rng.gen_range(0..16),
            y: rng.gen_range(0..16),
        },
        strength: 2,
    };
    let mut enemy_list: Vec<Enemy> = Vec::new();
    // place enemies
    for _ in 0..rng.gen_range(0..=255) {
        enemy_list.push(Enemy {
            pos: loop {
                let mut pos = Coordinate {
                    x: rng.gen_range(0..16),
                    y: rng.gen_range(0..16),
                };
                if pos.x == player.pos.x && pos.y == player.pos.y {
                    continue;
                } else {
                    break pos;
                };
            },
            strength: rng.gen_range(0..=255),
        });
    }
    // [y][x]
    // for row in map {
    //     println!("{:?}", row);
    // }
    let mut map: [[char; 16]; 16] = [['*'; 16]; 16];
    for enemy in enemy_list {
        map[enemy.pos.y][enemy.pos.x] = 'E';
    }
    map[player.pos.y][player.pos.x] = 'P';
    let mut fog_of_war_map: [[bool; 16]; 16] = [[true; 16]; 16];
    fog_of_war_map[player.pos.y][player.pos.x] = false; // 不可視タイルは?で表現
    for (y, row) in fog_of_war_map.iter().enumerate() {
        for (x, is_invisible) in row.iter().enumerate() {
            if *is_invisible {
                map[y][x] = '?';
            }
        }
    }
    // show map
    for row in map {
        println!("{}", &row.iter().collect::<String>());
    }
}
