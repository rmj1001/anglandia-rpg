use crate::{
    lib::{
        input::{self, out_of_bounds, select_from_str_array},
        tui::page_header,
    },
    user::profile::UserProfile,
};

pub fn main(user: &mut UserProfile) {
    page_header("Trading Post", crate::lib::tui::HeaderSubtext::None);

    user.inventory.print_table();

    let buysell = select_from_str_array(&["1. Purchase", "2. Sell", "NAV: Go Back"], None);

    match buysell {
        0 => purchase(user),
        1 => sell(user),
        2 => crate::menus::game_menu::main(user),
        _ => out_of_bounds(None),
    }

    crate::menus::game_menu::main(user);
}

pub fn purchase(user: &mut UserProfile) {
    let (item, quantity) = get_item_and_quantity(user)
        .expect("get_item_and_quantity() didn't go back to the main menu.");

    // TODO: inventory arithmetic here.
}

pub fn sell(user: &mut UserProfile) {
    let (item, quantity) = get_item_and_quantity(user)
        .expect("get_item_and_quantity() didn't go back to the main menu.");

    // TODO: inventory arithmetic here.
}

fn get_item_and_quantity(user: &mut UserProfile) -> Option<(String, usize)> {
    let items: Vec<String> = vec![
        "bait".to_string(),
        "seeds".to_string(),
        "furs".to_string(),
        "fish".to_string(),
        "food".to_string(),
        "wood".to_string(),
        "ore".to_string(),
        "ingots".to_string(),
        "potions".to_string(),
        "rubies".to_string(),
        "magic scrolls".to_string(),
        "bones".to_string(),
        "dragon hides".to_string(),
        "runic_tablets".to_string(),
    ];

    let result = input::get_item_and_quantity(items);

    match result {
        Ok(tuple) => Some(tuple),
        Err(_) => {
            main(user);
            None
        }
    }
}
