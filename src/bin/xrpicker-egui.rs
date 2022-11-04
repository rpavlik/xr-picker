// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

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

/// Immediate-mode GUI for when we're in an error condition
fn show_error<T: Platform>(
    platform: &T,
    err: Error,
    ctx: &egui::Context,
) -> Result<InnerState<T>, Error> {
    let repopulate = egui::CentralPanel::default()
        .show(ctx, |ui| {
            ui.heading(format!("ERROR! {:?}", err));
            if ui.button("Refresh").clicked() {
                return true;
            }
            false
        })
        .inner;

    if repopulate {
        return InnerState::new(platform);
    }
    Err(err)
}

/// Immediate-mode GUI routine for normal operation
fn show_state<T: Platform>(
    platform: &T,
    state: InnerState<T>,
    ctx: &egui::Context,
) -> Result<InnerState<T>, Error> {
    let repopulate = egui::CentralPanel::default()
        .show(ctx, |ui| {
            ui.heading("OpenXR Runtime Picker");

            // The closure this calls returns true if we should refresh the list
            egui::Grid::new("runtimes")
                .striped(true)
                .min_col_width(ui.spacing().interact_size.x * 2.0) // widen to avoid resizing based on default runtime
                .num_columns(4)
                .show(ui, |ui| {
                    let mut repopulate = false;
                    ui.label(""); // for button
                    ui.label("Runtime Name");
                    ui.label("State");
                    ui.label("Details");
                    ui.end_row();

                    for runtime in &state.runtimes {
                        let runtime_active_state =
                            platform.get_runtime_active_state(runtime, &state.active_data);
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
                .inner // get at the closure's inner return value
        })
        .inner?; // get at the nested closure's return value (whether to repopulate), after handling errors.
    if repopulate {
        return state.refresh(platform);
    }
    Ok(state)
}

impl<T: Platform> eframe::App for PickerApp<T> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(state_or_error) = self.state.take() {
            let new_state = match state_or_error {
                Ok(state) => show_state(&self.platform, state, ctx),
                Err(e) => show_error(&self.platform, e, ctx),
            };
            self.state.replace(new_state);
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
