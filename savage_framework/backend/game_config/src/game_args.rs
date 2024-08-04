use clap::{arg, command, value_parser, ArgMatches, Command, ValueEnum};
use tracing_subscriber::EnvFilter;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum LogVerbosity {
    All,
    Debug,
    Error,
    Warning,
    Info,
    Trace,
    None,
}

pub struct GameArgs {
    name: String,
    matches: ArgMatches,
    level: tracing::Level,
    filter: String,
}

impl GameArgs {
    pub fn new(name: &str, about: &str, level: tracing::Level, filter: &str) -> GameArgs {
        let command = command!().about(about.to_owned());
        Self::new_from_command(name.to_owned(), command, level, filter)
    }

    pub fn new_from_command(name: String, mut command: Command, level: tracing::Level, filter: &str) -> GameArgs {
        command = command.arg(
            arg!(
                -i --id <ID> "Set id (easier to track game instance from logs)"
            )
            .value_parser(value_parser!(u64))
            .default_value("0")
        );
        command = command.after_help("Made by Some Dinosaur@Zodiark\ne-mail: some.dinosaur@pm.me");
        
        #[cfg(feature = "log_verbosity")]
        {
            use clap::{arg, value_parser};
            command = command.arg(
                arg!(
                    -v --log_verbosity <LEVEL> "Specifies logs verbosity"
                )
                .value_parser(value_parser!(LogVerbosity))
                .default_value("warning"),
            );
        }

        #[cfg(feature = "log_to_file")]
        {
            use clap::{arg, value_parser};
            use std::path::PathBuf;
            command = command.arg(
                arg!(
                    --log_file <FILE> "Outputs logs to a file"
                )
                .value_parser(value_parser!(PathBuf)),
            );
        }

        #[cfg(feature = "log_to_loki")]
        {
            use clap::{arg, value_parser};
            use url::Url;
            command = command.arg(
                arg!(
                    --log_loki <URL> "Outputs logs to a Loki instance"
                )
                .value_parser(value_parser!(Url)),
            )
        }

        GameArgs {
            name,
            matches: command.get_matches(),
            level,
            filter: filter.to_owned()
        }
    }

    pub fn get_matches(&self) -> &ArgMatches {
        &self.matches
    }

    pub fn process(&self) -> Result<(), String> {
        let default_filter = { format!("{},{}", self.level, self.filter) };
        let filter_layer = EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new(&default_filter))
            .unwrap();

        #[cfg(feature = "log_verbosity")]
        {
            use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
            let registry = tracing_subscriber::registry().with(filter_layer);

            #[cfg(feature = "log_to_file")]
            let registry =
                if let Some(path) = self.matches.get_one::<std::path::PathBuf>("log_file") {
                    registry.with(Self::log_file_layer(path))
                } else {
                    registry.with(None)
                };

            #[cfg(feature = "log_to_loki")]
            let registry = if let Some(loki_url) = self.matches.get_one::<Url>("log_loki") {
                // TODO
            } else {
            };

            registry.init();
        }
        if let Some(game_id) = self.matches.get_one::<u64>("id") {
            tracing::info!(game_name = self.name, game_id, "start");
        }

        Ok(())
    }

    #[cfg(feature = "log_to_file")]
    fn log_file_layer<
        S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    >(
        file_path: &std::path::PathBuf,
    ) -> Option<impl tracing_subscriber::Layer<S>> {
        if let Some(parent) = std::path::Path::new(file_path).parent() {
            if let Err(err) = std::fs::create_dir_all(parent) {
                eprintln!("Could not create directories {:?}: {}", parent, err);
                return None;
            }
        }

        let file = std::fs::File::create(file_path);
        if let Err(err) = &file {
            eprintln!("Could not open the file {:?}: {}", file_path, err);
            return None;
        }
        let file = file.unwrap();

        Some(
            tracing_subscriber::fmt::Layer::new()
                .with_ansi(false)
                .with_writer(file),
        )
    }
}
