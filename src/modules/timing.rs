use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::sleep;

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

    pub fn get_and_clear_averages(&mut self) -> HashMap<String, f64> {
        let mut averages = HashMap::new();
        
        for (name, times) in self.execution_times.drain() {
            if !times.is_empty() {
                let total = times.iter().sum::<Duration>();
                let avg = total.as_secs_f64() / times.len() as f64;
                averages.insert(name, avg);
            }
        }
        
        averages
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
        let averages = guard.get_and_clear_averages();
        
        if !averages.is_empty() {
            println!("\n=== Performance Statistics (last 60s) ===");
            // Sort by name for consistent output
            let mut sorted_stats: Vec<_> = averages.into_iter().collect();
            sorted_stats.sort_by(|a, b| a.0.cmp(&b.0));
            
            for (name, avg) in sorted_stats {
                println!("{:<30} {:.4} ms", name, avg * 1000.0);
            }
            println!("=======================================\n");
        }
    }
}

#[macro_export]
macro_rules! time_execution {
    ($stats:expr, $name:expr) => {
        let _timer = $crate::modules::timing::ExecutionTimer::new($stats.clone(), $name);
    };
}
