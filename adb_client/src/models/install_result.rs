use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INSTALL_RESULT_REGEX: Regex =
        Regex::new(r"^(?P<status>Success|Failure|Error)?:?\s*(?P<message>.*)$")
            .expect("cannot build install result regex");
}

/// Represents the result of a pm install command
#[derive(Debug)]
pub enum InstallResult {
    /// Installation succeeded
    Success,
    /// Package manager returned an error
    Error(String),
}

impl From<String> for InstallResult {
    fn from(output: String) -> Self {
        let Some(groups) = INSTALL_RESULT_REGEX.captures(&output) else {
            return InstallResult::Error(output);
        };

        let status = groups.name("status").map(|m| m.as_str());
        let message = groups
            .name("message")
            .map(|m| m.as_str())
            .unwrap_or("Unknown");

        match status {
            Some("Success") => InstallResult::Success,
            Some(_) => InstallResult::Error(message.to_string()),
            None => InstallResult::Error(message.to_string()),
        }
    }
}
