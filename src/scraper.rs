use std::process::Command;
use std::str;
use failure::Error;
use rayon::prelude::*;

use metrics::{Metric, MetricType};

pub fn get_metrics<'a>() -> Result<Vec<Metric<'a>>, Error> {
    let mut metrics: Vec<Metric> = vec![
        Metric::new(
            vec!["connected_users_number"],
            "ejabberd_connected_users",
            "Number of connected users",
            MetricType::Gauge,
        ),
        Metric::new(
            vec!["incoming_s2s_number"],
            "ejabberd_incoming_s2s_connections",
            "Number of s2s incoming connections",
            MetricType::Gauge,
        ),
        Metric::new(
            vec!["outgoing_s2s_number"],
            "ejabberd_outgoing_s2s_connections",
            "Number of s2s outgoing connections",
            MetricType::Gauge,
        ),
        Metric::new(
            vec!["mnesia", "transaction_commits"],
            "ejabberd_mnesia_transaction_commits",
            "Number of mnesia transaction commits",
            MetricType::Counter,
        ),
        Metric::new(
            vec!["mnesia", "transaction_failures"],
            "ejabberd_mnesia_transaction_failures",
            "Number of mnesia transaction failures",
            MetricType::Counter,
        ),
        Metric::new(
            vec!["mnesia", "transaction_log_writes"],
            "ejabberd_mnesia_transaction_log_writes",
            "Number of mnesia transaction log writes",
            MetricType::Counter,
        ),
        Metric::new(
            vec!["stats", "registeredusers"],
            "ejabberd_users_registered",
            "Number of registered users",
            MetricType::Gauge,
        ),
        Metric::new(
            vec!["stats", "onlineusers"],
            "ejabberd_users_online",
            "Number of online users",
            MetricType::Gauge,
        ),
        Metric::new(
            vec!["stats", "onlineusersnode"],
            "ejabberd_users_online_node",
            "Number of online users served by this node",
            MetricType::Gauge,
        ),
        Metric::new(
            vec!["stats", "uptimeseconds"],
            "ejabberd_uptime",
            "Node uptime in seconds",
            MetricType::Gauge,
        ),
        Metric::new(
            vec!["stats", "processes"],
            "ejabberd_processes",
            "Number of erlang processes",
            MetricType::Gauge,
        ),
    ];

    metrics
        .par_iter_mut()
        .for_each(|m| m.value = get_metric(&m.arguments).ok());

    Ok(metrics)
}

fn get_metric(args: &[&str]) -> Result<i64, Error> {
    let output = Command::new("ejabberdctl").args(args).output()?;
    let value: i64 = str::from_utf8(&output.stdout)?.trim().parse()?;

    Ok(value)
}
