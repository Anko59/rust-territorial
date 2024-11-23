use ndarray::Array2;
use log::debug;

pub struct MapTransformer;

impl MapTransformer {
    pub fn apply_gall_peters_projection(map_data: &Array2<f64>) -> Array2<f64> {
        debug!("Applying Gall-Peters projection");
        let (height, width) = map_data.dim();
        let mut projected_map = Array2::zeros((height, width));

        for y in 0..height {
            let lat = std::f64::consts::PI * (y as f64 / height as f64 - 0.5);
            let y_proj = ((lat.sin() + 1.0) * height as f64 / 2.0) as usize;
            
            if y_proj < height {
                for x in 0..width {
                    projected_map[[y_proj, x]] = map_data[[y, x]];
                }
            }
        }

        debug!("Interpolating missing values");
        for y in 0..height {
            if projected_map.row(y).iter().all(|&x| x == 0.0) {
                let mut above = y as i32 - 1;
                let mut below = y + 1;
                
                while above >= 0 && projected_map.row(above as usize).iter().all(|&x| x == 0.0) {
                    above -= 1;
                }
                while below < height && projected_map.row(below).iter().all(|&x| x == 0.0) {
                    below += 1;
                }
                
                if above >= 0 && below < height {
                    for x in 0..width {
                        projected_map[[y, x]] = (
                            projected_map[[above as usize, x]] + 
                            projected_map[[below, x]]
                        ) / 2.0;
                    }
                }
            }
        }

        debug!("Projection complete");
        projected_map
    }

    pub fn resize_map(map: &Array2<f64>, new_width: usize, new_height: usize) -> Array2<f64> {
        debug!("Resizing map from {:?} to {}x{}", map.dim(), new_width, new_height);
        let (old_height, old_width) = map.dim();
        let mut resized = Array2::zeros((new_height, new_width));
        
        for y in 0..new_height {
            for x in 0..new_width {
                let old_x = (x as f64 * old_width as f64 / new_width as f64) as usize;
                let old_y = (y as f64 * old_height as f64 / new_height as f64) as usize;
                resized[[y, x]] = map[[old_y, old_x]];
            }
        }
        
        debug!("Resizing complete");
        resized
    }
}
