use clap::{command, ArgMatches, Command, ValueEnum};

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
    matches: ArgMatches,
}

impl GameArgs {
    pub fn new(about: &str) -> GameArgs {
        let command = command!().about(about.to_owned());
        Self::new_from_command(command)
    }

    pub fn new_from_command(mut command: Command) -> GameArgs {
        command = command.after_help("Made by Some Dinosaur@Zodiark\ne-mail: some.dinosaur@pm.me]");

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
            matches: command.get_matches(),
        }
    }

    pub fn get_matches(&self) -> &ArgMatches {
        &self.matches
    }

    pub fn process(&self) -> Result<(), String> {
        #[cfg(feature = "log_verbosity")]
        {
            use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
            let registry = tracing_subscriber::registry();

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

        Ok(())
    }

    #[cfg(feature = "log_to_file")]
    fn log_file_layer<
        S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    >(
        file_path: &std::path::PathBuf,
    ) -> Option<impl tracing_subscriber::Layer<S>> {
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
