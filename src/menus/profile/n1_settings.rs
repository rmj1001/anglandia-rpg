use crate::{
    data::settings::Settings,
    utils::{
        crypt,
        input::{confirm, password, prompt_colon, select_from_str_array},
        messages::*,
        tui::{page_header, HeaderSubtext},
    },
};

use crate::data::player::Player;

pub fn main(player: &mut Player) {
    page_header("Profile Settings", HeaderSubtext::Keyboard);

    let choice: usize = select_from_str_array(
        &[
            "1. Change Username",
            "2. Change Password",
            "3. Lock Profile",
            "4. Delete Profile",
            "5. Toggle Hard Mode",
            "6. View Player Data",
            "NAV: Go Back",
        ],
        None,
    );

    match choice {
        0 => change_username(player),
        1 => change_password(player),
        2 => lock_profile(player),
        3 => delete_profile(player),
        4 => hardmode(player),
        5 => {
            player.view();
            main(player)
        }
        6 => crate::menus::game_menu::main(player),
        _ => out_of_bounds(),
    }
}

fn change_username(player: &mut Player) {
    page_header("Profile Settings", HeaderSubtext::None);
    let new_username = prompt_colon("New Username");

    if new_username == player.settings.username {
        failure("This is your current username.");
        main(player);
    }

    let confirm_username = prompt_colon("Confirm Username");

    if new_username != confirm_username {
        failure("Usernames do not match");
        main(player);
    }

    Settings::change_username(player, new_username);

    page_header("Profile Settings", HeaderSubtext::None);
    success_msg("Username changed.");

    main(player);
}

fn change_password(player: &mut Player) {
    page_header("Profile Settings", HeaderSubtext::Other("Enter new password."));
    let new_password = password(false);
    let new_pass_is_old_pass = crypt::verify_hash(new_password.clone(), player.settings.password_hash.clone());

    if new_pass_is_old_pass {
        failure("This is your current password.");
        main(player);
    }

    let confirm_password = password(true);

    if new_password != confirm_password {
        failure("Passwords do not match.");
        main(player);
    }

    Settings::change_password(player, new_password);

    page_header("Profile Settings", HeaderSubtext::None);
    success_msg("Password changed.");

    main(player);
}

fn lock_profile(player: &mut Player) {
    let confirm_lock = confirm("Are you sure you want to lock your profile?");

    if !confirm_lock {
        cancelling();
        main(player);
    }

    Settings::toggle_lock(player);

    crate::menus::accounts::main();
}

fn delete_profile(player: &mut Player) {
    let confirm_delete = confirm("Are you sure you want to delete your profile?");

    if !confirm_delete {
        cancelling();
        main(player);
    }

    player.delete();

    page_header("Profile Settings", HeaderSubtext::None);
    success_msg("Profile deleted.");

    crate::menus::accounts::main();
}

fn hardmode(player: &mut Player) {
    if !player.settings.hardmode {
        println!("Are you sure you want to enable hardmode?");
        let confirmation = confirm("If you lose a battle, you could have your profile deleted.");

        if !confirmation {
            cancelling();
            main(player);
        }
    } else {
        let confirmation = confirm("Are you sure you want to disable hardmode?");

        if !confirmation {
            cancelling();
            main(player);
        }
    }

    Settings::toggle_hardmode(player);

    main(player);
}
