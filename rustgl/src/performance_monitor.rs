use gl::types::*;
use std::collections::HashMap;

/// GPU timer query for accurate performance measurement
pub struct GpuTimer {
    query_start: GLuint,
    query_end: GLuint,
    last_time_ns: Option<u64>,
    available: bool,
}

impl GpuTimer {
    pub fn new() -> Self {
        let mut query_start = 0;
        let mut query_end = 0;
        unsafe {
            gl::GenQueries(1, &mut query_start);
            gl::GenQueries(1, &mut query_end);
        }
        GpuTimer {
            query_start,
            query_end,
            last_time_ns: None,
            available: true,
        }
    }

    /// Start timing - call before rendering
    pub fn begin(&mut self) {
        unsafe {
            gl::QueryCounter(self.query_start, gl::TIMESTAMP);
        }
    }

    /// End timing - call after rendering
    pub fn end(&mut self) {
        unsafe {
            gl::QueryCounter(self.query_end, gl::TIMESTAMP);
        }
        self.available = false;
    }

    /// Try to retrieve results (non-blocking)
    /// Returns true if results were available
    pub fn try_collect(&mut self) -> bool {
        if self.available {
            return true;
        }

        unsafe {
            let mut available_end = 0i32;
            gl::GetQueryObjectiv(self.query_end, gl::QUERY_RESULT_AVAILABLE, &mut available_end);

            if available_end != 0 {
                let mut time_start = 0u64;
                let mut time_end = 0u64;
                gl::GetQueryObjectui64v(self.query_start, gl::QUERY_RESULT, &mut time_start);
                gl::GetQueryObjectui64v(self.query_end, gl::QUERY_RESULT, &mut time_end);

                self.last_time_ns = Some(time_end.saturating_sub(time_start));
                self.available = true;
                true
            } else {
                false
            }
        }
    }

    /// Get last measured time in milliseconds
    pub fn get_time_ms(&self) -> f32 {
        self.last_time_ns.unwrap_or(0) as f32 / 1_000_000.0
    }

    /// Get last measured time in nanoseconds
    pub fn get_time_ns(&self) -> u64 {
        self.last_time_ns.unwrap_or(0)
    }
}

impl Drop for GpuTimer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteQueries(1, &self.query_start);
            gl::DeleteQueries(1, &self.query_end);
        }
    }
}

/// Tracks performance for a specific named operation
pub struct PerformanceCounter {
    timer: GpuTimer,
    // Rolling average over last N frames
    history: Vec<f32>,
    history_size: usize,
    current_index: usize,
}

impl PerformanceCounter {
    pub fn new(history_size: usize) -> Self {
        PerformanceCounter {
            timer: GpuTimer::new(),
            history: vec![0.0; history_size],
            history_size,
            current_index: 0,
        }
    }

    pub fn begin(&mut self) {
        self.timer.begin();
    }

    pub fn end(&mut self) {
        self.timer.end();
    }

    pub fn update(&mut self) -> bool {
        if self.timer.try_collect() {
            let time_ms = self.timer.get_time_ms();
            self.history[self.current_index] = time_ms;
            self.current_index = (self.current_index + 1) % self.history_size;
            true
        } else {
            false
        }
    }

    pub fn get_avg_ms(&self) -> f32 {
        let sum: f32 = self.history.iter().sum();
        sum / self.history_size as f32
    }

    pub fn get_last_ms(&self) -> f32 {
        let prev_index = if self.current_index == 0 {
            self.history_size - 1
        } else {
            self.current_index - 1
        };
        self.history[prev_index]
    }
}

/// Central performance monitoring system
/// Automatically tracks all registered counters
pub struct PerformanceMonitor {
    counters: HashMap<String, PerformanceCounter>,
    history_size: usize,
    enabled: bool,
}

impl PerformanceMonitor {
    pub fn new(history_size: usize) -> Self {
        PerformanceMonitor {
            counters: HashMap::new(),
            history_size,
            enabled: true,
        }
    }

    /// Register or get a counter by name (auto-creates if doesn't exist)
    fn ensure_counter(&mut self, name: &str) -> &mut PerformanceCounter {
        self.counters
            .entry(name.to_string())
            .or_insert_with(|| PerformanceCounter::new(self.history_size))
    }

    /// Start timing for a named operation
    pub fn begin(&mut self, name: &str) {
        if !self.enabled {
            return;
        }
        self.ensure_counter(name).begin();
    }

    /// End timing for a named operation
    pub fn end(&mut self, name: &str) {
        if !self.enabled {
            return;
        }
        if let Some(counter) = self.counters.get_mut(name) {
            counter.end();
        }
    }

    /// Update all counters (call once per frame)
    pub fn update(&mut self) {
        if !self.enabled {
            return;
        }
        for counter in self.counters.values_mut() {
            counter.update();
        }
    }

    /// Get average time for a counter in milliseconds
    pub fn get_avg_ms(&self, name: &str) -> Option<f32> {
        self.counters.get(name).map(|c| c.get_avg_ms())
    }

    /// Get last time for a counter in milliseconds
    pub fn get_last_ms(&self, name: &str) -> Option<f32> {
        self.counters.get(name).map(|c| c.get_last_ms())
    }

    /// Get all counter names and their average times (sorted by name)
    pub fn get_all_counters(&self) -> Vec<(String, f32, f32)> {
        let mut counters: Vec<_> = self
            .counters
            .iter()
            .map(|(name, counter)| {
                (name.clone(), counter.get_last_ms(), counter.get_avg_ms())
            })
            .collect();
        counters.sort_by(|a, b| a.0.cmp(&b.0));
        counters
    }

    /// Enable/disable performance monitoring
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Clear all counters
    pub fn clear(&mut self) {
        self.counters.clear();
    }

    /// Get total render time by summing all counters
    pub fn get_total_time_ms(&self) -> f32 {
        self.counters.values().map(|c| c.get_last_ms()).sum()
    }
}
