use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::sleep;
use log::info;

#[derive(Default)]
pub struct TimingStats {
    execution_times: HashMap<String, Vec<Duration>>,
}

impl TimingStats {
    pub fn new() -> Self {
        Self {
            execution_times: HashMap::new(),
        }
    }

    pub fn record_execution(&mut self, name: &str, duration: Duration) {
        self.execution_times
            .entry(name.to_string())
            .or_default()
            .push(duration);
    }

    pub fn get_and_clear_totals(&mut self) -> HashMap<String, (f64, usize)> {
        let mut totals = HashMap::new();
        
        for (name, times) in self.execution_times.drain() {
            if !times.is_empty() {
                let total = times.iter().sum::<Duration>();
                totals.insert(name, (total.as_secs_f64(), times.len()));
            }
        }
        
        totals
    }
}

pub struct ExecutionTimer {
    stats: Arc<RwLock<TimingStats>>,
    name: String,
    start: Instant,
}

impl ExecutionTimer {
    pub fn new(stats: Arc<RwLock<TimingStats>>, name: &str) -> Self {
        Self {
            stats,
            name: name.to_string(),
            start: Instant::now(),
        }
    }
}

impl Drop for ExecutionTimer {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        let name = self.name.clone();
        let stats = self.stats.clone();
        
        tokio::spawn(async move {
            let mut guard = stats.write().await;
            guard.record_execution(&name, duration);
        });
    }
}

pub async fn start_timing_logger(stats: Arc<RwLock<TimingStats>>) {
    const LOG_INTERVAL: Duration = Duration::from_secs(60);

    loop {
        sleep(LOG_INTERVAL).await;
        
        let mut guard = stats.write().await;
        let totals = guard.get_and_clear_totals();
        
        if !totals.is_empty() {
            info!("\n=== Performance Statistics (last 60s) ===");
            // Sort by total execution time (descending)
            let mut sorted_stats: Vec<_> = totals.into_iter().collect();
            sorted_stats.sort_by(|a, b| b.1.0.partial_cmp(&a.1.0).unwrap());
            
            for (name, (total_secs, count)) in sorted_stats {
                info!("{:<30} Total: {:.4} ms, Count: {}", 
                    name, 
                    total_secs * 1000.0,
                    count
                );
            }
            info!("=======================================\n");
        }
    }
}

#[macro_export]
macro_rules! time_execution {
    ($stats:expr, $name:expr) => {
        let _timer = $crate::modules::timing::ExecutionTimer::new($stats.clone(), $name);
    };
}
