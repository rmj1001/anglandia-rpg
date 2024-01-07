// TODO: Remove this once the functions below are implemented.
#![allow(unused_variables)]

use albion_termrpg::lib::{
    input::{self, prompt_input, select_from_vector, selector},
    tui::{self, page_header, press_enter_to_continue},
    user::{
        bank::{Bank, BankAccount, BankResult},
        profile::{ProfileRetrievalResult, UserProfile},
    },
};

fn manage_user_profiles(user: &mut UserProfile) {
    page_header(
        "Developer Mode - Profile Management",
        Some("Use ↑ ↓ keys to select an option below, then press ENTER/RETURN to run it"),
    );

    // Listing profiles for printing or deletion
    let profiles: Vec<String> = UserProfile::list_all();

    let choice1 = selector(
        &[
            "1. List Users",
            "2. Delete User",
            "3. View User File",
            "NAV: Go Back",
        ],
        0,
        Some(""),
    );

    match choice1 {
        // listing profiles
        0 => {
            page_header("Developer Mode - Profile Management", None);

            for profile_string in &profiles {
                println!("- {}", profile_string);
            }

            println!();
            tui::press_enter_to_continue();

            manage_user_profiles(user);
        }

        // deleting profiles
        1 => {
            page_header(
                "Developer Mode - Profile Management",
                Some("Use ↑ ↓ keys to select an option below, then press ENTER/RETURN to run it"),
            );

            let choice =
                select_from_vector(profiles.clone(), 0, Some("Select a profile to delete"));

            let profile_choice = profiles.get(choice);

            match profile_choice {
                Some(profile_string) => {
                    match &prompt_input(&format!(
                        "Are you sure you want to delete profile '{}'? (y/n)",
                        profile_string
                    ))
                    .to_lowercase()[..]
                    {
                        "n" => {
                            println!("\nAborting.");
                            tui::press_enter_to_continue();

                            manage_user_profiles(user);
                        }
                        "no" => {
                            println!("\nAborting.");
                            tui::press_enter_to_continue();

                            manage_user_profiles(user);
                        }
                        "y" => {}
                        "yes" => {}

                        invalid_input => {
                            println!("\nInvalid input. Aborting.");
                            tui::press_enter_to_continue();

                            manage_user_profiles(user);
                        }
                    }

                    if *profile_string == user.username {
                        UserProfile::delete_from_username(&user.username);

                        page_header("Developer Mode - Profile Management", None);
                        println!("\nCurrent profile successfully deleted. Logging out.");
                        tui::press_enter_to_continue();

                        crate::menus::accounts::main::menu();
                    }

                    UserProfile::delete_from_username(profile_string);

                    page_header("Developer Mode - Profile Management", None);
                    println!("\nProfile '{}' successfully deleted.", profile_string);
                    tui::press_enter_to_continue();

                    manage_user_profiles(user);
                }
                None => panic!("Dialoguer picked vec index out of bounds"),
            }
        }

        2 => view_user(user),

        3 => main(user),

        _ => panic!("Dialoguer picked option out of bounds"),
    }
}

fn view_user(user: &mut UserProfile) {
    page_header("Developer Mode - User Data Viewer", None);
    let choice = select_from_vector(UserProfile::list_all(), 0, Some("Select a user to view"));

    let profiles = UserProfile::list_all();
    let profile_choice = profiles.get(choice);

    match profile_choice {
        Some(profile_string) => {
            let profile_result = UserProfile::retrieve(profile_string);

            match profile_result {
                ProfileRetrievalResult::Some(profile) => {
                    let json_string = profile.to_pretty_json();

                    page_header(&format!("User Profile - {}", profile.username), None);

                    println!("{}\n", json_string);

                    press_enter_to_continue();
                    manage_user_profiles(user);
                }
                ProfileRetrievalResult::None(message) => {
                    println!("\n{}", message);
                    press_enter_to_continue();

                    manage_user_profiles(user);
                }
            }
        }
        None => panic!("Dialoguer picked option out of bounds."),
    }

    manage_user_profiles(user);
}

fn manipulate_banks(user: &mut UserProfile) {
    let mut account: BankAccount = BankAccount::Account1;

    page_header("Developer Mode - Bank Management", None);
    println!("Coin Purse: {} Gold", user.gold);
    println!();
    println!("Account 1: {} Gold", user.bank.account1);
    println!("Account 2: {} Gold", user.bank.account2);
    println!("Account 3: {} Gold", user.bank.account3);
    println!("Account 4: {} Gold\n", user.bank.account4);

    let account_choice = selector(
        &[
            "Coin Purse",
            "Account 1",
            "Account 2",
            "Account 3",
            "Account 4",
            "NAV: Go Back",
        ],
        0,
        None,
    );

    match account_choice {
        0 => account = BankAccount::CoinPurse,
        1 => account = BankAccount::Account1,
        2 => account = BankAccount::Account2,
        3 => account = BankAccount::Account3,
        4 => account = BankAccount::Account4,
        5 => main(user),
        _ => panic!("Dialoguer selected vector index out of bounds."),
    }

    let option = selector(&["Add Money", "Subtract Money", "NAV: Cancel"], 0, None);

    if option == 2 {
        main(user);
    }

    let amount_result = input::prompt_input("Amount").parse::<u32>();
    let mut amount: u32 = 0;

    match amount_result {
        Ok(number) => amount = number,
        Err(_) => {
            println!("Invalid input. Cancelling.");
            press_enter_to_continue();
            manipulate_banks(user);
        }
    }

    let mut bank_result: BankResult = BankResult::Error("Uninitialized");

    match option {
        // Deposit
        0 => bank_result = Bank::deposit(user, account, amount, true),
        // Withdrawal
        1 => bank_result = Bank::withdraw(user, account, amount, true),
        2 => manipulate_banks(user),
        _ => panic!("Dialoguer selected vector index out of bounds."),
    }

    match bank_result {
        BankResult::Ok => {
            println!("\nOperation successful.");
            press_enter_to_continue();
            manipulate_banks(user);
        }

        BankResult::Error(message) => {
            println!("\n{}", message);
            press_enter_to_continue();
            manipulate_banks(user);
        }
    }
}

// TODO: XP Manipulation
fn manipulate_xp(user: &mut UserProfile) {}

// TODO: Inventory Manipulation
fn manipulate_inventory(user: &mut UserProfile) {}

pub fn main(user: &mut UserProfile) {
    page_header(
        "Developer Settings",
        Some("Use ↑ ↓ keys to select an option below, then press ENTER/RETURN to run it"),
    );

    let choice = selector(
        &[
            "1. Throw a panic",
            "2. Manipulate Inventory",
            "3. Manipulate XP",
            "4. Manipulate Banks",
            "5. Manage User Profiles",
            "NAV: Go Back",
        ],
        0,
        Some(""),
    );

    match choice {
        0 => panic!("This is a panic!"),
        1 => manipulate_inventory(user),
        2 => manipulate_xp(user),
        3 => manipulate_banks(user),
        4 => manage_user_profiles(user),
        5 => crate::menus::game::main::menu(user),
        _ => panic!("Dialogue picked option out of bounds"),
    }
}
