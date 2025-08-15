use std::process::Command;

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

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Most recent MSVC Tools is at -> {stdout}");
}
