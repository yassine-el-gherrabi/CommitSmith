use std::process::Command;

pub fn run(diff_file: Option<String>) {
    let diff = match diff_file {
        Some(path) => std::fs::read_to_string(path).unwrap_or_else(|_| "".to_string()),
        None => {
            let output = Command::new("git")
                .args(["diff", "--staged"])
                .output()
                .expect("Failed to run git diff --staged");
            String::from_utf8_lossy(&output.stdout).to_string()
        }
    };

    println!("Commit message would be generated from the following diff:\n{}", diff);
}
