// For developing qrate-gui

#![windows_subsystem = "windows"]


use qrate_gui::ControlTower;
use std::rc::Rc;
use std::cell::RefCell;

fn main() -> Result<(), String>
{
    let control_tower_rc = ControlTower::new()?;
    ControlTower::set_functionalities(&control_tower_rc)?;
    ControlTower::run(control_tower_rc)
}