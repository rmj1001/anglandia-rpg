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
    println!("1. Wander the Realm");
    println!("2. Enter the Stronghold");
    println!();

    tui::small_header("Economy", HeaderSubtext::None);
    println!("3. The Guilds");
    println!("4. The Bank");
    println!("5. Trading Post");
    println!("6. Weapons Shop");
    println!("7. Armor Shop");
    println!();

    tui::small_header("Profile", HeaderSubtext::None);
    println!("8. Inventory");
    println!("9. Hall of Records");
    println!();

    if player.settings.developer {
        println!("96. Developer Menu");
    }

    println!("97. Settings");
    println!("98. Save Game");
    println!("99. Logout");
    println!();

    let choice = prompt_arrow("Enter Menu Code").to_lowercase();

    match &choice[..] {
        // Combat
        "1" | "wander the realm" => {
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
        "2" | "enter the stronghold" => {
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
        "3" | "the guilds" => crate::menus::economy::e1_the_guilds::main(player),
        "4" | "the bank" => crate::menus::economy::e2_the_bank::main(player),
        "5" | "trading post" => crate::menus::economy::e3_trading_post::main(player),
        "6" | "weapons shop" => crate::menus::economy::e4_weapons_shop::main(player),
        "7" | "armor shop" => crate::menus::economy::e5_armor_shop::main(player),

        // Profile
        "8" | "inventory" => crate::menus::profile::p1_inventory::main(player),
        "9" | "hall of records" => crate::menus::profile::p2_hall_of_records::main(player),
        "97" | "settings" => crate::menus::profile::n1_settings::main(player),
        "98" | "save game" | "save" => {
            println!("\nSaving game...");
            sleep(2);

            player.save();
            success();

            main(player);
        }
        "99" | "logout" => {
            player.save();

            println!("\nLogging out...");
            sleep(2);

            crate::menus::accounts::main();
        }

        "exit" => {
            exit(Some(player));
        }

        "3.141592" => {
            Settings::toggle_developer(player);
            main(player);
        }

        misc => match misc {
            "96" | "developer" => {
                if player.settings.developer {
                    crate::menus::devmode::d1_developer_menu::main(player);
                } else {
                    messages::invalid_input(Some(misc), None, true);
                    main(player);
                }
            }
            _ => {
                messages::invalid_input(Some(misc), None, true);
                main(player);
            }
        },
    }
}
