// TODO: Logging game messages
// TODO: Calculating and showing avereage of strength of enemies on the path player traveled.
use std::io::{self, Write};

use rand::{random, Rng};
use regex::Regex;

struct Player {
    pos: Coordinate,
    strength: u8,
    hp: u8,
}

struct Enemy {
    pos: Coordinate,
    strength: u8,
}

#[derive(Clone, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn draw_line<T: Copy>(
    canvas: &mut Vec<Vec<T>>,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
    brush: T,
) {
    // DDA algorithm implementation
    let dx = end_x as isize - start_x as isize;
    let dy = end_y as isize - start_y as isize;
    let steps = if dx.abs() > dy.abs() {
        dx.abs()
    } else {
        dy.abs()
    };
    let x_step = dx as f64 / steps as f64;
    let y_step = dy as f64 / steps as f64;
    let mut x: f64 = start_x as f64;
    let mut y: f64 = start_y as f64;
    for _ in 0..=steps {
        canvas[y.round() as usize][x.round() as usize] = brush;
        x += x_step;
        y += y_step;
    }
}

fn print_how_to_play() {
    println!("--- How to play ---");
    println!("Enter \"exit\" to exit program");
    println!("Input \"x y\" of your destination (For example, \"12 2\" means go to (12, 2))");
    println!("Input 'a' to combat enemy");
    println!("Input 'help' to see this message");
}

fn prepare_enemies(
    rng: &mut rand::rngs::ThreadRng,
    floor_progress: u8,
    map_height: usize,
    map_width: usize,
) -> Vec<Enemy> {
    let mut enemy_list: Vec<Enemy> = Vec::new();
    let mut already_used_positions: Vec<Coordinate> = Vec::new();
    let mut how_many_enemies = map_height * map_width;
    if how_many_enemies > (1 + floor_progress).pow(2).into() {
        how_many_enemies = 1 + floor_progress as usize;
    }
    for _ in 0..rng.gen_range(1..=how_many_enemies) {
        let mut pos = Coordinate {
            x: rng.gen_range(0..map_width),
            y: rng.gen_range(0..map_height),
        };
        loop {
            if already_used_positions.contains(&pos) {
                if random::<bool>() {
                    if pos.x < map_width - 1 {
                        pos.x += 1
                    }
                } else {
                    if pos.x > 0 {
                        pos.x -= 1
                    }
                };
                if random::<bool>() {
                    if pos.y > 0 {
                        pos.y -= 1
                    }
                } else {
                    if pos.y < map_height - 1 {
                        pos.y += 1
                    }
                };
            } else {
                break;
            }
        }
        already_used_positions.push(pos.clone());
        enemy_list.push(Enemy {
            pos,
            strength: rng.gen_range(1..=(1 + floor_progress).pow(2)),
        });
    }
    enemy_list
}

fn main() {
    let mut floor_progress: u8 = 0;
    let map_width = 16;
    let map_height = 16;
    let map_width_digits = map_width.to_string().len();
    println!("--- Dungeon Monster Sweeper ---");
    print_how_to_play();
    println!("Press any key to continue");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut rng = rand::thread_rng();
    let mut player = Player {
        pos: Coordinate { x: 0, y: 0 },
        strength: 2,
        hp: 3,
    };
    let mut enemy_list: Vec<Enemy> =
        prepare_enemies(&mut rng, floor_progress, map_height, map_width);
    let mut fog_of_war_map = vec![vec![false; map_width]; map_height];
    let re = Regex::new(r"^\d+ \d+$").unwrap();
    'mainloop: loop {
        if enemy_list.is_empty() {
            if floor_progress > 0 {
                println!("All enemy are eliminated! Player goes to the next floor…");
            };
            fog_of_war_map.fill(vec![true; map_width]);
            enemy_list = prepare_enemies(&mut rng, floor_progress, map_height, map_width);
            player.pos.x = rng.gen_range(0..map_width);
            player.pos.y = rng.gen_range(0..map_height);
            player.hp += 1;
        }
        // prepare map display
        let mut enemy_map = vec![vec![false; map_width]; map_height];
        let mut map_display = vec![vec!['.'; map_width]; map_height];
        for enemy in &enemy_list {
            enemy_map[enemy.pos.y][enemy.pos.x] = true;
            map_display[enemy.pos.y][enemy.pos.x] = 'E';
        }
        map_display[player.pos.y][player.pos.x] = 'P';
        fog_of_war_map[player.pos.y][player.pos.x] = false; // 不可視タイルは?で表現
        for (y, row) in fog_of_war_map.iter().enumerate() {
            for (x, is_invisible) in row.iter().enumerate() {
                if *is_invisible {
                    map_display[y][x] = '?';
                }
            }
        }
        // print x-axis ruler
        println!(
            "{}",
            (0..map_width)
                .map(|x| x.to_string() + &" ".repeat(map_width_digits - (x.to_string().len())))
                .collect::<Vec<String>>()
                .join("")
        ); 
        // print map
        for (y, row) in map_display.iter().enumerate() {
            for row in row.iter() {
                print!("{}{}", row, " ".repeat(map_width_digits - 1));
            }
            println!("{}", y);
        }
        // print infomation
        println!(
            "| Current floor: {}F | Player's strength: {} HP: {} | Remaining enemy: {} |",
            floor_progress,
            player.strength,
            player.hp,
            enemy_list.len()
        );
        // process command from input
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim_end();
        if command == "exit" {
            break;
        } else if command == "help" {
            print_how_to_play();
        } else if command == "a" {
            let mut is_enemy_found = false;
            for (enemy_id, enemy) in enemy_list.iter().enumerate() {
                if enemy.pos.x == player.pos.x && enemy.pos.y == player.pos.y {
                    is_enemy_found = true;
                    println!(
                        "Player combat enemy at (x,y = {},{})",
                        enemy.pos.x, enemy.pos.y
                    );
                    if rng.gen_ratio(
                        player.strength.into(),
                        (player.strength + enemy.strength).into(),
                    ) {
                        let gained_strength = rng.gen_range(1..=enemy.strength);
                        println!(
                            "Player triumped over enemy! +{} Player's strength points",
                            gained_strength
                        );
                        player.strength += gained_strength;
                        enemy_list.remove(enemy_id);
                    } else {
                        println!("Player was defeated by enemy! -1 Player's hit points");
                        player.hp -= 1;
                        if player.hp == 0 {
                            println!("GAMEOVER… Press any key to exit.");
                            break 'mainloop;
                        }
                    }
                    break;
                }
            }
            if !is_enemy_found {
                println!("No enemy found at your current coordinate");
            }
        } else if re.is_match(command) {
            let mut destination = command.split_whitespace();
            let x: usize = destination.next().unwrap().parse().unwrap();
            let y: usize = destination.next().unwrap().parse().unwrap();
            (player.pos.x, player.pos.y) = if (x < map_width) && (y < map_height) {
                // ↑ don't need to worry about negative numbers because they are already checked in the regular expression.
                println!(
                    "Player move to (x,y = {},{}) from (x,y = {},{})",
                    x, y, player.pos.x, player.pos.y
                );
                // 移動元座標から移動先座標のfog of warを明らかにする
                draw_line(&mut fog_of_war_map, player.pos.x, player.pos.y, x, y, false);
                (x, y)
            } else {
                println!("Invalid coordinate for destination");
                continue;
            };
            match enemy_map[player.pos.y][player.pos.x] {
                true => {
                    for enemy in &enemy_list {
                        if enemy.pos.x == player.pos.x && enemy.pos.y == player.pos.y {
                            println!(
                                "Player found enemy at (x,y = {},{}). It's strength is {}. ",
                                enemy.pos.x, enemy.pos.y, enemy.strength
                            );
                            println!(
                                "Player's winning percentage against it is {}%",
                                (player.strength as f64
                                    / (player.strength + enemy.strength) as f64)
                                    * 100.0
                            );
                        }
                    }
                }
                false => (),
            };
        };
        if enemy_list.is_empty() {
            floor_progress += 1;
        };
    }
}
