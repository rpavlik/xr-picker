// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use itertools::Itertools;
use xrpicker::{make_platform, platform::PlatformRuntime, Error, ManifestError, Platform};

struct InnerState<T: Platform> {
    runtimes: Vec<T::PlatformRuntimeType>,
    nonfatal_errors: Vec<ManifestError>,
    active_data: T::PlatformActiveData,
}

impl<T: Platform> InnerState<T> {
    fn new(platform: &T) -> Result<Self, Error> {
        let (runtimes, nonfatal_errors) = platform.find_available_runtimes()?;
        let active_data = platform.get_active_data();
        Ok(Self {
            runtimes,
            nonfatal_errors,
            active_data,
        })
    }

    /// "refresh" existing state: we don't re-create if we can avoid it,
    /// to preserve the order of existing entries.
    fn refresh(self, platform: &T) -> Result<Self, Error> {
        let (new_runtimes, new_nonfatal_errors) = platform.find_available_runtimes()?;
        let active_data = platform.get_active_data();

        // start with existing runtimes
        let runtimes = self
            .runtimes
            .into_iter()
            // chain on the new ones
            .chain(new_runtimes.into_iter())
            // only keep the unique ones, preferring the earlier ones
            .unique_by(|r| {
                // compare by the list of manifests used
                r.get_manifests()
                    .into_iter()
                    .map(|p| p.to_owned())
                    .collect::<Vec<_>>()
            })
            .collect();
        Ok(Self {
            runtimes,
            nonfatal_errors: new_nonfatal_errors,
            active_data,
        })
    }
}

struct PickerApp<T: Platform> {
    platform: T,
    state: Option<Result<InnerState<T>, Error>>,
}

impl<T: Platform> PickerApp<T> {
    fn new(platform: T) -> Self {
        let state = Some(InnerState::new(&platform));

        PickerApp { platform, state }
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
    fn update(self, platform: &T, ctx: &egui::Context) -> Result<InnerState<T>, Error>;
}

impl<T: Platform> GuiView<T> for Error {
    fn update(self, platform: &T, ctx: &egui::Context) -> Result<InnerState<T>, Error> {
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
            return InnerState::new(platform);
        }
        Err(self)
    }
}

// These are GUI-specific impls
impl<T: Platform> InnerState<T> {
    /// Add the non-fatal errors from manifest parsing to the UI
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

    /// Adds a grid with the runtimes to the given `egui::Ui`, handling "make active" button presses.
    ///
    /// Returns an error (in which case that becomes the new state), or a boolean indicating whether to refresh.
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

/// Creates a top panel with a header and a refresh button.
/// returns true if it should refresh
fn header_with_refresh_button(ctx: &egui::Context) -> bool {
    egui::TopBottomPanel::top("header")
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("OpenXR Runtime Picker");
                ui.centered_and_justified(|ui| ui.button("Refresh").clicked())
                    .inner
            })
            .inner
        })
        .inner
}

impl<T: Platform> GuiView<T> for InnerState<T> {
    fn update(self, platform: &T, ctx: &egui::Context) -> Result<InnerState<T>, Error> {
        egui::TopBottomPanel::bottom("about").show(ctx, add_about_contents);

        if !self.nonfatal_errors.is_empty() {
            egui::TopBottomPanel::bottom("non_fatal_errors")
                .show(ctx, |ui| self.add_non_fatal_errors_listing(ui));
        }

        let should_refresh: bool = header_with_refresh_button(ctx);

        // Central panel must come last
        let should_refresh = should_refresh
            || egui::CentralPanel::default()
                .show(ctx, |ui| self.add_runtime_grid(platform, ui))
                .inner?; // get at the nested closure's return value (whether to repopulate), after handling errors.
        if should_refresh {
            return self.refresh(platform);
        }
        Ok(self)
    }
}

impl<T: Platform> GuiView<T> for Result<InnerState<T>, Error> {
    fn update(self, platform: &T, ctx: &egui::Context) -> Result<InnerState<T>, Error> {
        match self {
            Ok(state) => state.update(platform, ctx),
            Err(e) => e.update(platform, ctx),
        }
    }
}

impl<T: Platform> eframe::App for PickerApp<T> {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let Some(state_or_error) = self.state.take() {
            let new_state = state_or_error.update(&self.platform, ctx);
            self.state.replace(new_state);
        } else {
            // unlikely/impossible to get here, but let's clean up nicely if we do.
            frame.close()
        }
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "OpenXR Runtime Picker",
        options,
        Box::new(|_cc| Box::new(PickerApp::new(make_platform()))),
    );
}
