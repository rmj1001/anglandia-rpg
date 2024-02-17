use crate::{
    combat::{battle::BattleSettings, enemy::Enemy},
    data::{achievements::Achievements, settings::Settings},
    utils::{
        input::{confirm, prompt_arrow},
        messages::{self, success},
        terminal::exit,
        tui::{self, page_header, press_enter_to_continue, sleep, HeaderSubtext},
    },
};

use crate::data::player::Player;

pub fn main(player: &mut Player) {
    // Check for achievements at login to keep the player file up to date
    Achievements::check(player);

    page_header(
        format!("Game Menu (Player: {})", player.settings.username),
        tui::HeaderSubtext::EnterCode,
    );

    tui::small_header("Combat", HeaderSubtext::None);
    println!("c1. Wander the Realm");
    println!("c2. Enter the Stronghold");
    println!();

    tui::small_header("Economy", HeaderSubtext::None);
    println!("e1. The Guilds");
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
            let mut battle_settings = BattleSettings {
                header: "Wandering Gielnor",
                prompt: "You are wandering the realm...",
                enemy: Enemy::new(player.xp.combat, player.health.hp),
                player,
                loops: 0,
                floor: 0,
                is_first_battle: true,
                is_looped: false,
                pause_seconds: 1,
                end_function: None,
            };

            crate::combat::battle::new_battle(&mut battle_settings);
        }
        "c2" | "enter the stronghold" => {
            page_header("The Stronghold", HeaderSubtext::None);

            fn exit_stronghold(player: &mut Player) {
                page_header("The Stronghold", HeaderSubtext::None);

                println!("\nYou have successfully completed the stronghold and won the game! Congratulations!");
                player.achievements.stronghold_defeated = true;
                player.save();

                press_enter_to_continue();
                main(player);
            }

            let mut battle_settings = BattleSettings {
                header: "The Stronghold",
                prompt: "You delve into the stronghold...",
                enemy: Enemy::new(player.xp.combat, player.health.hp),
                player: &mut player.clone(),
                loops: 50,
                floor: 0,
                is_first_battle: true,
                is_looped: true,
                pause_seconds: 1,
                end_function: Some(exit_stronghold),
            };

            let confirm_stronghold = confirm(&format!(
                "Are you sure you want to enter the stronghold? You must win {} hard battles.",
                battle_settings.loops
            ));

            if !confirm_stronghold {
                main(player);
            }

            crate::combat::battle::new_battle(&mut battle_settings);
        }

        // Economy
        "e1" | "the guilds" => crate::menus::economy::e1_the_guilds::main(player),
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

        "exit" => {
            exit(Some(player));
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
            Settings::toggle_developer(player);
            main(player);
        }

        wrong_input => {
            messages::invalid_input(Some(wrong_input), None, true);
            main(player);
        }
    }
}
