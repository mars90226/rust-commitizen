use std::fmt;
use std::process::Command;

use console::Style;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};

#[allow(dead_code)]
#[derive(Clone)]
struct ConventionalCommitType {
    class: &'static str,
    title: &'static str,
    description: &'static str,
}

impl ConventionalCommitType {
    fn conventional_commits() -> Vec<ConventionalCommitType> {
        return vec![
            ConventionalCommitType {
                class: "feat",
                title: "Features",
                description: "A new feature",
            },
            ConventionalCommitType {
                class: "fix",
                title: "Bug Fixes",
                description: "A bug fix",
            },
            ConventionalCommitType {
                class: "docs",
                title: "Documentation",
                description: "Documentation only changes",
            },
            ConventionalCommitType {
                class: "style",
                title: "Styles",
                description: "Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)",
            },
            ConventionalCommitType {
                class: "refactor",
                title: "Code Refactoring",
                description: "A code change that neither fixes a bug nor adds a feature",
            },
            ConventionalCommitType {
                class: "perf",
                title: "Performance Improvements",
                description: "A code change that improves performance",
            },
            ConventionalCommitType {
                class: "test",
                title: "Tests",
                description: "Adding missing tests or correcting existing tests",
            },
            ConventionalCommitType {
                class: "build",
                title: "Builds",
                description: "Changes that affect the build system or external dependencies (example scopes: gulp, broccoli, npm)",
            },
            ConventionalCommitType {
                class: "ci",
                title: "Continuous Integrations",
                description: "Changes to our CI configuration files and scripts (example scopes: Travis, Circle, BrowserStack, SauceLabs)",
            },
            ConventionalCommitType {
                class: "chore",
                title: "Chores",
                description: "Other changes that don't modify src or test files",
            },
            ConventionalCommitType {
                class: "revert",
                title: "Reverts",
                description: "Reverts a previous commit",
            }
        ];
    }
}

impl fmt::Display for ConventionalCommitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:9} {}", format!("{}:", self.class), self.description)
    }
}

fn compose_commit_message(
    conventional_commit: &ConventionalCommitType,
    scope: &str,
    short_commit_message: &str,
    long_commit_message: &str,
) -> String {
    let mut commit_message = conventional_commit.class.to_string();

    if scope.is_empty() {
        commit_message.push_str(": ");
    } else {
        commit_message.push_str(&format!("({}): ", scope));
    }

    commit_message.push_str(short_commit_message);

    if !long_commit_message.is_empty() {
        commit_message.push_str(&format!("\n\n{}", long_commit_message));
    }

    commit_message
}

fn main() {
    let input_theme = ColorfulTheme {
        values_style: Style::new().green().dim(),
        ..ColorfulTheme::default()
    };

    let conventional_commits = ConventionalCommitType::conventional_commits();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select conventional commit type")
        .default(0)
        .items(&conventional_commits)
        .interact_opt()
        .unwrap();

    if selection.is_none() {
        return;
    }

    let scope: String = Input::with_theme(&input_theme)
        .with_prompt("What is the scope of this change")
        .allow_empty(true)
        .interact()
        .unwrap();

    let short_commit_message: String = Input::with_theme(&input_theme)
        .with_prompt("Write a short commit message")
        .interact()
        .unwrap();

    let long_commit_message: String = Input::with_theme(&input_theme)
        .with_prompt("Write a long commit message")
        .allow_empty(true)
        .interact()
        .unwrap();

    let commit_message = compose_commit_message(
        &conventional_commits[selection.unwrap()],
        &scope,
        &short_commit_message,
        &long_commit_message,
    );

    Command::new("git")
        .args(&["commit", "-m", &commit_message])
        .status()
        .expect("Failed to git commit");
}
