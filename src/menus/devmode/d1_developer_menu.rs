use crate::lib::{
    input::{confirm, out_of_bounds, select_from_str_array},
    tui::{self, page_header, press_enter_to_continue, HeaderSubtext},
};

use crate::user::profile::UserProfile;

pub fn main(user: &mut UserProfile) {
    page_header("Developer Mode", HeaderSubtext::Keyboard);

    let choice = select_from_str_array(
        &[
            "1. Throw a panic",
            "2. Manipulate Inventory",
            "3. Manipulate XP",
            "4. Manipulate Banks",
            "5. Manage User Profiles",
            "6. Disable developer mode",
            "NAV: Go Back",
        ],
        None,
    );

    match choice {
        0 => panic!("This is a panic!"),
        1 => super::d4_inventory_mgr::main(user),
        2 => super::d3_xp_mgr::main(user),
        3 => super::d5_bank_mgr::main(user),
        4 => super::d2_user_mgr::main(user),
        5 => disable_developer_mode(user),
        6 => crate::menus::game_menu::main(user),
        _ => out_of_bounds(None),
    }
}

pub fn disable_developer_mode(user: &mut UserProfile) {
    page_header("Developer Mode", HeaderSubtext::None);

    let disable_dev_mode = confirm("Are you sure you want to disable developer mode?");

    if !disable_dev_mode {
        println!("\nAborting.");
        press_enter_to_continue();
        main(user);
    }

    user.settings.set_developer(None, false);
    println!("\nDeveloper mode disabled.");
    tui::press_enter_to_continue();

    crate::menus::game_menu::main(user);
}