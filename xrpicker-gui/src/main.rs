// Copyright 2022-2023, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use image;

use itertools::Itertools;
use xrpicker::{
    make_platform, platform::PlatformRuntime, AppState, Error, PersistentAppState, Platform,
};

// const ICON_32: &[u8; 542] = include_bytes!("../../assets/icon/icon32.png");
const ICON_48: &[u8; 727] = include_bytes!("../../assets/icon/icon48.png");

fn load_icon(icon_data: &[u8]) -> Option<eframe::IconData> {
    let image = image::load_from_memory_with_format(icon_data, image::ImageFormat::Png).ok()?;
    let image = image.into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    Some(eframe::IconData {
        rgba,
        width,
        height,
    })
}

struct PickerApp<T: Platform> {
    platform: T,
    state: Option<Result<AppState<T>, Error>>,
    persistent_state: PersistentAppState,
}

impl<T: Platform> PickerApp<T> {
    fn new(platform: T, cc: &eframe::CreationContext<'_>) -> Self {
        let persistent_state = cc
            .storage
            .and_then(|storage| eframe::get_value::<PersistentAppState>(storage, eframe::APP_KEY))
            .unwrap_or_default();
        let state = Some(AppState::new_with_persistent_state(
            &platform,
            &persistent_state,
        ));

        PickerApp {
            platform,
            state,
            persistent_state,
        }
    }

    fn store_persistent_data(&self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.persistent_state);
    }
}

const PROJECT_URL: &str = "https://github.com/rpavlik/xr-picker";

fn add_about_contents(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label("This is an open-source software project, maintained at");
        ui.hyperlink(PROJECT_URL);
        ui.label(". You are welcome and encouraged to participate in development.");
    });
}

/// Trait implemented for all states of the GUI.
trait GuiView<T: Platform> {
    fn update(
        self,
        platform: &T,
        ctx: &egui::Context,
        persistent_state: &mut PersistentAppState,
    ) -> Result<AppState<T>, Error>;
}

impl<T: Platform> GuiView<T> for Error {
    fn update(
        self,
        platform: &T,
        ctx: &egui::Context,
        persistent_state: &mut PersistentAppState,
    ) -> Result<AppState<T>, Error> {
        egui::TopBottomPanel::bottom("about").show(ctx, add_about_contents);
        let repopulate = egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.heading(format!("ERROR! {:?}", self));
                if ui.button("Refresh").clicked() {
                    return true;
                }
                false
            })
            .inner;

        if repopulate {
            return AppState::new_with_persistent_state(platform, persistent_state);
        }
        Err(self)
    }
}

// These are GUI-specific impls on our AppState
trait EguiAppState<T: Platform> {
    /// Add the non-fatal errors from manifest parsing to the UI
    fn add_non_fatal_errors_listing(&self, ui: &mut egui::Ui);

    /// Adds a grid with the runtimes to the given `egui::Ui`, handling "make active" button presses.
    ///
    /// Returns an error (in which case that becomes the new state), or a boolean indicating whether to refresh.
    fn add_runtime_grid(&self, platform: &T, ui: &mut egui::Ui) -> Result<bool, Error>;
}

impl<T: Platform> EguiAppState<T> for AppState<T> {
    fn add_non_fatal_errors_listing(&self, ui: &mut egui::Ui) {
        if self.nonfatal_errors.is_empty() {
            return;
        }

        ui.label("Non-fatal errors from manifest loading:");
        ui.label(
            self.nonfatal_errors
                .iter()
                .map(|e| format!("- {} - {:?}\n", e.0.display(), e.1))
                .join("\n"),
        );
    }

    fn add_runtime_grid(&self, platform: &T, ui: &mut egui::Ui) -> Result<bool, Error> {
        // The closure this calls returns true if we should refresh the list
        egui::Grid::new("runtimes")
            .striped(true)
            .min_col_width(ui.spacing().interact_size.x * 2.0) // widen to avoid resizing based on default runtime
            .min_row_height(ui.spacing().interact_size.y * 2.5)
            .num_columns(4)
            .show(ui, |ui| -> Result<bool, Error> {
                let mut repopulate = false;
                ui.label(""); // for button
                ui.label("Runtime Name");
                ui.label("State");
                ui.label("Details");
                ui.end_row();

                for runtime in &self.runtimes {
                    let runtime_active_state =
                        platform.get_runtime_active_state(runtime, &self.active_data);
                    if runtime_active_state.should_provide_make_active_button() {
                        if ui.button("Make active").clicked() {
                            if let Err(e) = runtime.make_active() {
                                eprintln!("error in make_active: {:?}", e);
                                return Err(e);
                            }
                            repopulate = true;
                        }
                    } else {
                        ui.label("");
                    }
                    ui.label(runtime.get_runtime_name());
                    ui.label(format!("{}", runtime_active_state));
                    ui.label(runtime.describe());
                    ui.end_row();
                }
                Ok(repopulate)
            })
            .inner
    }
}

#[derive(Debug, PartialEq, Eq)]
enum HeaderAction {
    Nothing,
    ShouldRefresh,
    ShouldBrowse,
}

/// Creates a top panel with a header and a refresh button.
/// returns true if it should refresh
fn header_with_browse_and_refresh_button(ctx: &egui::Context) -> HeaderAction {
    egui::TopBottomPanel::top("header")
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("OpenXR Runtime Picker");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Refresh").clicked() {
                        return HeaderAction::ShouldRefresh;
                    }
                    if ui.button("Browse for manifest").clicked() {
                        return HeaderAction::ShouldBrowse;
                    }
                    HeaderAction::Nothing
                })
                .inner
            })
            .inner
        })
        .inner
}

impl<T: Platform> GuiView<T> for AppState<T> {
    fn update(
        self,
        platform: &T,
        ctx: &egui::Context,
        persistent_state: &mut PersistentAppState,
    ) -> Result<AppState<T>, Error> {
        egui::TopBottomPanel::bottom("about").show(ctx, add_about_contents);

        if !self.nonfatal_errors.is_empty() {
            egui::TopBottomPanel::bottom("non_fatal_errors")
                .show(ctx, |ui| self.add_non_fatal_errors_listing(ui));
        }

        let header_action = header_with_browse_and_refresh_button(ctx);

        let mut new_extra_paths = vec![];

        // handle browse button
        if header_action == HeaderAction::ShouldBrowse {
            if let Some(p) = rfd::FileDialog::new().pick_file() {
                println!("Got a new path from file dialog: {}", p.display());
                new_extra_paths.push(p);
            }
        }

        // handle drag and drop
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                for file in &i.raw.dropped_files {
                    if let Some(p) = &file.path {
                        println!("Got a new path from drag and drop: {}", p.display());
                        new_extra_paths.push(p.clone());
                    }
                }
            }
        });

        let should_refresh: bool =
            header_action == HeaderAction::ShouldRefresh || !new_extra_paths.is_empty();

        // Central panel must come last
        let should_refresh = should_refresh
            || egui::CentralPanel::default()
                .show(ctx, |ui| self.add_runtime_grid(platform, ui))
                .inner?; // get at the nested closure's return value (whether to repopulate), after handling errors.
        persistent_state.append_new_extra_paths(new_extra_paths);
        if should_refresh {
            return self.refresh(platform, Some(persistent_state));
        }
        Ok(self)
    }
}

impl<T: Platform> GuiView<T> for Result<AppState<T>, Error> {
    fn update(
        self,
        platform: &T,
        ctx: &egui::Context,
        persistent_state: &mut PersistentAppState,
    ) -> Result<AppState<T>, Error> {
        match self {
            Ok(state) => state.update(platform, ctx, persistent_state),
            Err(e) => e.update(platform, ctx, persistent_state),
        }
    }
}

impl<T: Platform> eframe::App for PickerApp<T> {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let Some(state_or_error) = self.state.take() {
            let new_state = state_or_error.update(&self.platform, ctx, &mut self.persistent_state);
            self.state.replace(new_state);
        } else {
            // unlikely/impossible to get here, but let's clean up nicely if we do.
            frame.close()
        }
    }

    // Do not save window size/position, it can get messed up.
    fn persist_egui_memory(&self) -> bool {
        false
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        self.store_persistent_data(storage)
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        icon_data: load_icon(ICON_48),
        // icon_data: load_icon(ICON_32),
        ..Default::default()
    };
    eframe::run_native(
        "OpenXR Runtime Picker",
        options,
        Box::new(|cc| Box::new(PickerApp::new(make_platform(), cc))),
    )
}
