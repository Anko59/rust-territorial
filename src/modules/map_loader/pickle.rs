use std::error::Error;
use ndarray::Array2;
use log::debug;
use ndarray_npy::ReadNpyExt;
use std::fs::File;

pub struct PickleLoader;

impl PickleLoader {
    pub fn load_pickle_file(path: &str) -> Result<Array2<f64>, Box<dyn Error>> {
        // Convert path from .pkl to .npy
        let npy_path = path.replace(".pkl", ".npy");
        debug!("Loading npy file: {}", npy_path);
        
        // Open the file
        let reader = File::open(&npy_path)?;
        
        // Read the array directly from the .npy file
        let array: Array2<f64> = Array2::read_npy(reader)?;
        
        let (height, width) = array.dim();
        debug!("Loaded array with dimensions: {}x{}", height, width);
        
        // Print some stats to verify the data
        let mut min_val = f64::INFINITY;
        let mut max_val = f64::NEG_INFINITY;
        for &x in array.iter() {
            if x != -999.0 {
                min_val = min_val.min(x);
                max_val = max_val.max(x);
            }
        }
        debug!("Min value (excluding -999): {}", min_val);
        debug!("Max value (excluding -999): {}", max_val);
        
        Ok(array)
    }
}
