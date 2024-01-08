use albion_termrpg::lib::{
    input::*,
    tui::{self, page_header},
    user::profile::ProfileRetrievalResult,
    user::profile::UserProfile,
};

fn get_password(profile: &UserProfile) -> bool {
    let password: String = password();

    if password != profile.password {
        println!("\nIncorrect password.");
        return false;
    }

    true
}

fn profile_remains_locked() {
    println!("\nProfile will remain locked.");
    tui::press_enter_to_continue();
    crate::menus::accounts::main::menu();
}

pub fn menu() {
    page_header("Login", tui::HeaderInstructions::None);

    let username: String = prompt_input("Username");
    let profile_result = UserProfile::retrieve(&username);

    match profile_result {
        ProfileRetrievalResult::Some(profile) => {
            let mut profile = profile;

            if profile.locked {
                let unlock = prompt_input("\nProfile is locked. Unlock? (y/n)").to_lowercase();

                match &unlock[..] {
                    "y" => {
                        if get_password(&profile) {
                            profile.unlock();
                            println!("\nProfile unlocked. Proceed with login.\n");
                        } else {
                            profile_remains_locked()
                        }
                    }
                    "yes" => {
                        if get_password(&profile) {
                            profile.unlock();
                        } else {
                            profile_remains_locked()
                        }
                    }
                    "n" => profile_remains_locked(),
                    "no" => profile_remains_locked(),
                    invalid_input => {
                        tui::invalid_input(Some(invalid_input));
                        tui::press_enter_to_continue();
                        crate::menus::accounts::main::menu();
                    }
                }
            }

            if !get_password(&profile) {
                tui::press_enter_to_continue();
                crate::menus::accounts::main::menu();
            }

            println!("\nLogin successful.");
            tui::press_enter_to_continue();

            crate::menus::game::main::menu(&mut profile);
        }

        ProfileRetrievalResult::None(message) => {
            println!("\n{}", message);
            tui::press_enter_to_continue();
            crate::menus::accounts::main::menu();
        }
    }
}
