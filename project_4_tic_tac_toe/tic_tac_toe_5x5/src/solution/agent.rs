use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;
use tic_tac_toe_stencil::agents::Agent;
use std::cmp::{min, max};

pub struct SolutionAgent;

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let max_depth = 2;
        
        let moves = board.moves();
        if moves.is_empty() {
            return (0, 0, 0);
        }
        
        let is_maximizing = player == Player::X;
        let (_, best_move) = minimax(board, max_depth, is_maximizing, i32::MIN, i32::MAX);
        
        match best_move {
            Some((row, col)) => {
                let score = board.score();
                (score, row, col)
            }
            None => {
                (0, moves[0].0, moves[0].1)
            }
        }
    }
}

fn heuristic(board: &Board) -> i32 {
    board.score()
}

fn minimax(
    board: &Board, 
    depth: u32, 
    is_maximizing: bool,
    mut alpha: i32,
    mut beta: i32,
) -> (i32, Option<(usize, usize)>) {
    if board.game_over() {
        return (board.score(), None);
    }
    
    if depth == 0 {
        return (heuristic(board), None);
    }
    
    let moves = board.moves();
    if moves.is_empty() {
        return (board.score(), None);
    }
    
    if is_maximizing {
        let mut max_eval = i32::MIN;
        let mut best_move = None;
        
        for &mov in &moves {
            let mut new_board = board.clone();
            new_board.apply_move(mov, Player::X);
            
            let (eval, _) = minimax(&new_board, depth - 1, false, alpha, beta);
            
            if eval > max_eval {
                max_eval = eval;
                best_move = Some(mov);
            }
            
            alpha = max(alpha, eval);
            if beta <= alpha {
                break;
            }
        }
        (max_eval, best_move)
    } else {
        let mut min_eval = i32::MAX;
        let mut best_move = None;
        
        for &mov in &moves {
            let mut new_board = board.clone();
            new_board.apply_move(mov, Player::O);
            
            let (eval, _) = minimax(&new_board, depth - 1, true, alpha, beta);
            
            if eval < min_eval {
                min_eval = eval;
                best_move = Some(mov);
            }
            
            beta = min(beta, eval);
            if beta <= alpha {
                break;
            }
        }
        (min_eval, best_move)
    }
}
