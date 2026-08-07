use std::{fs, path::PathBuf, process::Command};

#[cfg(target_os = "linux")]
pub fn get_current_song() -> String {
    use std::process::Command;

    let output = Command::new("playerctl")
        .args(["metadata", "--format", "{{title}} - {{artist}}"])
        .output();

    match output {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).trim().to_string(),
        _ => "No Music Playing".to_string(),
    }
}

#[cfg(not(target_os = "linux"))]
pub fn get_current_song() -> String {
    "Music PLayer only on Linux for now :( Sorry for that".to_string()
}

fn get_fastfetch_logo() -> String {
    // NO_COLOR=1 fastfetch --logo-type small -s none
    let output = Command::new("fastfetch")
        .env("NO_COLOR", "1")
        .args(["--logo-type", "small", "--pipe", "-s", "none"])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let raw_stdout = String::from_utf8_lossy(&out.stdout);
            raw_stdout.trim_matches('\n').to_string()
        }
        _ => "   _ _\n  (o.o)\n   > <\n /  |  \\".to_string(),
    }
}

pub fn get_cached_logo(os_name: &str) -> String {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("startui");

    let safe_name = os_name.to_lowercase().replace(" ", "_");
    let cached_path = config_dir.join(format!("{}.txt", safe_name));

    if cached_path.exists()
        && let Ok(cached_logo) = fs::read_to_string(&cached_path)
    {
        return cached_logo;
    }

    let logo = get_fastfetch_logo();

    let _ = fs::create_dir_all(&config_dir);
    let _ = fs::write(&cached_path, &logo);
    logo
}
