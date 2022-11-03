// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use eframe::egui;
use xrpicker::{make_platform, platform::PlatformRuntime, Error, Platform};

struct InnerState<T: Platform> {
    runtimes: Vec<T::PlatformRuntimeType>,
    active_data: T::PlatformActiveData,
}

impl<T: Platform> InnerState<T> {
    fn new(platform: &T) -> Result<Self, Error> {
        let runtimes = platform.find_available_runtimes()?;
        let active_data = platform.get_active_data();
        Ok(InnerState {
            runtimes,
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

fn update<T: Platform>(
    platform: &T,
    result_or_state: Result<InnerState<T>, Error>,
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
) -> Result<InnerState<T>, Error> {
    match result_or_state {
        Ok(state) => {
            let mut repopulate = false;
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("OpenXR Runtime Picker");

                egui::Grid::new("runtimes")
                    .striped(true)
                    .num_columns(4)
                    .show(ui, |ui| {
                        ui.label(""); // for button
                        ui.label("Runtime Name");
                        ui.label("State");
                        ui.label("Details");
                        ui.end_row();

                        for runtime in &state.runtimes {
                            let runtime_active_state =
                                platform.get_runtime_active_state(runtime, &state.active_data);
                            if runtime_active_state.provide_make_active_button() {
                                if ui.button("Make active").clicked() {
                                    if let Err(e) = runtime.make_active() {
                                        eprintln!("error in make_active: {:?}", e);
                                    }
                                    repopulate = true;
                                }
                            } else {
                                ui.label("");
                            }
                            ui.label(runtime.get_runtime_name());
                            ui.label(format!("{}", runtime_active_state));
                            ui.label(runtime.describe());
                            // ui.vertical(|ui| {
                            //     for manifest in runtime.get_manifests() {
                            //         ui.label(format!("{}", manifest.display()));
                            //     }
                            // });
                            // ui.vertical(|ui| {
                            //     for library in runtime.get_libraries() {
                            //         ui.label(format!("{}", library.display()));
                            //     }
                            // });
                            ui.end_row();
                            // ui.horizontal(|ui| {
                            //     ui.label(format!(
                            //         "{:?}",
                            //         platform.get_runtime_active_state(runtime, &state.active_data)
                            //     ));
                            //     ui.label(runtime.get_runtime_name());
                            // });
                        }
                    });
            });
            if repopulate {
                return InnerState::new(platform);
            }
            return Ok(state);
        }
        Err(e) => {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading(format!("ERROR! {}", e));
            });
            return Err(e);
        }
    }
}

impl<T: Platform> eframe::App for PickerApp<T> {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let Some(state) = self.state.take() {
            self.state
                .replace(update(&self.platform, state, ctx, frame));
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
