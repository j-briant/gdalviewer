use egui::{Align, Widget};
use egui_file_dialog::FileDialog;
use gdal::Dataset;
use std::path::PathBuf;

#[derive(Default)]
pub struct ViewerPage(String);

impl ViewerPage {
    fn new(s: String) -> Self {
        Self(s)
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
    }

    fn show(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // HomePage is a simple page with centered text and file loader.
            ui.centered_and_justified(|inner| {
                inner.heading("Hey you did it!");
            })
        });
    }
}
pub struct HomePage {
    file_dialog: FileDialog,
}

impl HomePage {
    fn new() -> Self {
        Self {
            file_dialog: FileDialog::new(),
        }
    }

    fn show(&mut self, ctx: &egui::Context) -> Option<String> {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(Align::Center), |inner| {
                inner.heading("Select a source Dataset");
                if inner.button("Select file").clicked() {
                    self.file_dialog.select_file();
                }
                // Update the dialog and check if the user selected a file
                if let Some(path) = self.file_dialog.update(ctx).selected() {
                    inner.label(format!("Selected file: {:?}", path));
                    Some(String::from("Hey wassup"))
                } else {
                    None
                }
            })
        });
        None
    }
}

impl Default for HomePage {
    fn default() -> Self {
        HomePage {
            file_dialog: FileDialog::new(),
        }
    }
}

/* impl eframe::App for HomePage {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // HomePage is a simple page with centered text and file loader.
            ui.with_layout(egui::Layout::top_down_justified(Align::Center), |inner| {
                inner.heading("Select a source Dataset");
                if inner.button("Select file").clicked() {
                    self.0.select_file();
                }
                // Update the dialog and check if the user selected a file
                if let Some(path) = self.0.update(ctx).selected() {
                    inner.label(format!("Selected file: {:?}", path));
                    ViewerPage::new(ctx);
                }
            })
        });
    }
} */

#[derive(Default)]
pub struct ViewerApp {
    homepage: HomePage,
    viewer: ViewerPage,
}

impl ViewerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for ViewerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.viewer.0.len() > 0 {
            self.viewer.show(ctx)
        } else {
            match self.homepage.show(ctx) {
                Some(s) => {
                    self.viewer = ViewerPage::new(s);
                    self.viewer.show(ctx)
                }
                None => {
                    self.homepage.show(ctx);
                }
            }
        }

        /*
        ui.separator();

        ui.add(egui::github_link_file!(
            "https://github.com/emilk/eframe_template/blob/main/",
            "Source code."
        ));

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            powered_by_egui_and_eframe(ui);
            egui::warn_if_debug_build(ui);
        });
        */
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
