use std::{env, process::Command};

// TODO(DavidC) not pretty yet, but works.
// - [ ] Separate out into separate methods, main calls at same abstraction level.
// - [ ] User Result error for propagation.
// - [ ] Document.
fn main() {
    println!("Finding most recent MSVC Tools...");
    let vswhere_path = "C:\\Program Files (x86)\\Microsoft Visual Studio\\Installer\\vswhere.exe";
    let output = Command::new(vswhere_path)
        .args(&[
            "-latest",
            "-products",
            "*",
            "-requires",
            "Microsoft.VisualStudio.Component.VC.Tools.x86.x64",
            "-property",
            "installationPath",
        ])
        .output()
        .expect("Failed to run vswhere.exe");

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Something went wrong.");
        println!("Output -> {stdout}");

        if !output.stderr.is_empty() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Error -> {stderr}");
        }

        return;
    }

    let tools_path = String::from_utf8_lossy(&output.stdout).into_owned();
    let tools_path = tools_path.trim();
    println!("Most recent MSVC Tools is at -> {tools_path}");

    let vcvarsall_path = format!("{tools_path}\\VC\\Auxiliary\\Build\\vcvarsall.bat");
    println!("Running \"{vcvarsall_path}\"");
    let env_output = Command::new("cmd")
        .args(&["/C", &vcvarsall_path, "x64", "&&", "set"])
        .output()
        .expect("Failed to run vcvarsall.bat");

    if !env_output.status.success() {
        println!("Something went wrong running vcvarsall.bat");
        println!("Tried to run {vcvarsall_path}");
        let stderr = String::from_utf8_lossy(&env_output.stderr);
        println!("Error -> {stderr}");
        return;
    }

    // Parse the environment variables from the output
    let env_output_str = String::from_utf8_lossy(&env_output.stdout);
    let mut env_vars = std::collections::HashMap::new();

    for line in env_output_str.lines() {
        if let Some((key, value)) = line.split_once('=') {
            env_vars.insert(key.to_string(), value.to_string());
        }
    }

    let args: Vec<String> = env::args().skip(1).collect();

    println!("Spawning \"neovide.exe\"");

    Command::new("neovide.exe")
        .current_dir(env::current_dir().unwrap())
        .envs(&env_vars)
        .args(args)
        .spawn()
        .expect("Failed to run neovide.exe");
}
