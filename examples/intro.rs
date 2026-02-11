// For getting to know how to use slint and developping ideas

#![windows_subsystem = "windows"]

use slint::{self, SharedString, Weak};
use rfd::FileDialog;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError>
{
    let main_window = MainWindow::new()?;

    // Use a weak reference for the load_question_bank callback
    let main_window_weak_lb = main_window.as_weak();
    main_window.on_load_question_bank(move || {
        let path = FileDialog::new()
            .add_filter("Question Bank", &["qbdb", "qb.xlsx"])
            .set_directory(".")
            .pick_file();

        let path_question_bank = if let Some(path_buf) = path
        {
            SharedString::from(path_buf.to_string_lossy().into_owned())
        }
        else
        {
            SharedString::from("파일이 선택되지 않았습니다.")
        };

        // Get a strong reference and invoke the show_path_dialog callback
        if let Some(main_window_strong) = main_window_weak_lb.upgrade() {
            main_window_strong.invoke_show_path_dialog(path_question_bank);
        }
    });

    // Use a weak reference for the show_path_dialog callback
    let main_window_weak_show = main_window.as_weak();
    main_window.on_show_path_dialog(move |path: SharedString| {
        // Here we don't need to unwrap main_window_weak_show as it's not used to invoke Slint callbacks
        // but if we wanted to access the UI from here, we would:
        // if let Some(main_window_strong) = main_window_weak_show.upgrade() { ... }
        rfd::MessageDialog::new()
            .set_title("선택된 파일 경로")
            .set_description(&path)
            .show();
    });

    main_window.run()
}