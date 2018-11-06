use std::fmt;

#[derive(Debug)]
pub enum MetricType {
    Gauge,
    Counter,
}

#[derive(Debug)]
pub struct Metric<'a> {
    pub arguments: Vec<&'a str>,
    pub name: &'a str,
    pub help: &'a str,
    pub value: Option<i64>,
    pub metric_type: MetricType,
}

impl<'a> Metric<'a> {
    pub fn new(
        arguments: Vec<&'a str>,
        name: &'a str,
        help: &'a str,
        metric_type: MetricType,
    ) -> Metric<'a> {
        Metric {
            arguments,
            name,
            help,
            metric_type,
            value: None,
        }
    }
}

impl<'a> fmt::Display for Metric<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            Some(value) => write!(
                f,
                "# HELP {} {}\n# TYPE {} {}\n{} {}",
                self.name, self.help, self.name, self.metric_type, self.name, value
            ),
            None => write!(f, ""),
        }
    }
}

impl fmt::Display for MetricType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let metric_type = match *self {
            MetricType::Counter => "counter",
            MetricType::Gauge => "gauge",
        };
        write!(f, "{}", metric_type)
    }
}
