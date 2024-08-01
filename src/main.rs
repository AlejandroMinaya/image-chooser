use eframe::{
    egui::{
        ViewportBuilder,
        Context,
        CentralPanel,
    },
    Result,
    run_native,
    NativeOptions,
    Frame
};

#[derive(Default)]
struct ImageChooser;
impl eframe::App for ImageChooser {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("klk");
        });
    }
}

fn main() -> Result {
    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1440.0, 810.0]),
        ..Default::default()
    };

    run_native("", options, Box::new(|cc| {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Ok(Box::<ImageChooser>::default())
    }))
}
