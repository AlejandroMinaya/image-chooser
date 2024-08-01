use eframe::{
    egui::{
        ViewportBuilder,
        Context,
        CentralPanel,
        Image,
        TopBottomPanel,
        Key,
    },
    Result,
    run_native,
    NativeOptions,
    Frame
};
use std::fs;
use rand::{thread_rng, seq::SliceRandom};

const WINNER_URI: &str = "bytes://winner_image.jpg";
const OTHER_URI: &str = "bytes://other_image.jpg";

#[derive(Default)]
struct ImageChooser {
    images: Vec<fs::DirEntry>,
    winner: Option<fs::DirEntry>
}


fn load_image<'a>(entry: &'a fs::DirEntry, uri: &'a str) -> Image<'a> {
    let image_bytes = fs::read(entry.path())
        .expect("Couldn't open image");
     return Image::from_bytes(uri.to_owned(), image_bytes)
         .shrink_to_fit()
}


impl ImageChooser {
    fn load_image_list(&mut self, path: String) {
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

        // Shuffle the files
        self.images.shuffle(&mut thread_rng());

        self.winner = self.images.pop();
    }
}
impl eframe::App for ImageChooser {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let screen_rect = ctx.input(|i| i.screen_rect());
        // let height = screen_rect.max.y - screen_rect.min.y;
        let width = screen_rect.max.x - screen_rect.min.x;

        // Status Bar
        TopBottomPanel::bottom("Status Bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                /* ====== DIRECTORY PICKER ====== */
                if ui.button("Choose images...").clicked() {
                    let file_picker = rfd::FileDialog::new();
                    if let Some(path) = file_picker.pick_folder() {
                        self.load_image_list(path.display().to_string());
                    }
                }
                /* ============================== */
                /* ===== LOADED IMAGES INFO ===== */
                if self.images.len() > 0 {
                    ui.monospace(format!("{} Image(s)", self.images.len()));
                }
                /* ============================== */
            })
        });

        CentralPanel::default().show(ctx, |ui| {
            /* === IMAGE COMPARISON === */
            if let Some(winner) = &self.winner {
                let other_entry = self.images.last();
                let half_width = ((width-25.0)/2.0).floor();

                let winner_image = load_image(&winner, WINNER_URI).max_width(half_width);
                ui.horizontal_centered(
                    |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(&winner.path().display().to_string());
                            ui.add(winner_image);
                        });

                        if let Some(other_entry) = other_entry {
                            let other_image = load_image(&other_entry, OTHER_URI).max_width(half_width);
                            ui.vertical_centered(|ui| {
                                ui.label(&other_entry.path().display().to_string());
                                ui.add(other_image);
                            });
                        }
                    }
                );
            }
            /* ======================== */
            /* ===== KEYBOARD INPUTS ===== */
            if ctx.input(|i| i.key_released(Key::ArrowLeft)) {
                self.images.pop();
                ui.ctx().forget_image(OTHER_URI);
            } else if ctx.input(|i| i.key_released(Key::ArrowRight)) {
                self.winner = self.images.pop();
                ui.ctx().forget_image(WINNER_URI);
                ui.ctx().forget_image(OTHER_URI);
            }
            /* =========================== */
        });


    }
}

fn main() -> Result {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_clamp_size_to_monitor_size(true)
        ,..Default::default()
    };


    run_native("", options, Box::new(|cc| {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Ok(Box::<ImageChooser>::default())
    }))
}
