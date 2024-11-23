use std::collections::HashSet;
use crate::modules::config::*;
use crate::modules::types::{Grid, GridCell};

#[derive(Clone)]
pub struct NeighborPair {
    pub source: usize,
    pub target: usize,
}

pub fn find_all_possible_targets(grid: &Grid) -> Vec<NeighborPair> {
    let mut pairs = Vec::new();
    
    for y in (0..GRID_HEIGHT).step_by(GRID_REDUCTION_FACTOR) {
        for x in (0..GRID_WIDTH).step_by(GRID_REDUCTION_FACTOR) {
            let source_id = grid[y][x];
            let directions = [
                (0, -(GRID_REDUCTION_FACTOR as i32)), 
                (GRID_REDUCTION_FACTOR as i32, 0)
            ];
            
            for &(dx, dy) in &directions {
                let new_x = x as i32 + dx;
                let new_y = y as i32 + dy;
                
                if new_x >= 0 && new_x < GRID_WIDTH as i32 && 
                    new_y >= 0 && new_y < GRID_HEIGHT as i32 {
                    let new_x = new_x as usize;
                    let new_y = new_y as usize;
                    
                    if grid[new_y][new_x] != GridCell::Mountain && grid[new_y][new_x] != GridCell::Water {
                        let target_id = grid[new_y][new_x];
                        if source_id != target_id {
                            pairs.push(NeighborPair {
                                source: source_id,
                                target: target_id,
                            });
                        }
                    }
                }
            }
        }
    }
    
    let mut unique_pairs = HashSet::new();
    for pair in pairs {
        let mut sorted = vec![pair.source, pair.target];
        sorted.sort_unstable();
        unique_pairs.insert((sorted[0], sorted[1]));
    }
    
    unique_pairs.into_iter()
        .map(|(source, target)| NeighborPair { source, target })
        .collect()
}

pub fn get_neighbors(neighbor_pairs: &[NeighborPair], player_id: isize) -> Vec<usize> {
    neighbor_pairs.iter()
        .filter_map(|pair| {
            if pair.source == player_id {
                Some(pair.target)
            } else if pair.target == player_id {
                Some(pair.source)
            } else {
                None
            }
        })
        .collect()
}
