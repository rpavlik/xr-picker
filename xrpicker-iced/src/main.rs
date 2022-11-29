// Copyright 2022, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::iter::once;

use iced::{
    executor,
    widget::{button, horizontal_space, row, text, vertical_space, Column, Text},
    Application, Command, Element, Length, Settings, Theme,
};
use itertools::Itertools;
use xrpicker::{
    make_platform, platform::PlatformRuntime, AppState, ConcretePlatform, Error, Platform,
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    RefreshPressed,
    MakeActivePressed(usize),
    Repopulate,
}

type Renderer = iced::Renderer<Theme>;

struct PickerApp<T: Platform> {
    platform: T,
    state: Option<Result<AppState<T>, Error>>,
}

impl<T: Platform> PickerApp<T> {
    fn new(platform: T) -> Self {
        let state = Some(AppState::new(&platform));

        PickerApp { platform, state }
    }
}

const ACTIVE_BUTTON_WIDTH: Length = Length::FillPortion(1);
const NAME_WIDTH: Length = Length::FillPortion(2);
const STATE_WIDTH: Length = Length::FillPortion(1);
const DESCRIBE_WIDTH: Length = Length::FillPortion(5);
const ROW_SPACING: u16 = 10;

const PROJECT_URL: &str = "https://github.com/rpavlik/xr-picker";

fn make_about_contents() -> Element<'static, Message, Renderer> {
    row![
        text("This is an open-source software project, maintained at"),
        text(PROJECT_URL), // TODO hyperlink
        text(". You are welcome and encouraged to participate in development."),
    ]
    .spacing(5)
    .into()
}

/// Trait implemented for all states of the GUI.
trait GuiView<T: Platform> {
    fn view(&self, platform: &T) -> Element<'_, Message, Renderer>;
}

impl<T: Platform> GuiView<T> for Error {
    fn view(&self, _platform: &T) -> Element<'_, Message, Renderer> {
        let contents = Column::new()
            .push(text("ERROR!"))
            .push(text(format!("{:?}", self)))
            .push(button("Refresh").on_press(Message::Repopulate));
        contents.into()
    }
}

// These are GUI-specific impls on our AppState
trait IcedAppState<T: Platform> {
    /// Add the non-fatal errors from manifest parsing to the UI
    fn make_non_fatal_errors_listing(&self) -> Element<'_, Message, iced::Renderer<iced::Theme>>;

    /// Adds a grid with the runtimes to the given `egui::Ui`, handling "make active" button presses.
    ///
    /// Returns an error (in which case that becomes the new state), or a boolean indicating whether to refresh.
    fn make_runtime_grid(&self, platform: &T) -> Element<'_, Message, iced::Renderer<iced::Theme>>;
}

fn make_grid_header() -> Element<'static, Message, Renderer> {
    let contents = row![
        horizontal_space(ACTIVE_BUTTON_WIDTH),
        text("Runtime Name").width(NAME_WIDTH),
        text("State").width(STATE_WIDTH),
        text("Details").width(DESCRIBE_WIDTH),
    ];
    contents.spacing(ROW_SPACING).into()
}

impl<T: Platform> IcedAppState<T> for AppState<T> {
    fn make_non_fatal_errors_listing(&self) -> Element<'_, Message, iced::Renderer<iced::Theme>> {
        if self.nonfatal_errors.is_empty() {
            return vertical_space(Length::Shrink).into();
        }

        let rows = Column::new()
            .push(text("Non-fatal errors from manifest loading:"))
            .push(text(
                self.nonfatal_errors
                    .iter()
                    .map(|e| format!("- {} - {:?}\n", e.0.display(), e.1))
                    .join("\n"),
            ));
        rows.into()
    }

    fn make_runtime_grid(&self, platform: &T) -> Element<'_, Message, Renderer> {
        let runtime_rows = once(make_grid_header())
            .chain(self.runtimes.iter().enumerate().map(|(i, runtime)| {
                let runtime_active_state =
                    platform.get_runtime_active_state(runtime, &self.active_data);
                let state_string = format!("{}", runtime_active_state);

                let active_button: Element<_, _> =
                    if runtime_active_state.should_provide_make_active_button() {
                        button("Make active")
                            .width(ACTIVE_BUTTON_WIDTH)
                            .on_press(Message::MakeActivePressed(i))
                            .into()
                    } else {
                        horizontal_space(ACTIVE_BUTTON_WIDTH).into()
                    };
                let contents = row![
                    active_button,
                    text(runtime.get_runtime_name()).width(NAME_WIDTH),
                    text(&state_string).width(STATE_WIDTH),
                    text(runtime.describe()).width(DESCRIBE_WIDTH),
                ];
                contents.spacing(ROW_SPACING).into()
            }))
            .collect();
        Column::with_children(runtime_rows)
            .spacing(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn header_with_refresh_button() -> Element<'static, Message, Renderer> {
    let header_row = row![
        Text::new("OpenXR Runtime Picker").size(26),
        horizontal_space(Length::Fill),
        button("Refresh").on_press(Message::RefreshPressed),
    ];

    header_row.padding(10).into()
}

impl<T: Platform> GuiView<T> for AppState<T> {
    fn view(&self, platform: &T) -> Element<'_, Message, Renderer> {
        let runtime_rows = self.make_runtime_grid(platform);
        let contents = Column::new()
            .padding(10)
            .spacing(10)
            .push(header_with_refresh_button())
            .push(runtime_rows)
            .push(vertical_space(Length::Fill))
            .push(self.make_non_fatal_errors_listing())
            .push(make_about_contents());

        contents.into()
    }
}

impl Application for PickerApp<ConcretePlatform> {
    type Message = Message;

    fn title(&self) -> String {
        "XR Picker".to_string()
    }

    type Executor = executor::Default;

    type Theme = Theme;

    type Flags = ();

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        let state = self.state.take();
        let state = state.unwrap();
        match message {
            Message::RefreshPressed => {
                if let Ok(state) = state {
                    self.state.replace(state.refresh(&self.platform));
                }
            }
            Message::MakeActivePressed(i) => {
                if let Ok(state) = state {
                    if let Err(e) = state.runtimes[i].make_active() {
                        self.state = Some(Err(e));
                    } else {
                        self.state = Some(Ok(state));
                    }
                }
            }
            Message::Repopulate => {
                self.state = Some(AppState::new(&self.platform));
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        match self.state.as_ref().unwrap() {
            Ok(state) => state.view(&self.platform),
            Err(e) => e.view(&self.platform),
        }
    }

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let platform = make_platform();

        (Self::new(platform), Command::none())
    }
}

pub fn main() -> iced::Result {
    PickerApp::<ConcretePlatform>::run(Settings::default())
}
