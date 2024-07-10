pub use clap;
use clap::{command, Command, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum LogVerbosity {
    All,
    Debug,
    Error,
    Warning,
    Info,
    Trace,
    None,
}

pub fn build_command() -> Command {
    let mut result = command!();
    result = result.after_help("Made by Some Dinosaur@Zodiark [some.dinosaur@pm.me]");

    #[cfg(feature = "log_verbosity")]
    {
        use clap::{arg, value_parser};
        result = result.arg(
            arg!(
                -v --log_verbosity <LEVEL> "Specifies logs verbosity"
            )
            .value_parser(value_parser!(LogVerbosity))
            .default_value("warning")
        );
    }

    #[cfg(feature = "log_to_file")]
    {
        use std::path::PathBuf;
        use clap::{arg, value_parser};
        result = result.arg(
            arg!(
                --log_file <FILE> "Outputs logs to a file"
            )
            .value_parser(value_parser!(PathBuf))
        );
    }   

    #[cfg(feature = "log_to_loki")]
    {
        use url::Url;
        use clap::{arg, value_parser};
        result = result.arg(
            arg!(
                --log_loki <URL> "Outputs logs to a Loki instance"
            )
            .value_parser(value_parser!(Url))
        )
    }
    
    result
}

#[allow(unused_imports)]
pub fn configure(_matches: &clap::ArgMatches) {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    let registry = tracing_subscriber::registry();

    #[cfg(feature = "log_to_file")]
    let registry =
        if let Some(path) = _matches.get_one::<std::path::PathBuf>("log_file") {
            registry.with(Some(log_file_layer(path)))
        } else {
            registry.with(None)
        };
    
    #[cfg(feature = "log_to_loki")]
    let registry = 
        if let Some(loki_url) = _matches.get_one::<Url>("log_loki") {
            // TODO
        } else {

        };

    registry.init();
}

#[cfg(feature = "log_to_file")]
fn log_file_layer<S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>>(file_path: &std::path::PathBuf) -> impl tracing_subscriber::Layer<S> {
    let file = std::fs::File::create(file_path);
    if let Err(err) = &file {
        eprintln!("Could not open the file {:?}: {}", file_path, err);
    }
    let file = file.unwrap();

    tracing_subscriber::fmt::Layer::new()
        .with_ansi(false)
        .with_writer(file)
}