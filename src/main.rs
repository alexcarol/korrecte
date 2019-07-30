mod linters;
mod config;
mod reporting;
mod view;
mod kube;

use crate::linters::LintCollection;
use toml;
use std::fs::File;
use std::io::prelude::*;
use crate::config::Config;
use crate::reporting::Reporter;
use crate::view::cli::Cli;
use crate::view::View;
use crate::linters::evaluator::OneShotEvaluator;
use ::kube::config as kube_config;
use crate::kube::objects::ObjectRepository;

fn main() {
    let cfg: Config = load_config().unwrap_or_default();
    let reporter = reporting::SingleThreadedReporter::default();

    let object_repository = ObjectRepository::new(kube_config::load_kube_config().unwrap()).unwrap();
    let list = LintCollection::all(cfg, reporter.clone(), object_repository.clone());
    OneShotEvaluator::evaluate(list, object_repository);

    let cli = Cli {};
    cli.render(&reporter.findings());
}

fn load_config() -> Result<Config, ConfigError> {
    let mut file = File::open("korrecte.toml")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(toml::from_str(&buffer)?)
}

enum ConfigError {
    Io(std::io::Error),
    Serde(toml::de::Error),
}

impl From<std::io::Error> for ConfigError {
    fn from(e: std::io::Error) -> Self {
        ConfigError::Io(e)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(e: toml::de::Error) -> Self {
        ConfigError::Serde(e)
    }
}