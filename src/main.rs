use std::io;

use rand::{self, Rng};
use regex::Regex;

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
    println!("Enter \"exit\" to exit program");
    println!("Input \"x y\" of your destination (For example, \"12 2\" means go to (12, 2))");
    println!("Press any key to continue");
    io::stdin().read_line(&mut input).unwrap();
    // マップ生成
    // 生成したマップを表示
    // 現在の階を表示
    // 操作方法を表示
    let mut rng = rand::thread_rng();
    let mut player = Player {
        pos: Coordinate {
            x: rng.gen_range(0..16),
            y: rng.gen_range(0..16),
        },
        strength: 2,
    };
    let mut enemy_list: Vec<Enemy> = Vec::new();
    // prepare enemies
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
    let mut fog_of_war_map: [[bool; 16]; 16] = [[true; 16]; 16];
    let re = Regex::new(r"^\d+ \d+$").unwrap();
    loop {
        // prepare map
        let mut map: [[char; 16]; 16] = [['*'; 16]; 16];
        for enemy in &enemy_list {
            map[enemy.pos.y][enemy.pos.x] = 'E';
        }
        map[player.pos.y][player.pos.x] = 'P';
        fog_of_war_map[player.pos.y][player.pos.x] = false; // 不可視タイルは?で表現
        for (y, row) in &fog_of_war_map.iter().enumerate() {
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

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim_end();
        if command == "exit" {
            break;
        } else if re.is_match(command){
            let mut destination = command.split_whitespace();
            let x: usize = destination.next().unwrap().parse().unwrap();
            let y: usize = destination.next().unwrap().parse().unwrap();
            (player.pos.x, player.pos.y) = if (x < 16) && (y < 16) { (x, y) } else {
                // ↑ don't need to worry about negative numbers because they are already checked in the regular expression.
                println!("Invalid coordinate for destination");
                continue;
            };
            println!("Player move to (x, y) = ({}, {})", player.pos.x, player.pos.y);
        }
    };
}
