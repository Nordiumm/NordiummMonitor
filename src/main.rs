use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::ruin_native(
        "Nordiumm Monitor",
        options,
        Box::new(|_cc| Ok(Box::new(NordiummMonitor::default()))),
    )
}