use eframe::{
    egui::{
        ViewportBuilder,
        Context,
        CentralPanel,
        Image,
        TopBottomPanel,
        Layout,
        Direction,
        Align,
        Key,
        Ui
    },
    Result,
    run_native,
    NativeOptions,
    Frame
};
use std::fs;
use rand::{thread_rng, seq::SliceRandom};

const WINNER_URI: &str = "bytes://winner_image";
const OTHER_URI: &str = "bytes://other_image";

#[derive(Default)]
struct ImageChooser {
    images: Vec<fs::DirEntry>,
    winner: Option<fs::DirEntry>
}


fn load_image<'a>(entry: &'a fs::DirEntry, uri: &'a str) -> Image<'a> {
    let image_bytes = fs::read(entry.path())
        .expect("Couldn't open image");
     return Image::from_bytes(uri.to_owned(), image_bytes)
}

fn render_image<'a>(ui: &'a mut Ui, entry: &'a fs::DirEntry, uri: &'a str, width: f32) {
    ui.with_layout(Layout {
        main_dir: Direction::TopDown,
        main_align: Align::Center,
        main_wrap: false,
        main_justify: false,
        cross_align: Align::Min,
        cross_justify: false,
    },|ui| {
        ui.set_width(width);
        ui.set_height(ui.available_height());

        let image = load_image(entry, uri)
            .maintain_aspect_ratio(true)
            .fit_to_exact_size([width, width].into());

        ui.add(image);
        ui.label(entry.path().display().to_string());

    });
}


impl ImageChooser {
    fn load_image_list(&mut self, path: String) {
        let files = fs::read_dir(path).expect("Expected directory to work");
        self.images = files
            .map(|file_entry| file_entry.expect("Expected file"))
            .filter(|file_entry| {
                match file_entry.path().extension() {
                    Some(extension) => extension == "jpg" || extension == "png",
                    None => false
                }
            })
            .collect();
        self.images.shuffle(&mut thread_rng());
        self.winner = self.images.pop();
    }
    fn trigger_file_explorer(&mut self) {
        let file_picker = rfd::FileDialog::new();
        if let Some(path) = file_picker.pick_folder() {
            self.load_image_list(path.display().to_string());
        }
    }
}
impl eframe::App for ImageChooser {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // STATUS BAR
        TopBottomPanel::bottom("Status Bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                /* ====== DIRECTORY PICKER ====== */
                if ui.button("Choose images...").clicked() {
                    self.trigger_file_explorer()
                }
                /* ============================== */
                /* ===== LOADED IMAGES INFO ===== */
                if !self.images.is_empty() {
                    ui.monospace(format!("{} Image(s)", self.images.len()));
                }
                /* ============================== */
            })
        });

        CentralPanel::default().show(ctx, |ui| {
            let width = ui.available_width();
            let half_width = ui.available_width() / 2.0;
            ui.with_layout(Layout {
                main_dir: Direction::LeftToRight,
                main_align: Align::Center,
                main_wrap: true,
                main_justify: false,
                cross_align: Align::Center,
                cross_justify: false
            },|mut ui| {
                ui.set_width(width);
            /* === IMAGE COMPARISON === */
            if let Some(winner) = &self.winner {

                let other_entry = self.images.last();
                // WINNER IMAGE
                render_image(&mut ui, winner, WINNER_URI, half_width);

                // OTHER IMAGE
                if let Some(other_entry) = other_entry {
                    render_image(&mut ui, other_entry, OTHER_URI, half_width);
                }
            }
            });
            /* ======================== */
            /* ===== KEYBOARD INPUTS ===== */
            if ctx.input(|i| i.key_released(Key::Enter)) && self.images.is_empty() {
                self.trigger_file_explorer();

            }
            if ctx.input(|i| i.key_released(Key::ArrowLeft) || i.key_released(Key::ArrowUp)) {
                self.images.pop();
                ui.ctx().forget_image(OTHER_URI);
            } else if ctx.input(|i| i.key_released(Key::ArrowRight) || i.key_released(Key::ArrowDown)) {
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
