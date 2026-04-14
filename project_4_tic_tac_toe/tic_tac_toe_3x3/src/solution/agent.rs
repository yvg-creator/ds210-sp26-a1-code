use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

pub struct SolutionAgent {}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let (best_score, best_move) = minimax(board, player, true);
        
        if let Some((x, y)) = best_move {
            (best_score, x, y)
        } else {
            (0, 0, 0)
        }
    }
}

fn minimax(board: &Board, player: Player, is_root: bool) -> (i32, Option<(usize, usize)>) {
    if board.game_over() {
        let score = match board.score() as i32 {
            1 => 1,
            -1 => -1,
            _ => 0,
        };
        return (score, None);
    }
    
    let moves = board.moves();
    if moves.is_empty() {
        return (0, None);
    }
    
    let is_maximizing = matches!(player, Player::X);
    let mut best_score = if is_maximizing { i32::MIN } else { i32::MAX };
    let mut best_move = None;
    
    for &mov in &moves {
        let mut new_board = board.clone();
        new_board.apply_move(mov, player);
        
        let next_player = match player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
        
        let (score, _) = minimax(&new_board, next_player, false);
        
        if is_maximizing {
            if score > best_score {
                best_score = score;
                if is_root {
                    best_move = Some(mov);
                }
            }
        } else {
            if score < best_score {
                best_score = score;
                if is_root {
                    best_move = Some(mov);
                }
            }
        }
    }
    
    (best_score, best_move)
}
