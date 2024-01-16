use crate::lib::{
    input::{self, prompt_input},
    terminal,
    tui::{self, page_header, press_enter_to_continue, HeaderInstructions},
};

use crate::user::profile::UserProfile;

pub fn main(user: &mut UserProfile) {
    page_header(
        &format!("Game Menu (user: {})", user.settings.username),
        tui::HeaderInstructions::EnterCode,
    );

    tui::small_header("Combat", HeaderInstructions::None);
    println!("c1. Wander the Realm");
    println!("c2. Enter the Stronghold");
    println!("\n");

    tui::small_header("Economy", HeaderInstructions::None);
    println!("e1. Work in the Guilds");
    println!("e2. The Bank");
    println!("e3. Trading Post");
    println!("e4. Weapons Shop");
    println!("e5. Armor Shop");
    println!("e6. Mystic Shop");
    println!("e7. Celestial Shop");

    tui::small_header("Profile", HeaderInstructions::None);
    println!("p1. Inventory");
    println!("p2. Hall of Records");

    println!("\n");
    if user.settings.developer {
        println!("d1. Developer Menu");
    }
    println!("n1. Settings");
    println!("n2. Logout");
    println!("n3. Exit Game\n");

    let choice = prompt_input("Enter Menu Code").to_lowercase();

    match &choice[..] {
        // Combat
        "c1" => crate::menus::game::combat::c1_the_stronghold::main(user),
        "c2" => crate::menus::game::combat::c2_wander_realm::main(user),

        // Economy
        "e1" => crate::menus::game::economy::e1_the_guilds::main(user),
        "e2" => crate::menus::game::economy::e2_the_bank::main(user),
        "e3" => crate::menus::game::economy::e3_trading_post::main(user),
        "e4" => crate::menus::game::economy::e4_weapons_shop::main(user),
        "e5" => crate::menus::game::economy::e5_armor_shop::main(user),
        "e6" => crate::menus::game::economy::e6_mystic_shop::main(user),
        "e7" => crate::menus::game::economy::e7_celestial_shop::main(user),

        // Profile
        "p1" => crate::menus::game::profile::p1_inventory::main(user),
        "p2" => crate::menus::game::profile::p2_hall_of_records::main(user),
        "n1" => crate::menus::game::profile::n1_settings::main(user),
        "n2" => {
            user.save();
            crate::menus::accounts::accounts::main();
        }
        "n3" => {
            user.save();
            terminal::exit();
        }

        // Developer Mode
        "d1" => {
            if user.settings.developer {
                crate::menus::game::profile::d1_developer_menu::main(user);
            } else {
                input::invalid_input(None, None, true);
                main(user);
            }
        }

        "3.141592" => {
            if !user.settings.developer {
                page_header("Developer Mode", HeaderInstructions::None);
                user.achievements.hacked_the_game = true;
                user.settings.set_developer(None, true);

                println!("\nDeveloper mode enabled.");
                press_enter_to_continue();
                main(user);
            } else {
                super::profile::d1_developer_menu::disable_developer_mode(user);
            }
        }

        wrong_input => {
            input::invalid_input(Some(wrong_input), None, true);
            main(user);
        }
    }
}
