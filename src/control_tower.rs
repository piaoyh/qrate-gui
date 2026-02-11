// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use qrate::{ Generator, QBank, SBank };
use slint::{ self, SharedString, Weak, PlatformError };
use rfd::FileDialog;
use std::rc::Rc; // For Rc
use std::cell::RefCell; // For RefCell


slint::include_modules!();


pub struct ControlTower
{
    generator: Generator,
    main_window: MainWindow,
    main_window_weak: Weak<MainWindow>,
}

impl ControlTower
{
    pub fn new() -> Result<Rc<RefCell<Self>>, String>
    {
        let generator;
        match Generator::new(&QBank::new_empty(), 1, 1, 1, &SBank::new())
        {
            Some(g) => { generator = g; },
            None => { return Err("Failed to create Generator object.".to_string()); }
        }

        let main_window;
        match MainWindow::new()
        {
            Ok(w) => { main_window = w; },
            Err(e) => { return Err(e.to_string()); }
        }

        let main_window_weak = main_window.as_weak();
        Ok(Rc::new(RefCell::new(Self { generator, main_window, main_window_weak })))
    }

    pub fn set_functionalities(control_tower_rc: &Rc<RefCell<Self>>) -> Result<(), String>
    {
        // Clone the Rc for the load_question_bank callback
        let control_tower_rc_lb = control_tower_rc.clone();
        control_tower_rc.borrow().main_window.on_load_question_bank(move || {
            // Get mutable access to ControlTower inside the closure
            if let Ok(mut ct) = control_tower_rc_lb.try_borrow_mut() {
                ct.load_question_bank();
            } else {
                eprintln!("Failed to borrow ControlTower mutably for load_question_bank.");
            }
        });

        // The show_path_dialog callback is on the main_window, not ControlTower.
        // It's implemented in examples/intro.rs as main_window.on_show_path_dialog.
        // So this block of code for `on_show_path_dialog` should remain in simple_app.rs if it's there.
        // If the intent is for ControlTower to set this callback, it needs to capture main_window_weak.
        // Let's assume ControlTower is responsible for setting all window callbacks.
        // So the `on_show_path_dialog` should be set here.
        let control_tower_rc_show = control_tower_rc.clone();
        control_tower_rc.borrow().main_window.on_show_path_dialog(move |path: SharedString| {
            // This callback is already correctly handling the display via rfd::MessageDialog
            // and does not need to call back into ControlTower for now.
            rfd::MessageDialog::new()
                .set_title("선택된 파일 경로")
                .set_description(&path)
                .show();
        });
        Ok(())
    }

    pub fn run(control_tower_rc: Rc<RefCell<Self>>) -> Result<(), String>
    {
        // To call main_window.run(), which consumes main_window, we need to take ownership of ControlTower
        // and then extract the main_window from it.
        let control_tower = Rc::try_unwrap(control_tower_rc)
            .map_err(|_| "Failed to unwrap Rc from ControlTower".to_string())?
            .into_inner();

        match control_tower.main_window.run()
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn load_question_bank(&mut self)
    {
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
        if let Some(main_window_strong) = self.main_window_weak.upgrade()
        {
            main_window_strong.invoke_show_path_dialog(path_question_bank);
        }
    }
}

impl Rc<RefCell<ControlTower>>
{

}