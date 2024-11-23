use ndarray::Array2;
use log::debug;
use crate::modules::config::{MAX_TEMPERATURE, MIN_TEMPERATURE, TEMPERATURE_LAPSE_RATE};

pub struct TemperatureGenerator;

impl TemperatureGenerator {
    pub fn generate_temperature_map(lat_grid: &Array2<f64>, elevation_map: &Array2<f64>) -> Array2<f64> {
        debug!("Generating temperature map");
        let (height, width) = lat_grid.dim();
        let mut temp_map = Array2::zeros((height, width));
        
        let temp_range = MAX_TEMPERATURE - MIN_TEMPERATURE;
        
        for y in 0..height {
            for x in 0..width {
                let normalized_lat = lat_grid[[y, x]] / 90.0;
                let base_temp = MAX_TEMPERATURE - temp_range * normalized_lat.abs();
                let elevation = elevation_map[[y, x]];
                temp_map[[y, x]] = base_temp - elevation.max(0.0) * TEMPERATURE_LAPSE_RATE;
            }
        }
        
        debug!("Temperature map generation complete");
        temp_map
    }
}
