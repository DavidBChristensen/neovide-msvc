use std::{
    collections::HashMap,
    env,
    io::{self},
    process::Command,
};

/// The main entry point of the program.
///
/// This function locates the most recent MSVC Tools installation, sets up the build environment
/// by running vcvarsall.bat (depending on the architecture), parses the resulting environment
/// variables, collects command-line arguments, and spawns neovide.exe with the inherited
/// environment and arguments.
///
/// # Errors
///
/// Returns an io::Error if any of the following operations fail:
/// - Executing vswhere.exe to find the MSVC tools path.
/// - Running vcvarsall.bat to set up the environment.
/// - Spawning the neovide.exe process.
fn main() -> io::Result<()> {
    println!("Running Neovide with MSVC Developer Tools...");

    let tools_path = find_msvc_tools_path()?;
    println!("Most recent MSVC Tools is at -> {tools_path}");

    let env_vars = get_vcvarsall_env(&tools_path)?;
    println!("Fetched MSVC environment variables.");

    let args: Vec<String> = env::args().skip(1).collect();
    println!("Spawning \"neovide.exe\" with passed-in args.");
    spawn_neovide(&env_vars, &args)?;

    Ok(())
}

/// Finds the installation path of the most recent MSVC Tools using vswhere.exe.
///
/// This function executes vswhere.exe with specific arguments to locate the latest
/// Visual Studio installation that includes the VC Tools component. It trims the
/// output and returns the path as a String.
///
/// # Errors
///
/// Returns an `io::Error` if vswhere.exe fails to execute, exits with a non-zero status,
/// or if the output cannot be parsed as UTF-8.
fn find_msvc_tools_path() -> io::Result<String> {
    let program_files_x86 = env::var("ProgramFiles(x86)").unwrap_or_else(|_| {
        env::var("ProgramFiles").unwrap_or_else(|_| "C:\\Program Files".to_string())
    });

    let vswhere_path =
        format!("{program_files_x86}\\Microsoft Visual Studio\\Installer\\vswhere.exe");
    let output = Command::new(&vswhere_path)
        .args([
            "-latest",
            "-products",
            "*",
            "-requires",
            "Microsoft.VisualStudio.Component.VC.Tools.x86.x64",
            "-property",
            "installationPath",
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);

        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("vswhere.exe failed: {stderr}"),
        ));
    }

    let tools_path = String::from_utf8(output.stdout)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
        .trim()
        .to_string();

    Ok(tools_path)
}

/// Retrieves the environment variables set by running vcvarsall.bat for x64.
///
/// This function constructs the path to vcvarsall.bat based on the provided MSVC tools path,
/// executes it via cmd.exe to set up the environment for x64, and captures the output of 'set'
/// to parse all environment variables into a HashMap.
///
/// # Arguments
///
/// * `tools_path` - The path to the MSVC tools installation.
///
/// # Errors
///
/// Returns an `io::Error` if cmd.exe fails to execute, vcvarsall.bat exits with a non-zero status,
/// or if the output cannot be parsed as UTF-8.
fn get_vcvarsall_env(tools_path: &str) -> io::Result<HashMap<String, String>> {
    let vcvarsall_path = format!("{tools_path}\\VC\\Auxiliary\\Build\\vcvarsall.bat");

    println!("Running \"{vcvarsall_path}\"");
    let output = Command::new("cmd")
        .args(["/C", &vcvarsall_path, "x64", "&&", "set"])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("vcvarsall.bat failed: {stderr}"),
        ));
    }

    let env_output_str = String::from_utf8(output.stdout)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let mut env_vars = HashMap::new();
    for line in env_output_str.lines() {
        if let Some((key, value)) = line.split_once('=') {
            env_vars.insert(key.to_string(), value.to_string());
        }
    }

    Ok(env_vars)
}

/// Spawns neovide.exe with the provided environment variables and arguments.
///
/// This function launches neovide.exe, passing along the given environment variables
/// and command-line arguments. It inherits the current working directory.
///
/// # Arguments
///
/// * `env_vars` - A HashMap of environment variables to set for the process.
/// * `args` - A slice of command-line arguments to pass to neovide.exe.
///
/// # Errors
///
/// Returns an `io::Error` if neovide.exe fails to spawn.
fn spawn_neovide(env_vars: &HashMap<String, String>, args: &[String]) -> io::Result<()> {
    Command::new("neovide.exe")
        .envs(env_vars)
        .args(args)
        .spawn()?;
    Ok(())
}
