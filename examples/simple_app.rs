
// The `control_tower` module is now brought in via `lib.rs`, so `mod control_tower;` is no longer needed here.
use qrate_gui::ControlTower; // Using crate path

fn main() -> iced::Result
{
    ControlTower::run()
}


