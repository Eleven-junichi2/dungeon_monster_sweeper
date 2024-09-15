use std::io;

// fn generate_map() ->  {}


fn main() {
    let floor_progress: u128 = 0;
    println!("--- Dungeon Monster Sweeper ---");
    println!("HighScore: 未実装");
    println!("Press any key to continue");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input);
    println!("--- How to play ---");
    println!("Press \"escape key\" to exit from the game");
    println!("Input \"x y\" of your destination (For example, \"12 2\" means go to (12, 2))");
    println!("Press any key to continue");
    io::stdin().read_line(&mut input);
    // マップ生成
    // 生成したマップを表示
    // 現在の階を表示
    // 操作方法を表示
}
