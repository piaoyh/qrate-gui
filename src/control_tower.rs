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
        rust_i18n::set_locale("en"); // Set initial locale for the application
        (
            Self {
                selected_file_path: None,
                current_menu_key: None,
                menu_font_size_in_pixel: 24.0,
                current_locale: "en".to_string(), // Initialize current_locale field
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

        // Calculate x-position of the current menu button's left edge
        let mut current_menu_offset_x = 0.0;
        let menu_bar_outer_padding = 5.0; // .padding(5) on menu_bar row
        current_menu_offset_x += menu_bar_outer_padding; // 메뉴바 전체의 왼쪽 패딩

        if let Some(current_menu_key_str) = &self.current_menu_key
        {
            for &key in &menu_keys
            {
                if key == current_menu_key_str.as_str()
                    { break; }
                
                let text_width_estimate = self.calculate_text_width_estimate(t!(key).as_ref());
                // 버튼의 실제 렌더링 너비는 텍스트 너비 + button_padding * 2 (좌우 패딩)로 추정합니다.
                let button_total_width = text_width_estimate + (button_padding * 2.0) + 1.0; 
                current_menu_offset_x += button_total_width + menu_bar_spacing;
            }
        }

        let menu_bar = row(menu_keys.into_iter().map(|key| {
            button(text(t!(key)).size(self.menu_font_size_in_pixel))
                .on_press(Message::MenuClicked(key.to_string()))
                .padding(button_padding as u16)
                .width(Length::Shrink)
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

        // Submenu area
        let sub_menu_area: Element<'_, Message> = if let Some(menu_key) = &self.current_menu_key
        {
            let items = match menu_key.as_str()
            {
                "problem-bank-management" => vec![
                    "create-new-problem-bank",
                    "load",
                    "edit",
                    "export",
                    "export-as",
                    "optimize",
                ],
                "generate-exam-paper" => vec![
                    "load-problem-bank",
                    "criteria-for-problem-extraction",
                    "load-student-list",
                    "export-exam-paper",
                ],
                "student-list-management" => vec![
                    "load",
                    "edit",
                    "export",
                    "export-as",
                ],
                "learning" => vec![
                    "load-problem-bank",
                    "criteria-for-problem-extraction",
                    "grading-criteria",
                    "take-exam",
                ],
                "settings" => vec![
                    "storage-path",
                    "atmosphere",
                    "font",
                    "language",
                ],
                "information" => vec![
                    "help",
                    "software-info",
                    "copyright-info",
                ],
                _ => vec!["coming-soon"],
            };

            container(
                column(items.into_iter().map(|item_key| {
                    let on_press_message = if menu_key == "settings" && item_key == "language" {
                        Message::GoToPage("language-settings".to_string())
                    } else {
                        Message::SubMenuClicked(item_key.to_string())
                    };

                    button(text(t!(item_key)).size(self.menu_font_size_in_pixel))
                        .on_press(on_press_message)
                        .width(Length::Fill)
                        .padding(8)
                        .style(|_theme: &Theme, status| {
                            let mut style = button::Style::default();
                            style.background = Some(Color::WHITE.into());
                            style.text_color = Color::BLACK;

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
            .into()
        }
        else
        {
            iced::widget::Space::new().into() // 서브메뉴 없으면 빈 공간
        };

        // Render main content or specific page based on current_page
        let main_content_area: Element<'_, Message> = match self.current_page.as_str() {
            "main" => {
                // 3. 메인 화면
                let path_text = if let Some(path) = &self.selected_file_path
                    { t!("selected-file", path = path.to_string_lossy().as_ref()).to_string() }
                else
                    { t!("no-file-selected").to_string() };

                center(
                    column![
                        text(t!("welcome-message")).size(32),
                        text(path_text).size(18),
                    ]
                    .spacing(20)
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
            },
            "language-settings" => {
                // Language selection page
                let available_locales = Self::get_available_locales();
                let current_i18n_locale = rust_i18n::locale(); // Get current i18n locale
                let language_buttons = available_locales.into_iter().fold(
                    column![].spacing(10),
                    |col: iced::widget::Column<'_, Message>, (language_name, locale)| {
                        col.push(
                            button(text(language_name).size(self.menu_font_size_in_pixel))
                                .on_press(Message::SetLocale(locale))
                                .width(Length::Fill)
                                .padding(8),
                        )
                    },
                );

                column![
                    text(t!("language")).size(32),
                    language_buttons,
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

        // menu_bar의 높이를 추정합니다 (폰트 크기 + 버튼 패딩 * 2 + 메뉴 바 외부 패딩 * 2)
        // menu_bar_outer_padding은 row 전체에 적용되는 padding이므로 실제 높이에 2배 적용
        let menu_bar_height_estimate = self.menu_font_size_in_pixel + (button_padding * 2.0) + (menu_bar_outer_padding * 2.0);

        // 기본 콘텐츠 (menu_bar + main_content_area)
        let content: Element<'_, Message> = column![
            menu_bar,
            main_content_area,
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into();

        // 만약 메뉴가 열려있다면 overlay를 적용합니다.
        if let Some(_) = &self.current_menu_key {
            iced::widget::container(content).overlay(
                iced::widget::overlay::menu::menu(
                    sub_menu_area, // 서브메뉴 콘텐츠
                    current_menu_offset_x, // X 위치
                    menu_bar_height_estimate, // Y 위치
                )
            ).into() // Element<'_, Message> with overlay
        } else {
            content // overlay 없이 일반 콘텐츠 반환
        }
    }

    // fn get_available_locales() -> Vec<(String, String)>
    /// Returns a list of available locales by reading the `assets/locales` directory.
    ///
    /// # Output
    /// A `Vec<(String, String)>` where each tuple contains the language name and the locale code.
    ///
    /// # Examples
    /// ```no_run
    /// use crate::control_tower::ControlTower;
    /// 
    /// let locales = ControlTower::get_available_locales();
    /// assert!(!locales.is_empty());
    /// ```
    fn get_available_locales() -> Vec<(String, String)>
    {
        let mut locales = Vec::new();

        for file in LOCALES_DIR.files() {
            if let Some(file_name_os) = file.path().file_name() {
                if let Some(file_name) = file_name_os.to_str() {
                    if file_name.ends_with(".yml") {
                        let locale = file_name.trim_end_matches(".yml");
                        let language_name = match locale {
                            "en" => "English".to_string(),
                            "ko" => "한국어".to_string(),
                            "ru" => "Русский".to_string(),
                            _ => locale.to_string(),
                        };
                        locales.push((language_name.clone(), locale.to_string()));
                    }
                }
            }
        }
        locales
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