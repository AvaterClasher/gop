use regex::Regex;
use std::process::{exit, Command};

#[cfg(target_os = "linux")]
fn get_command_output(command: &str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");
    return String::from(
        String::from_utf8_lossy(&output.stdout)
            .trim_end()
            .trim_start(),
    );
}

#[cfg(target_os = "linux")]
fn run(browser: &str, url: &str) {
    Command::new("sh")
        .arg("-c")
        .arg(format!("{} {}", browser, url))
        .spawn()
        .expect("Failed to open browser");
}

fn is_inside_working_tree() -> bool {
    get_command_output("git rev-parse --is-inside-work-tree") == "true"
}

fn get_remote_url() -> String {
    get_command_output("git config --get remote.origin.url")
}

fn main() {
    // Check that the user is in a git repository.
    if !is_inside_working_tree() {
        eprintln!("This is not a git directory");
        exit(1);
    }

    let remote = get_remote_url();

    // Updated regex to handle both SSH and HTTPS URLs
    let re = Regex::new(r"(?:git@|https://)([^:/]+)[:/](.+)\.git").unwrap();
    let caps = match re.captures(remote.as_str()) {
        Some(caps) => caps,
        None => {
            eprintln!("Failed to parse the remote URL: {}", remote);
            exit(1);
        }
    };

    let domain = caps.get(1).map_or("github.com", |m| m.as_str());
    let repository = caps.get(2).map_or("", |m| m.as_str());

    let url = format!("https://{}/{}", domain, repository);

    run("opera", url.as_str());
}

