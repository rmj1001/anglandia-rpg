use crate::{
    player::settings::Settings,
    utils::{
        input::prompt_arrow,
        messages::{self, success, success_msg},
        tui::{self, page_header, sleep, HeaderSubtext},
    },
};

use crate::player::profile::Player;

pub fn main(player: &mut Player) {
    page_header(
        format!("Game Menu (Player: {})", player.settings.username),
        tui::HeaderSubtext::EnterCode,
    );

    tui::small_header("Combat", HeaderSubtext::None);
    println!("c1. Wander the Realm");
    println!("c2. Enter the Stronghold");
    println!();

    tui::small_header("Economy", HeaderSubtext::None);
    println!("e1. Work in the Guilds");
    println!("e2. The Bank");
    println!("e3. Trading Post");
    println!("e4. Weapons Shop");
    println!("e5. Armor Shop");
    println!();

    tui::small_header("Profile", HeaderSubtext::None);
    println!("p1. Inventory");
    println!("p2. Hall of Records");
    println!();

    if player.settings.developer {
        println!("d1. Developer Menu");
    }

    println!("n1. Settings");
    println!("n2. Save Game");
    println!("n3. Logout");
    println!();

    let choice = prompt_arrow("Enter Menu Code").to_lowercase();

    match &choice[..] {
        // Combat
        "c1" | "wander the realm" => {
            crate::combat::battle::battle("Wandering the Realm", "You are wandering the realm...", player, false)
        }
        "c2" | "enter the stronghold" => crate::menus::combat::c1_the_stronghold::main(player),

        // Economy
        "e1" | "work in the guilds" => crate::menus::economy::e1_the_guilds::main(player),
        "e2" | "the bank" => crate::menus::economy::e2_the_bank::main(player),
        "e3" | "trading post" => crate::menus::economy::e3_trading_post::main(player),
        "e4" | "weapons shop" => crate::menus::economy::e4_weapons_shop::main(player),
        "e5" | "armor shop" => crate::menus::economy::e5_armor_shop::main(player),

        // Profile
        "p1" | "inventory" => crate::menus::profile::p1_inventory::main(player),
        "p2" | "hall of records" => crate::menus::profile::p2_hall_of_records::main(player),
        "n1" | "settings" => crate::menus::profile::n1_settings::main(player),
        "n2" | "save game" | "save" => {
            println!("\nSaving game...");
            sleep(2);

            player.save();
            success();

            main(player);
        }
        "n3" | "logout" => {
            player.save();

            println!("\nLogging out...");
            sleep(2);

            crate::menus::accounts::main();
        }

        // Developer Mode
        "d1" | "developer menu" => {
            if player.settings.developer {
                crate::menus::devmode::d1_developer_menu::main(player);
            } else {
                messages::invalid_input(None, None, true);
                main(player);
            }
        }

        "3.141592" => {
            if !player.settings.developer {
                page_header("Developer Mode", HeaderSubtext::None);
                player.achievements.hacked_the_game = true;
                Settings::set_developer(player, true);

                success_msg("Developer mode enabled.");

                main(player);
            } else {
                super::devmode::d1_developer_menu::disable_developer_mode(player);
            }
        }

        wrong_input => {
            messages::invalid_input(Some(wrong_input), None, true);
            main(player);
        }
    }
}
