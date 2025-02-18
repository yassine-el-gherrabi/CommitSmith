use std::process::Command;

pub fn run(diff_file: Option<String>) {
    let diff = match diff_file {
        Some(path) => std::fs::read_to_string(path).unwrap_or_else(|_| "".to_string()),
        None => {
            let output = Command::new("git")
                .args(["diff", "origin/main"])
                .output()
                .expect("Failed to run git diff origin/main..HEAD");
            String::from_utf8_lossy(&output.stdout).to_string()
        }
    };

    println!("PR description would be generated from the following diff:\n{}", diff);
}
