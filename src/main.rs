use eframe::{
    egui::{
        ViewportBuilder,
        Context,
        CentralPanel,
        SidePanel,
        Image,
        TopBottomPanel,
    },
    Result,
    run_native,
    NativeOptions,
    Frame
};
use std::fs;

#[derive(Default)]
struct ImageChooser {
    path: Option<String>,
    images: Vec<fs::DirEntry>,
    winner: Option<fs::DirEntry>
}

const WINDOW_HEIGHT: f32 = 1440.0;
const WINDOW_WIDTH: f32 = 810.0;


impl eframe::App for ImageChooser {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let file_picker = rfd::FileDialog::new();


        // SidePanel
        SidePanel::left("Main Menu").show(ctx, |ui| {
            if ui.button("Choose images...").clicked() {
                if let Some(path) = file_picker.pick_folder() {
                    self.path = Some(path.display().to_string());
                }
            }

            if let Some(path) = &self.path {
                let files = fs::read_dir(path).expect("Expected directory to work");
                self.images = files
                    .map(|file_entry| file_entry.expect("Expected file"))
                    .filter(|file_entry| {
                        match file_entry.path().extension() {
                            Some(extension) => extension == "jpg",
                            None => false
                        }
                    })
                    .collect();

                self.winner = self.images.pop();
            }

        });

        // Status Bar
        TopBottomPanel::bottom("Status Bar").show(ctx, |ui| {
            if self.images.len() > 0 {
                ui.monospace(format!("Images ({})", self.images.len()));
            }
        });

        // Central Panel
        CentralPanel::default().show(ctx, |ui| {
            if let Some(winner) = &self.winner {
                ui.horizontal(|ui| {
                    let winner_image_bytes = fs::read(winner.path()).expect("Couldn't open winner image");
                    let winner_image = Image::from_bytes("bytes://winner_image", winner_image_bytes);
                    ui.add_sized([WINDOW_WIDTH/3.0, WINDOW_HEIGHT/3.0], winner_image);

                    if self.images.len() > 0 {
                        let other_image_path = self.images.last().expect("Expected another image").path();
                        let other_image_bytes = fs::read(other_image_path).expect("Couldn't open other image");
                        let other_image = Image::from_bytes("bytes://other_image", other_image_bytes);
                        ui.add_sized([WINDOW_WIDTH/3.0, WINDOW_HEIGHT/3.0], other_image);
                    }
                });
            }
        });
    }
}

fn main() -> Result {
    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
        ..Default::default()
    };


    run_native("", options, Box::new(|cc| {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Ok(Box::<ImageChooser>::default())
    }))
}
