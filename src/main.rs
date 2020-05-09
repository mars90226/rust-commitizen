extern crate console;
extern crate dialoguer;

use std::fmt;
use std::process::Command;

use console::Style;
use dialoguer::{Input, Select};
use dialoguer::theme::ColorfulTheme;

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
        .interact()
        .unwrap();

    let commit_message: String = Input::with_theme(&input_theme)
        .with_prompt("Write a short commit message")
        .interact()
        .unwrap();

    let final_commit_message = format!("{}({}): {}", conventional_commits[selection.unwrap()], scope, commit_message.to_lowercase());

    Command::new("git")
        .args(&["commit", "-m", &final_commit_message])
        .status()
        .expect("Failed to git commit");
}
