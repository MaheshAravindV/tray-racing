use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum MetricKey {
    RayCountMaxD,
    RayCountSky,
    RayDepth,
    ReflectedRayColorR,
    ReflectedRayColorG,
    ReflectedRayColorB
}

#[derive(Default)]
struct Metric {
    sum: f64,
    count: u64,
}

#[derive(Default)]
pub struct Metrics {
    metrics: HashMap<MetricKey, Metric>,
}

impl Metrics {
    pub fn record(&mut self, key: MetricKey, value: f64) {
        let metric = self.metrics.entry(key).or_default();
        metric.sum += value;
        metric.count += 1;
    }

    pub fn average(&self, key: &MetricKey) -> Option<f64> {
        self.metrics.get(key).map(|m| m.sum / m.count as f64)
    }

    pub fn print(&self) {
        for (name, metric) in &self.metrics {
            println!(
                "{:?}: avg = {:.2} ({} samples)",
                name,
                metric.sum / metric.count as f64,
                metric.count
            );
        }
    }
}