use rand::Rng;

enum CellKind {
    Normal,
    Rosetta
}

enum CellVal {
    P1,
    P2,
    Empty
}

fn pp_cell (c : &(CellKind,CellVal)) {
    match c.0 {
        CellKind::Normal =>
            match c.1 {
                CellVal::P1 => print!("|1 o|"),
                CellVal::P2 => print!("|2 o|"),
                CellVal::Empty => print!("|o|"),
            },
        CellKind::Rosetta =>
            match c.1 {
                CellVal::P1 => print!("|1 o|"),
                CellVal::P2 => print!("|2 o|"),
                CellVal::Empty => print!("|o|"),
            }
    }
}

//a token belonging to a player.
//position refers to the absolute
//position on the game board:
//that is to say the concatenation
//of the four first safe cells,
//the battle cells and the two last
//safe cells

struct Player {
    points: u32, //number of points
    safe_line : [(CellKind,CellVal);6],
    //the safe part of the board, where the players can safely move
    //first four cells are the starting point, then the player are
    //moving to the battle line
}

fn new_player () -> Player {
    Player {points: 0,
    safe_line:
        [
        (CellKind::Normal,CellVal::Empty),
        (CellKind::Normal,CellVal::Empty),
        (CellKind::Normal,CellVal::Empty),
        (CellKind::Rosetta,CellVal::Empty),
        (CellKind::Normal,CellVal::Empty),
        (CellKind::Normal,CellVal::Empty),
        ],}
}

struct Board {
    battle_line : [(CellKind,CellVal);8], //the line where the players fight
}

fn new_board () -> Board {
    Board {battle_line:
        [
        (CellKind::Normal,CellVal::Empty),
        (CellKind::Normal,CellVal::Empty),
        (CellKind::Normal,CellVal::Empty),
        (CellKind::Rosetta,CellVal::Empty),
        (CellKind::Normal,CellVal::Empty),
        (CellKind::Normal,CellVal::Empty),
        (CellKind::Normal,CellVal::Empty),
        (CellKind::Normal,CellVal::Empty),
        ]}
}

fn pp_board (b: Board, p1: Player, p2: Player) {
    let s_p2 = p2.safe_line;
    let start_p2 = &s_p2[0..4];
    let end_p2 = &s_p2[4..6];
    println!(" _  _  _  _     _  _");
    for cell in start_p2{
        pp_cell(&cell);
    }
    print!("   ");
    for cell in end_p2{
        pp_cell(&cell);
    }
    println!("");
    println!(" _  _  _  _     _  _");
    println!(" _  _  _  _  _  _  _  _");
    let b_l = b.battle_line;
    for cell in &b_l {
        pp_cell(&cell);
    }
    println!("");
    println!(" _  _  _  _  _  _  _  _");
    let s_p1 = p1.safe_line;
    let start_p1 = &s_p1[0..4];
    let end_p1 = &s_p1[4..6];
    println!(" _  _  _  _     _  _");
    for cell in start_p1{
        pp_cell(&cell);
    }
    print!("   ");
    for cell in end_p1{
        pp_cell(&cell);
    }
    println!("");
    println!(" _  _  _  _     _  _");
}

fn roll_dice () -> u32 {
    let d1 = rand::thread_rng().
        gen_range(0, 1);
    let d2 = rand::thread_rng().
        gen_range(0, 1);
    let d3 = rand::thread_rng().
        gen_range(0, 1);
    let d4 = rand::thread_rng().
        gen_range(0, 1);
    d1 + d2 + d3 + d4 
    
}

//TODO: add moving logic: when rolling a dice,
//either choose to move existing token or add
//another (so it is needed to scan player's
//safe line and the board to see if there are
//any tokens)

fn main() {
    println!("Let us start a game!");
    println!("Player 1 start.");
    let p1 = new_player();
    let p2 = new_player();
    let board = new_board();
    pp_board(board, p1, p2);
}
