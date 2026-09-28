use std::process::Command;
use gpui::*;

#[derive(Clone)]
pub struct RunnerOutput {
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
    pub running: bool,
}

pub fn run_code(code: &str, cx: &mut Context<impl Render>) -> Entity<RunnerOutput> {
    let temp_dir = std::env::temp_dir().join(format!("borrow_run_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&temp_dir);
    std::fs::create_dir_all(&temp_dir).unwrap();

    let cargo_toml = r#"[package]
name = "borrow_run"
version = "0.1.0"
edition = "2021"
"#;
    std::fs::write(temp_dir.join("Cargo.toml"), cargo_toml).unwrap();
    std::fs::create_dir_all(temp_dir.join("src")).unwrap();
    std::fs::write(temp_dir.join("src").join("main.rs"), code).unwrap();

    let entity = cx.new(|_| RunnerOutput {
        stdout: String::new(),
        stderr: String::new(),
        success: false,
        running: true,
    });

    let entity_weak = entity.downgrade();
    let task = cx.background_executor().spawn(async move {
        let result = Command::new("cargo")
            .arg("run")
            .current_dir(&temp_dir)
            .output();

        match result {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                let success = out.status.success();
                (stdout, stderr, success)
            }
            Err(e) => (String::new(), format!("Failed to run: {}", e), false),
        }
    });

    cx.spawn(async move |_, cx| {
        let (stdout, stderr, success) = task.await;
        if let Some(entity) = entity_weak.upgrade() {
            entity.update(cx, |output, cx| {
                output.stdout = stdout;
                output.stderr = stderr;
                output.success = success;
                output.running = false;
                cx.notify();
            });
        }
    }).detach();

    entity
}
