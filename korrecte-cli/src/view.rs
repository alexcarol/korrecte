use colored::*;
use korrecte::reporting::Finding;
use korrecte::view::View;

pub struct Cli;

impl View for Cli {
    fn render(&self, findings: &[Finding]) {
        for finding in findings {
            println!(
                "{} on {} [{}]. Metadata: {:?}",
                finding.spec().name.bold(),
                finding.object_metadata().name.green(),
                finding
                    .object_metadata()
                    .namespace
                    .as_ref()
                    .unwrap_or(&"default".to_string())
                    .blue(),
                finding.lint_metadata(),
            )
        }
    }
}
