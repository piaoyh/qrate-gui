// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use std::path::PathBuf;

use iced::widget::{ button, column, row, text, center, container };
use iced::{ Element, Task, Theme, Length, Color };
use rfd::FileDialog;
use rust_i18n::t;


/// Represents the messages that can be sent to the [ControlTower] to update its state.
#[derive(Debug, Clone)]
pub enum Message
{
    /// Indicates that a file has been selected, carrying its path.
    FileSelected(Option<PathBuf>),

    /// Signals that a main menu item has been clicked, with the menu's name.
    MenuClicked(String),

    /// Signals that a sub-menu item has been clicked, with the sub-menu's name.
    SubMenuClicked(String),

    /// Sets the application's locale to the given string (e.g., "en-US", "ko-KR").
    SetLocale(String),

    /// Navigates to a specified page.
    GoToPage(String),
}

/// The main application state and logic holder for the Qrate-GUI.
///
/// // pub struct ControlTower
#[derive(Debug, Default)]
pub struct ControlTower
{
    /// The currently selected file path, if any.
    selected_file_path: Option<PathBuf>,

    /// The key of the currently active main menu, used for localization.
    current_menu_key: Option<String>,

    /// The font size in pixels for menu items.
    menu_font_size_in_pixel: f32,

    /// The current locale of the application, used for internationalization.
    current_locale: String,

    /// The current active page or view to display.
    current_page: String,
}

impl ControlTower
{
    // pub fn new() -> (Self, Task<Message>)
    /// Creates a new instance of [ControlTower] with default values.
    ///
    /// # Output
    /// A tuple containing the new [ControlTower] instance and an initial [iced::Task].
    ///
    /// # Examples
    /// ```
    /// use iced::Task;
    /// use crate::control_tower::{ControlTower, Message};
    ///
    /// let (control_tower, task) = ControlTower::new();
    /// assert_eq!(control_tower.selected_file_path, None);
    /// assert_eq!(control_tower.current_menu_key, None);
    /// assert_eq!(control_tower.menu_font_size_in_pixel, 24.0);
    /// assert_eq!(control_tower.current_locale, "en-US".to_string());
    /// assert_eq!(control_tower.current_page, "main".to_string());
    /// assert!(task.is_none());
    /// ```
    pub fn new() -> (Self, Task<Message>)
    {
        rust_i18n::set_locale("en-US"); // Set initial locale for the application
        (
            Self {
                selected_file_path: None,
                current_menu_key: None,
                menu_font_size_in_pixel: 24.0,
                current_locale: "en-US".to_string(), // Initialize current_locale field
                current_page: "main".to_string(), // Initialize current_page field
            },
            Task::none(),
        )
    }

    // pub fn update(&mut self, message: Message) -> Task<Message>
    /// Handles incoming messages and updates the state of the [ControlTower].
    ///
    /// # Arguments
    /// * `message` - The [Message] to be processed.
    ///
    /// # Output
    /// An [iced::Task] that may produce further messages.
    ///
    /// # Examples
    /// ```
    /// use iced::Task;
    /// use crate::control_tower::{ControlTower, Message};
    ///
    /// let (mut control_tower, _) = ControlTower::new();
    ///
    /// // Test Message::MenuClicked
    /// control_tower.update(Message::MenuClicked("settings".to_string()));
    /// assert_eq!(control_tower.current_menu_key, Some("settings".to_string()));
    ///
    /// control_tower.update(Message::MenuClicked("settings".to_string())); // Click again to close
    /// assert_eq!(control_tower.current_menu_key, None);
    ///
    /// // Test Message::SetLocale
    /// control_tower.update(Message::SetLocale("ko-KR".to_string()));
    /// assert_eq!(control_tower.current_locale, "ko-KR".to_string());
    ///
    /// // Test Message::GoToPage
    /// control_tower.update(Message::GoToPage("language-settings".to_string()));
    /// assert_eq!(control_tower.current_page, "language-settings".to_string());
    /// ```
    pub fn update(&mut self, message: Message) -> Task<Message>
    {
        match message
        {
            Message::MenuClicked(menu_key) => {
                if self.current_menu_key.as_deref() == Some(&menu_key)
                    { self.current_menu_key = None; }
                else
                    { self.current_menu_key = Some(menu_key); }
                Task::none()
            },
            Message::SubMenuClicked(sub_item_key) => { // sub_item을 sub_item_key로 변경
                if sub_item_key == "load" || sub_item_key == "load-problem-bank" // 키로 비교
                {
                    return Task::perform(Self::pick_file(), Message::FileSelected);
                }
                self.current_menu_key = None; // 현재 메뉴 키를 초기화
                Task::none()
            },
            Message::FileSelected(path) => {
                self.selected_file_path = path;
                self.current_menu_key = None; // current_menu_key로 변경
                Task::none()
            },
            Message::SetLocale(locale) => {
                rust_i18n::set_locale(&locale);
                self.current_locale = locale;
                Task::none()
            },
            Message::GoToPage(page_name) => {
                self.current_page = page_name;
                Task::none()
            },
        }
    }

    // fn calculate_text_width_estimate(&self, name: &str) -> f32
    /// Calculates the estimated width of a given string `name` based on character type and font size.
    ///
    /// # Arguments
    /// * `name` - The string whose width is to be estimated.
    ///
    /// # Output
    /// The estimated width of the string as an `f32`.
    ///
    /// # Examples
    /// ```
    /// let control_tower = ControlTower {
    ///     selected_file_path: None,
    ///     current_menu: None,
    ///     menu_font_size_in_pixel: 20.0
    /// };
    /// let name = "문제은행 관리";
    /// assert_eq!(control_tower.calculate_text_width_estimate(name), 120.0);
    /// let name = "File Menu";
    /// assert_eq!(control_tower.calculate_text_width_estimate(name), 80.0);
    /// let name = "한글 abc";
    /// assert_eq!(control_tower.calculate_text_width_estimate(name), 80.0);
    /// ```
    fn calculate_text_width_estimate(&self, name: &str) -> f32
    {
        let mut width = 0.0;
        for c in name.chars()
        {
            // Check for Korean (Hangul Syllables), Hanja (CJK Unified Ideographs), Hiragana, Katakana
            if (c >= '\u{AC00}' && c <= '\u{D7AF}') || // Hangul Syllables
               (c >= '\u{4E00}' && c <= '\u{9FFF}') || // CJK Unified Ideographs (Hanja)
               (c >= '\u{3040}' && c <= '\u{30FF}')    // Hiragana & Katakana
                { width += self.menu_font_size_in_pixel; }
            else
                { width += (self.menu_font_size_in_pixel) / 2.0; }
        }
        width
    }

    // pub fn view(&self) -> Element<'_, Message>
    /// Returns the current view of the application as an [iced::Element].
    ///
    /// # Output
    /// An [iced::Element] representing the user interface.
    ///
    /// # Examples
    /// ```
    /// use iced::widget::text;
    /// use iced::Element;
    /// use crate::control_tower::{ControlTower, Message};
    ///
    /// let (control_tower, _) = ControlTower::new();
    /// let view_element: Element<'_, Message> = control_tower.view();
    /// // In a real application, you would inspect the generated Element.
    /// // For this conceptual example, we just ensure it compiles and returns an Element.
    /// // More detailed assertions would require inspecting the widget tree.
    /// assert_eq!(view_element.as_widget().is_some(), true);
    /// ```
    pub fn view(&self) -> Element<'_, Message>
    {
        // Define menu keys, not translated strings
        let menu_keys = vec![
            "problem-bank-management",
            "generate-exam-paper",
            "student-list-management",
            "learning",
            "settings",
            "information",
        ];
        let menu_bar_spacing = 10.0; // Spacing for the menu bar
        let button_padding = 5.0; // Padding for each button

        // Calculate x-offset for the currently selected menu
        let mut offset_x = 0.0;
        if let Some(current_menu_key) = &self.current_menu_key // current_menu_key 사용
        {
            for &key in &menu_keys // key 사용
            {
                if key == current_menu_key.as_str() // 키 비교
                    { break; }  // Stop calculation when the current menu is reached
                // Approximate width calculation for each menu button (text width + horizontal padding)
                // Korean, Hanja, Hiragana, and Katakana characters are calculated as full menu_font_size_in_pixel,
                // while other characters including spaces are calculated as half.
                let text_width_estimate = self.calculate_text_width_estimate(t!(key).as_ref()); // t!(key)의 결과로 width 계산
                offset_x += text_width_estimate + menu_bar_spacing; // + (button_padding * 2.0)
            }
        }

        let menu_bar = row(menu_keys.into_iter().map(|key| { // key 사용
            button(text(t!(key)).size(self.menu_font_size_in_pixel)) // t!(key)로 번역된 텍스트 표시
                .on_press(Message::MenuClicked(key.to_string())) // Message에 키 전달
                .padding(button_padding as u16)
                .style(|_theme: &Theme, status| {
                    let mut style = button::Style::default();
                    style.background = Some(Color::WHITE.into());
                    style.text_color = Color::BLACK;

                    match status
                    {
                        button::Status::Hovered => { style.background = Some(Color::from_rgb(0.9, 0.9, 0.9).into()); },
                        button::Status::Pressed => { style.background = Some(Color::from_rgb(0.8, 0.8, 0.8).into()); },
                        _ => {}
                    }
                    style
                })
                .into()
        }))
        .spacing(menu_bar_spacing)
        .padding(5);

        // Render main content or specific page based on current_page
        let main_view_content: Element<'_, Message> = match self.current_page.as_str() {
            "main" => {
                // 2. Submenu area
                let sub_menu_area: Element<'_, Message> = if let Some(menu_key) = &self.current_menu_key // current_menu_key 사용
                {
                    let items = match menu_key.as_str() // 키 비교
                    {
                        "problem-bank-management" => vec![ // 키 사용
                            "create-new-problem-bank",
                            "load",
                            "edit",
                            "export",
                            "export-as",
                            "optimize",
                        ],
                        "generate-exam-paper" => vec![ // 키 사용
                            "load-problem-bank",
                            "criteria-for-problem-extraction",
                            "load-student-list",
                            "export-exam-paper",
                        ],
                        "student-list-management" => vec![ // 키 사용
                            "load",
                            "edit",
                            "export",
                            "export-as",
                        ],
                        "learning" => vec![ // 키 사용
                            "load-problem-bank",
                            "criteria-for-problem-extraction",
                            "grading-criteria",
                            "take-exam",
                        ],
                        "settings" => vec![ // 키 사용
                            "storage-path",
                            "atmosphere",
                            "font",
                            "language", // 언어 선택 메뉴는 여기에 포함
                        ],
                        "information" => vec![ // 키 사용
                            "help",
                            "software-info",
                            "copyright-info",
                        ],
                        _ => vec!["coming-soon"], // 키 사용
                    };

                    container(
                        column(items.into_iter().map(|item_key| { // item_key 사용
                            let on_press_message = if menu_key == "settings" && item_key == "language" {
                                Message::GoToPage("language-settings".to_string())
                            } else {
                                Message::SubMenuClicked(item_key.to_string()) // Message에 키 전달
                            };

                            button(text(t!(item_key)).size(self.menu_font_size_in_pixel)) // t!(item_key)로 번역된 텍스트 표시
                                .on_press(on_press_message)
                                .width(Length::Fill)
                                .padding(8)
                                .style(|_theme: &Theme, status| {
                                    let mut style = button::Style::default();
                                    style.background = Some(Color::WHITE.into()); // Default background color
                                    style.text_color = Color::BLACK; // Default text color

                                    match status
                                    {
                                        button::Status::Hovered => { style.background = Some(Color::from_rgb(0.9, 0.9, 0.9).into()); },
                                        button::Status::Pressed => { style.background = Some(Color::from_rgb(0.8, 0.8, 0.8).into()); },
                                        _ => {},
                                    }
                                    style
                                })
                                .into()
                        }))
                        .spacing(2)
                        .width(220.0)
                    )
                    .padding(5)
                    .style(|_theme: &Theme| {
                        container::Style {
                            background: Some(Color::WHITE.into()),
                            ..Default::default()
                        }
                    })
                    .into() // 여기에 .into() 추가
                }
                else
                {
                    container(column![]).into() // 여기에 .into() 추가
                };

                // 3. 메인 화면
                let path_text = if let Some(path) = &self.selected_file_path
                    { t!("selected-file", path = path.to_string_lossy().as_ref()).to_string() } // 'path' 인자 전달 및 String으로 변환
                else
                    { t!("no-file-selected").to_string() }; // String으로 변환

                let main_content = center(
                    column![
                        text(t!("welcome-message")).size(32),
                        text(path_text).size(18),
                    ]
                    .spacing(20)
                );

                row![
                    iced::widget::Space::new().width(Length::Fixed(offset_x)), // offset_x is f32, so use it as is
                    sub_menu_area,
                    main_content,
                ]
                .into()
            },
            "language-settings" => {
                // Language selection page
                column![
                    text(t!("language")).size(32),
                    button(text("English").size(self.menu_font_size_in_pixel))
                        .on_press(Message::SetLocale("en-US".to_string()))
                        .width(Length::Fill)
                        .padding(8),
                    button(text("한국어").size(self.menu_font_size_in_pixel))
                        .on_press(Message::SetLocale("ko-KR".to_string()))
                        .width(Length::Fill)
                        .padding(8),
                    button(text("Русский").size(self.menu_font_size_in_pixel))
                        .on_press(Message::SetLocale("ru-RU".to_string()))
                        .width(Length::Fill)
                        .padding(8),
                    iced::widget::Space::new().height(Length::Fixed(20.0)),
                    button(text(t!("back")).size(self.menu_font_size_in_pixel))
                        .on_press(Message::GoToPage("main".to_string()))
                        .width(Length::Fill)
                        .padding(8),
                ]
                .spacing(10)
                .padding(20)
                .into()
            },
            _ => {
                // Default view for unknown pages
                center(text(t!("coming-soon")).size(32)).into()
            }
        };


        column![
            menu_bar,
            main_view_content,
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    // async fn pick_file() -> Option<PathBuf>
    /// Asynchronously opens a file dialog for the user to pick a question bank file.
    ///
    /// # Output
    /// An `Option<PathBuf>` representing the path to the selected file, or `None` if no file was selected.
    ///
    /// # Examples
    /// ```no_run
    /// // This is an async function that opens a GUI file dialog.
    /// // It cannot be directly tested with assert_eq! without mocking the GUI,
    /// // but here's how you would typically call it:
    /// async fn example_usage() {
    ///     use std::path::PathBuf;
    ///     use crate::control_tower::ControlTower;
    ///
    ///     let selected_path: Option<PathBuf> = ControlTower::pick_file().await;
    ///     match selected_path {
    ///         Some(path) => println!("File selected: {:?}", path),
    ///         None => println!("No file selected."),
    ///     }
    /// }
    /// ```
    async fn pick_file() -> Option<PathBuf> {
        FileDialog::new()
            .add_filter("Question Bank", &["qbdb", "xlsx"])
            .set_directory(".")
            .pick_file()
    }
}
