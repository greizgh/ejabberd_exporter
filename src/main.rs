extern crate actix_web;
extern crate clap;
extern crate env_logger;
extern crate failure;
#[macro_use]
extern crate log;
extern crate rayon;

mod metrics;
mod scraper;

use actix_web::{server, App, HttpRequest};
use failure::Error;
use scraper::get_metrics;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    env_logger::init();
    let matches = clap::App::new("ejabberd-exporter")
        .version(VERSION.unwrap_or("unknown"))
        .about("Prometheus exporter for ejabberd")
        .arg(
            clap::Arg::with_name("PORT")
                .short("p")
                .long("port")
                .help("Listening port for embedded http server")
                .takes_value(true)
                .default_value("9334"),
        )
        .arg(
            clap::Arg::with_name("ADDRESS")
                .short("a")
                .long("address")
                .help("Address to bind the webserver to")
                .takes_value(true)
                .default_value("0.0.0.0"),
        )
        .get_matches();
    match run(
        matches.value_of("ADDRESS").unwrap(),
        matches.value_of("PORT").unwrap(),
    ) {
        Ok(_) => {}
        Err(e) => error!("{}", e),
    }
}

fn metrics(_req: HttpRequest) -> Result<String, Error> {
    let mut output = String::new();

    let metrics = get_metrics().map_err(|error| {
        error!("{}", error);
        error
    })?;

    for metric in metrics {
        output += &format!("{}\n", metric);
    }

    Ok(output)
}

fn run(address: &str, port: &str) -> Result<(), Error> {
    server::new(|| App::new().resource("/metrics", |r| r.f(metrics)))
        .bind(format!("{}:{}", address, port))?
        .run();
    Ok(())
}
