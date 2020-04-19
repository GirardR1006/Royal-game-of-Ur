use rand::Rng;

#[derive(Copy, Clone)]
enum CellKind {
    Normal,
    Rosetta
}

#[derive(Copy, Clone)]
enum CellVal {
    P1,
    P2,
    Empty
}
impl PartialEq for CellVal {
    fn eq(&self, other: &Self) -> bool {
        match self {
            CellVal::P1 => match other {
                CellVal::P1 => true,
                _ => false
            },
            CellVal::P2 => match other {
                CellVal::P2 => true,
                _ => false
            },
            CellVal::Empty => match other {
                CellVal::Empty => true,
                _ => false
            },
        }
    }
}
//Possible move when considering
//a cell from a player point of view
enum PossibleMove {
    Yes,
    CanEat,
    Blocked,
    YesReroll,
}
#[derive(Copy, Clone)]
struct Cell (CellKind,CellVal);
impl Cell {
    fn is_empty (&self) -> bool{
        self.1 == CellVal::Empty
    }
    fn check_possible_move(&self, p: &Player) -> PossibleMove {
        let p_id = p.id;
        match self.1 == p_id {
            true => PossibleMove::Blocked,
            false => match &self.1 {
                CellVal::Empty => match self.0 {
                    CellKind::Normal =>
                        PossibleMove::Yes,
                    CellKind::Rosetta =>
                        PossibleMove::YesReroll,
                }
                _ => PossibleMove::CanEat,
            }
        }
    }
    fn pp_cell (&self) {
        match self.0 {
            CellKind::Normal =>
                match self.1 {
                    CellVal::P1 => print!("|1|"),
                    CellVal::P2 => print!("|2|"),
                    CellVal::Empty => print!("|x|"),
                },
            CellKind::Rosetta =>
                match self.1 {
                    CellVal::P1 => print!("|1|"),
                    CellVal::P2 => print!("|2|"),
                    CellVal::Empty => print!("|x|"),
                }
        }
    }
}

struct Player {
    points: u32, //number of points
    safe_line : [Cell;6],
    //the safe part of the board, where the players can safely move
    //first four cells are the starting point, then the player are
    //moving to the battle line
    id : CellVal
}
impl Player {
    fn new_player (i: u32) -> Player {
        let id = match i {
            1 => CellVal::P1,
            2 => CellVal::P2,
            _ => panic!("Error, provide either
            1 for player 1 or 2 for player 2 in
            new_player"),
        };
        Player {points: 0,
        safe_line:
            [
            Cell(CellKind::Normal,CellVal::Empty),
            Cell(CellKind::Normal,CellVal::Empty),
            Cell(CellKind::Normal,CellVal::Empty),
            Cell(CellKind::Rosetta,CellVal::Empty),
            Cell(CellKind::Normal,CellVal::Empty),
            Cell(CellKind::Rosetta,CellVal::Empty),
            ],
            id  : id,
        }
    }
}
struct Board {
    battle_line : [Cell;8], //the line where the players fight
}
impl Board{
    fn new_board () -> Board {
        Board {battle_line:
            [
                Cell(CellKind::Normal,CellVal::Empty),
                Cell(CellKind::Normal,CellVal::Empty),
                Cell(CellKind::Rosetta,CellVal::Empty),
                Cell(CellKind::Normal,CellVal::Empty),
                Cell(CellKind::Normal,CellVal::Empty),
                Cell(CellKind::Normal,CellVal::Empty),
                Cell(CellKind::Normal,CellVal::Empty),
                Cell(CellKind::Normal,CellVal::Empty),
            ]}
    }

    fn pp_board (&self, p1: &Player, p2: &Player) {
        let s_p2 = &p2.safe_line;
        let start_p2 = &s_p2[0..4];
        let end_p2 = &s_p2[4..6];
        println!(" _  _  _  _     _  _");
        for cell in start_p2.iter().rev() {
            cell.pp_cell();
        }
        print!("   ");
        for cell in end_p2.iter().rev() {
            cell.pp_cell();
        }
        println!("");
        println!(" _  _  _  _     _  _");
        println!(" _  _  _  _  _  _  _  _");
        let b_l = &self.battle_line;
        for cell in b_l {
            cell.pp_cell();
        }
        println!("");
        println!(" _  _  _  _  _  _  _  _");
        let s_p1 = &p1.safe_line;
        let start_p1 = &s_p1[0..4];
        let end_p1 = &s_p1[4..6];
        println!(" _  _  _  _     _  _");
        for cell in start_p1.iter().rev() {
            cell.pp_cell();
        }
        print!("   ");
        for cell in end_p1.iter().rev() {
            cell.pp_cell();
        }
        println!("");
        println!(" _  _  _  _     _  _");
    }
} 

struct Game {
    board:Board,
    p1:Player,
    p2:Player
}
impl Game {
    fn new_game () -> Game{
        Game {board:Board::new_board(),
        p1:Player::new_player(1),
        p2:Player::new_player(2)
        }
    }
    // Scan the player's safe line, 
    // the board's battle line, and return
    // the cell slices corresponding to the
    // potential moves for existing tokens
    fn potential_move(&self, p: &Player) ->
        (std::vec::Vec<usize>,std::vec::Vec<usize>) {
        let p_safe = &p.safe_line;
        let b_line = &self.board.battle_line;
        //will store a list of index in safe_line and battle line
        let mut s_moves = Vec::new();
        let mut b_moves = Vec::new();
        for (i,c) in p_safe.iter().enumerate(){
            match &c.check_possible_move(p) {
                PossibleMove::Yes | PossibleMove::YesReroll => s_moves.push(i),
                _ => (),
            }
        }
        for (i,c) in b_line.iter().enumerate(){
            match &c.check_possible_move(p) {
                PossibleMove::Yes | PossibleMove::YesReroll => b_moves.push(i),
                _ => (),
            }
        }
        (s_moves,b_moves)
    }
    // Function placing a new token for player p
    fn place_new_token(p:&mut Player, b: &mut Board) -> Result<(),usize>{
        let roll = roll_dice();
        match roll {
            0 => Ok (()),
            i => {
                let p_id = p.id.clone();
                let target_cell = p.safe_line[i-1].clone();
                match target_cell.check_possible_move(&p) {
                    PossibleMove::Yes => {
                        Ok(p.safe_line[i-1] = Cell(target_cell.0,p_id))
                    },
                    //reroll if on rosetta, and go directly on battle line
                    PossibleMove::YesReroll =>
                        match roll_dice() {
                        // if the new roll gives us nothing, move to the 
                        // final safe_line
                        0 => Ok(p.safe_line[3] = Cell(p.safe_line[3].0,p_id)),
                        i => match b.battle_line[i-1].check_possible_move(&p) {
                            PossibleMove::Yes | PossibleMove::YesReroll =>
                                Ok (b.battle_line[i-1] = Cell(b.battle_line[i-1].0,p_id)),
                            PossibleMove::Blocked => Err(i-1),
                            PossibleMove::CanEat => Err(i-1), //TODO: implement eating
                        }
                    }
                PossibleMove::Blocked => Err(roll),
                PossibleMove::CanEat => Err(roll), //should never happen
                }
            }
        }
    }
    fn pp_game(&self) {
        Board::pp_board(&self.board,&self.p1,&self.p2);
    }
}


fn roll_dice () -> usize {
    let d1 = rand::thread_rng().
        gen_range(0, 2);
    let d2 = rand::thread_rng().
        gen_range(0, 2);
    let d3 = rand::thread_rng().
        gen_range(0, 2);
    let d4 = rand::thread_rng().
        gen_range(0, 2);
    let roll = d1 + d2 + d3 + d4;
    println!("roll result: {}", roll);
    roll  
}

fn main() {
    println!("Let us start a game!");
    let mut game = Game::new_game();
    &game.pp_game();
    println!("Player 1 starts and place a token");
    //first move is always the same: players put a new token
    let _ = Game::place_new_token(&mut game.p1, &mut game.board);
    &game.pp_game();
    println!("Player 2 starts and place a token");
    let _ = Game::place_new_token(&mut game.p2, &mut game.board);
    &game.pp_game();

}
