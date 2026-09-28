use gitgat::policy::LockPolicyChecker;
use gitgat::process::ProcessRunner;
use gitgat::store::Catalog;
use gitgat::ui::GitGatApp;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();

    if args.iter().any(|arg| arg == "--self-check") {
        println!("GitGat {}", env!("CARGO_PKG_VERSION"));
        let runner = ProcessRunner;
        let git = runner.run("git", ["--version"], None)?;
        println!("{}", git.trim());
        return Ok(());
    }

    if args.iter().any(|arg| arg == "--check-lock-policy") {
        return run_lock_policy(&args);
    }

    let catalog = Catalog::open_default()?;
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1440.0, 900.0])
            .with_min_inner_size([1000.0, 680.0]),
        ..Default::default()
    };
    eframe::run_native(
        "GitGat",
        options,
        Box::new(move |_cc| Ok(Box::new(GitGatApp::new(catalog.clone())))),
    )?;
    Ok(())
}

fn run_lock_policy(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let base = arg_value(args, "--base").ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "--check-lock-policy requires --base <ref>",
        )
    })?;
    let actor = arg_value(args, "--actor").ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "--check-lock-policy requires --actor <identity>",
        )
    })?;
    let repo = arg_value(args, "--repo")
        .map(PathBuf::from)
        .unwrap_or(std::env::current_dir()?);

    let report = LockPolicyChecker::default().evaluate(&repo, &base, &actor)?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    if !report.allows_merge() {
        std::process::exit(2);
    }
    Ok(())
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|arg| arg == name)
        .and_then(|index| args.get(index + 1))
        .cloned()
}
