use crate::modules::config::*;
use crate::modules::types::Grid;
use crate::modules::timing::ExecutionTimer;
use crate::TIMING_STATS;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;
use base16ct;

#[derive(serde::Serialize)]
pub struct GridMessage {
    pub grid: String,
    pub width: usize,
    pub height: usize,
}

pub fn serialize_grid(grid: &Grid) -> GridMessage {
    let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "grid_serialization");
    
    // Pre-allocate buffer for flattened grid
    let mut flat_grid = Vec::with_capacity(GRID_WIDTH * GRID_HEIGHT);
    
    // Flatten grid to bytes, using 255 for None
    for row in grid {
        for &cell in row {
            flat_grid.push(cell.unwrap_or(255) as u8);
        }
    }
    
    // Compress using zlib
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&flat_grid).unwrap();
    let compressed = encoder.finish().unwrap();
    
    // Convert to hex
    let mut hex_buf = vec![0u8; base16ct::encoded_len(&compressed)];
    base16ct::upper::encode(&compressed, &mut hex_buf).unwrap();
    
    // Convert to string
    GridMessage {
        grid: String::from_utf8(hex_buf).unwrap(),
        width: GRID_WIDTH,
        height: GRID_HEIGHT,
    }
}
