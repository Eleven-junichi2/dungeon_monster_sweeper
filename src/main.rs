// TODO: Logging game messages
// TODO: Calculating and showing avereage of strength of enemies on the path player traveled.
use std::{
    collections::HashSet,
    hash::Hash,
    io::{self, Write},
    ops::{Index, IndexMut},
};

use crossterm::{
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
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

#[derive(Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn mut_each_step_of_line_drawing(
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
    f: &mut dyn FnMut(f64, f64),
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
        f(x, y);
        x += x_step;
        y += y_step;
    }
}

fn draw_line<T: Copy>(
    canvas: &mut Vec<Vec<T>>,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
    brush: T,
) {
    mut_each_step_of_line_drawing(start_x, start_y, end_x, end_y, &mut |x, y| {
        canvas[y.round() as usize][x.round() as usize] = brush
    });
}

#[derive(Eq, Hash, PartialEq)]
enum FogOfWar {
    Coordinate(Coordinate),
}

struct DungeonFloor {
    width: usize,
    height: usize,
    fog_of_wars: HashSet<FogOfWar>,
    enemies: Vec<Enemy>,
}

impl DungeonFloor {
    fn fog_of_war_maskmap(&self, width: usize, height: usize) -> Vec<Vec<bool>> {
        let mut map = vec![vec![false; width]; height];
        for fog_of_war in &self.fog_of_wars {
            match fog_of_war {
                FogOfWar::Coordinate(c) => map[c.y][c.x] = true,
            };
        }
        map
    }
    fn enemy_maskmap(&self, width: usize, height: usize) -> Vec<Vec<bool>> {
        let mut map = vec![vec![false; width]; height];
        for enemy in &self.enemies {
            map[enemy.pos.y][enemy.pos.x] = true;
        }
        map
    }
}

struct Dungeon {
    floors: Vec<DungeonFloor>,
    floor_progress: usize,
}

impl Dungeon {
    fn advance_floor_progress(&mut self) {
        self.floor_progress += 1;
    }
    fn add_floor(&mut self, floor: DungeonFloor) {
        self.floors.push(floor);
    }
    fn current_floor(&self) -> &DungeonFloor {
        &self.floors[self.floor_progress]
    }
    fn current_floor_mut(&mut self) -> &mut DungeonFloor {
        &mut self.floors[self.floor_progress]
    }
    fn prepare_enemies_in_current_floor(&mut self, rng: &mut rand::rngs::ThreadRng) {
        let mut enemy_list: Vec<Enemy> = Vec::new();
        let mut already_used_positions: Vec<Coordinate> = Vec::new();
        let mut how_many_enemies =
            self.floors[self.floor_progress].height * self.floors[self.floor_progress].width;
        if how_many_enemies > (1 + self.floor_progress).pow(2).into() {
            how_many_enemies = 1 + self.floor_progress as usize;
        }
        for _ in 0..rng.gen_range(1..=how_many_enemies) {
            let mut pos = Coordinate {
                x: rng.gen_range(0..self.floors[self.floor_progress].width),
                y: rng.gen_range(0..self.floors[self.floor_progress].height),
            };
            // TODO: fix retry logic
            loop {
                if already_used_positions.contains(&pos) {
                    if random::<bool>() {
                        if pos.x < self.floors[self.floor_progress].width - 1 {
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
                        if pos.y < self.floors[self.floor_progress].height - 1 {
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
                strength: rng.gen_range(1..=(1 + self.floor_progress as u8).pow(2)),
            });
        }
        self.floors[self.floor_progress].enemies = enemy_list;
    }
}

struct ColorTheme {
    floor_color: Color,
    enemy_color: Color,
    player_color: Color,
    fog_of_war_color: Color,
}

struct GameWorld {
    char_for_enemy: char,
    char_for_floor_square: char,
    char_for_player: char,
    char_for_fog_of_war: char,
    dungeon: Dungeon,
    player: Player,
    color_theme: ColorTheme,
}

impl GameWorld {
    fn print_dungeon_map(&self) {
        let width = self.dungeon.current_floor().width;
        let height = self.dungeon.current_floor().height;
        let width_digits = width.to_string().len();
        // prepare 2d vec to display dungeon map
        let mut map_display = vec![vec![self.char_for_floor_square; width]; height];
        for enemy in &self.dungeon.current_floor().enemies {
            map_display[enemy.pos.y][enemy.pos.x] = self.char_for_enemy;
        }
        for fog_of_war in &self.dungeon.current_floor().fog_of_wars {
            match fog_of_war {
                FogOfWar::Coordinate(c) => map_display[c.y][c.x] = self.char_for_fog_of_war,
            };
        }
        map_display[self.player.pos.y][self.player.pos.x] = self.char_for_player;
        // print x-axis ruler
        println!(
            "{}",
            (0..width)
                .map(|x| x.to_string() + &" ".repeat(width_digits - (x.to_string().len())))
                .collect::<Vec<String>>()
                .join("")
        );
        // print map
        for (y, row) in map_display.iter().enumerate() {
            for x in row.iter() {
                queue!(
                    io::stdout(),
                    SetForegroundColor(if x == &self.char_for_floor_square {
                        self.color_theme.floor_color
                    } else if x == &self.char_for_player {
                        self.color_theme.player_color
                    } else if x == &self.char_for_enemy {
                        self.color_theme.enemy_color
                    } else if x == &self.char_for_fog_of_war {
                        self.color_theme.fog_of_war_color
                    } else {
                        Color::Reset
                    }),
                    Print(format!("{}{}", x, " ".repeat(width_digits - 1))),
                    ResetColor
                )
                .unwrap();
                // print!();
            }
            println!("{}", y);
        }
    }
}

fn print_how_to_play() {
    println!("--- How to play ---");
    println!("Enter \"exit\" to exit program");
    println!("Input \"x y\" of your destination (For example, \"12 2\" means go to (12, 2))");
    println!("Input 'a' to combat enemy");
    println!("Input 'help' to see this message");
}

fn main() {
    println!("--- Dungeon Monster Sweeper ---");
    print_how_to_play();
    println!("Press any key to continue");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut gameworld = GameWorld {
        char_for_enemy: 'E',
        char_for_floor_square: '.',
        char_for_player: '@',
        char_for_fog_of_war: '?',
        dungeon: Dungeon {
            floors: vec![],
            floor_progress: 0,
        },
        player: Player {
            pos: Coordinate { x: 0, y: 0 },
            strength: 2,
            hp: 3,
        },
        color_theme: ColorTheme {
            floor_color: Color::DarkYellow,
            enemy_color: Color::Red,
            player_color: Color::Green,
            fog_of_war_color: Color::DarkBlue,
        },
    };
    gameworld.dungeon.add_floor(DungeonFloor {
        width: 16,
        height: 16,
        fog_of_wars: HashSet::new(),
        enemies: Vec::new(),
    });
    let mut rng = rand::thread_rng();
    gameworld.dungeon.prepare_enemies_in_current_floor(&mut rng);
    let re = Regex::new(r"^\d+ \d+$").unwrap();
    'mainloop: loop {
        if gameworld.dungeon.current_floor().enemies.is_empty() {
            gameworld.dungeon.advance_floor_progress();
            if gameworld.dungeon.floor_progress > 0 {
                println!("All enemies are slain! The player goes to the next floor.");
            };
            if gameworld.dungeon.floor_progress > 99 {
                println!("Congratulations! You've reached the final floor!");
                println!("Press any key to exit.");
                io::stdin().read_line(&mut input).unwrap();
                break 'mainloop;
            }
            gameworld.dungeon.add_floor(DungeonFloor {
                width: 16,
                height: 16,
                fog_of_wars: HashSet::new(),
                enemies: Vec::new(),
            });
            for y in 0..gameworld.dungeon.current_floor().height {
                for x in 0..gameworld.dungeon.current_floor().width {
                    gameworld
                        .dungeon
                        .current_floor_mut()
                        .fog_of_wars
                        .insert(FogOfWar::Coordinate(Coordinate { x, y }));
                }
            }
            gameworld.dungeon.prepare_enemies_in_current_floor(&mut rng);
            gameworld.player.pos.x = rng.gen_range(0..gameworld.dungeon.current_floor().width);
            gameworld.player.pos.y = rng.gen_range(0..gameworld.dungeon.current_floor().height);
            gameworld.player.hp += 1;
        }
        // print map
        gameworld.print_dungeon_map();
        // print infomation
        println!(
            "| Current floor: {}F | Player's strength: {} HP: {} | Remaining enemy: {} |",
            gameworld.dungeon.floor_progress,
            gameworld.player.strength,
            gameworld.player.hp,
            gameworld.dungeon.current_floor().enemies.len()
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
            for (enemy_id, enemy) in gameworld.dungeon.current_floor().enemies.iter().enumerate() {
                if enemy.pos.x == gameworld.player.pos.x && enemy.pos.y == gameworld.player.pos.y {
                    is_enemy_found = true;
                    println!(
                        "Player combats enemy at (x,y = {},{})",
                        enemy.pos.x, enemy.pos.y
                    );
                    if rng.gen_ratio(
                        gameworld.player.strength.into(),
                        (gameworld.player.strength + enemy.strength).into(),
                    ) {
                        let gained_strength = rng.gen_range(1..=enemy.strength);
                        println!(
                            "The player triumphed over the enemy! +{} Player's strength points",
                            gained_strength
                        );
                        gameworld.player.strength += gained_strength;
                        gameworld
                            .dungeon
                            .current_floor_mut()
                            .enemies
                            .remove(enemy_id);
                    } else {
                        println!("Player was defeated by enemy! -1 Player's hit points");
                        gameworld.player.hp -= 1;
                        if gameworld.player.hp == 0 {
                            println!("GAMEOVER… Press any key to exit.");
                            io::stdin().read_line(&mut input).unwrap();
                            break 'mainloop;
                        }
                    }
                    break;
                }
            }
            if !is_enemy_found {
                println!("No enemies are found at your current coordinates.");
            }
        } else if re.is_match(command) {
            let mut destination = command.split_whitespace();
            let x: usize = destination.next().unwrap().parse().unwrap();
            let y: usize = destination.next().unwrap().parse().unwrap();
            (gameworld.player.pos.x, gameworld.player.pos.y) =
                if (x < gameworld.dungeon.current_floor().width)
                    && (y < gameworld.dungeon.current_floor().height)
                {
                    // ↑ don't need to worry about negative numbers because they are already checked in the regular expression.
                    println!(
                        "Player moved to (x,y = {},{}) from (x,y = {},{})",
                        x, y, gameworld.player.pos.x, gameworld.player.pos.y
                    );
                    // 移動元座標から移動先座標のfog of warを明らかにする
                    mut_each_step_of_line_drawing(
                        gameworld.player.pos.x,
                        gameworld.player.pos.y,
                        x,
                        y,
                        &mut |x, y| {
                            gameworld.dungeon.current_floor_mut().fog_of_wars.remove(
                                &FogOfWar::Coordinate(Coordinate {
                                    x: x as usize,
                                    y: y as usize,
                                }),
                            );
                        },
                    );
                    // draw_line(&mut fog_of_war_map, player.pos.x, player.pos.y, x, y, false);
                    for enemy in &gameworld.dungeon.current_floor().enemies {
                        if enemy.pos.x == x && enemy.pos.y == y {
                            println!(
                                "Player found enemy at (x,y = {},{}). It's strength is {}. ",
                                enemy.pos.x, enemy.pos.y, enemy.strength
                            );
                            println!(
                                "Player's winning percentage against the foe is {}%",
                                (gameworld.player.strength as f64
                                    / (gameworld.player.strength + enemy.strength) as f64)
                                    * 100.0
                            );
                        }
                    }
                    (x, y)
                } else {
                    println!("The destination coordinate is invalid.");
                    continue;
                };
        };
    }
}
