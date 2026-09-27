use gitgat::process::ProcessRunner;
use gitgat::store::Catalog;
use gitgat::ui::GitGatApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().any(|arg| arg == "--self-check") {
        println!("GitGat {}", env!("CARGO_PKG_VERSION"));
        let runner = ProcessRunner;
        let git = runner.run("git", ["--version"], None)?;
        println!("{}", git.trim());
        return Ok(());
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
