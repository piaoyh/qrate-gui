use iced::{ Element, Task };
use iced::widget::{ column, row, center, text };

// The `control_tower` module is now brought in via `lib.rs`, so `mod control_tower;` is no longer needed here.
use qrate_gui::{ ControlTower, Message }; // Using crate path
fn main() -> iced::Result
{
    // Removed `pub` as it's an example binary
    // To prevent lifetime errors, .title() and .theme() have been removed.
    // Only the basic form of application().run() remains.
    iced::application(
        ControlTower::new,
        ControlTower::update, 
        ControlTower::view
    )
    .run()
}


