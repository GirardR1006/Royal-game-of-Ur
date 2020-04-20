//TODO: rework and simplify the data
//structures to not call unnecessary references
//like some annoying dudes trying to convince
//you that watching the office means having a
//personality

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
    fn check_possible_move(&self, id: &CellVal) -> PossibleMove {
        match &self.1 == id {
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
            id  : id,
        }
    }
}
struct Game {
    //the battle part of the board where the players
    //fight against each other
    b_line:[Cell;8],
    //the safe part of the board, where the players can safely move
    //first four cells are the starting point, then the player are
    //moving to the battle line
    p1_line:[Cell;6],
    p2_line:[Cell;6],
    p1:Player,
    p2:Player
}
impl Game {
    fn new_game () -> Game{
        Game {b_line:[Cell(CellKind::Normal,CellVal::Empty),
        Cell(CellKind::Normal,CellVal::Empty),
        Cell(CellKind::Rosetta,CellVal::Empty),
        Cell(CellKind::Normal,CellVal::Empty),
        Cell(CellKind::Normal,CellVal::Empty),
        Cell(CellKind::Normal,CellVal::Empty),
        Cell(CellKind::Normal,CellVal::Empty),
        Cell(CellKind::Normal,CellVal::Empty),
        ],
        p1_line:[Cell(CellKind::Normal,CellVal::Empty),
        Cell(CellKind::Normal,CellVal::Empty),
        Cell(CellKind::Normal,CellVal::Empty),
        Cell(CellKind::Rosetta,CellVal::Empty),
        Cell(CellKind::Normal,CellVal::Empty),
        Cell(CellKind::Rosetta,CellVal::Empty),
        ],
        p2_line:[Cell(CellKind::Normal,CellVal::Empty),
        Cell(CellKind::Normal,CellVal::Empty),
        Cell(CellKind::Normal,CellVal::Empty),
        Cell(CellKind::Rosetta,CellVal::Empty),
        Cell(CellKind::Normal,CellVal::Empty),
        Cell(CellKind::Rosetta,CellVal::Empty),
        ],
        p1:Player::new_player(1),
        p2:Player::new_player(2)
        }
    }
    // Scan the player's safe line, 
    // the board's battle line, and return
    // the cell slices corresponding to the
    // potential moves for existing tokens
    fn potential_moves(&self, p: &Player) ->
        (std::vec::Vec<usize>,std::vec::Vec<usize>) {
            let p_safe = match p.id {
                CellVal::P1 => &self.p1_line,
                CellVal::P2 => &self.p2_line,
            };
            let b_line = &self.b_line;
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
    // Move on a cell assumed to be either
    // free or with an enemy on it
    fn do_move(&mut self, p:&Player, c:&mut Cell){
        match c.1 == p.id {
            true => panic!("Error, this cell already
            has one of your token"),
            // replace current value by the cell
            // by the player's id
            false => {c.1 = p.id;},
        }
    }
    // Select the next action to take between
    // moving and adding a new token
    // Currently just pick the first
    // non-empty safe move, then battle move,
    // then place a token
    // TODO: rewrite to not use double mutable
    // borrow
    fn select_and_move(&mut self, p:&mut Player,
                       s_moves:&Vec<usize>,
                       b_moves:&Vec<usize>)
        -> Result<(),usize>{
            match s_moves[..].first() {
                None => {
                    match b_moves[..].first() {
                        None => self.place_new_token(p),
                        Some(idx) => Ok(self.do_move(p,&mut self.b_line[*idx])),
                    }
                },
                Some(idx) => Ok(self.do_move(p,&mut p.safe_line[*idx])),
            }
        }
    // Function placing a new token for player p
    fn place_new_token(&mut self, id: &CellVal) -> Result<(),usize>{
        let roll = roll_dice();
        match roll {
            0 => Ok (()),
            i => {
                let target_cell = match id {
                    CellVal::P1 => self.p1_line[i-1],
                    CellVal::P2 => self.p2_line[i-1],
                    CellVal::Empty => panic!("lol"),
                };
                match target_cell.check_possible_move(id) {
                    PossibleMove::Yes => {
                        Ok(target_cell.1 = id)
                    },
                    //reroll if on rosetta, and go directly on battle line
                    PossibleMove::YesReroll =>
                        match roll_dice() {
                            // if the new roll gives us nothing, move to the 
                            // final safe_line
                            0 => Ok(p.safe_line[3] = Cell(p.safe_line[3].0,p_id)),
                            i => match self.b_line[i-1].check_possible_move(&p) {
                                PossibleMove::Yes | PossibleMove::YesReroll =>
                                    Ok (self.b_line[i-1] =
                                        Cell(self.b_line[i-1].0,p_id)),
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
        let s_p2 = &self.p2_line;
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
        let b_l = &self.b_line;
        for cell in b_l {
            cell.pp_cell();
        }
        println!("");
        println!(" _  _  _  _  _  _  _  _");
        let s_p1 = &self.p1_line;
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
    println!("Roll result: {}", roll);
    roll  
}

fn main() {
    println!("Let us start a game!");
    let mut game = Game::new_game();
    &game.pp_game();
    println!("Player 1 starts and places a token");
    //first move is always the same: players put a new token
    Game::place_new_token(&mut game, &mut game.p1);
    &game.pp_game();
    println!("Player 2 starts and places a token");
    Game::place_new_token(&mut game, &mut game.p2);
    &game.pp_game();
    let (s_moves,b_moves) = game.potential_moves(& game.p1);
    //For now just select the first possible move, and proceed

}
