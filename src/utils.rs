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
