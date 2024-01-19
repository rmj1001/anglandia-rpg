use crate::lib::tui::press_enter_to_continue;

/// input: The invalid input
///
/// Parameters:
///
/// - expected: The expected input
///
/// - pause: Ask the user to press enter to continue?
pub fn invalid_input(input: Option<&str>, expected: Option<&str>, pause: bool) {
    let mut output_string = String::new();

    match input {
        Some(text) => output_string.push_str(&format!("\nInvalid input '{}'.", text)),
        None => output_string.push_str("\nInvalid input."),
    }

    if let Some(text) = expected {
        output_string.push_str(&format!(" Expected '{}'.", text));
    }

    println!("{}", output_string);

    if pause {
        press_enter_to_continue();
    }
}

pub fn cancelling() {
    println!("\nCancelling.");
    press_enter_to_continue();
}

pub fn success() {
    println!("\nSuccess!");
    press_enter_to_continue();
}

pub fn failure<T>(message: T)
where
    T: Into<String>,
{
    eprintln!("\nFailure: {}", message.into());
    press_enter_to_continue();
}

/// Standard panic message for dialogue selector
pub fn out_of_bounds<T>(optional_error: Option<T>)
where
    T: Into<String>,
{
    match optional_error {
        Some(error) => panic!(
            "\nDialogue selected index out of option's bounds: {}",
            error.into()
        ),
        None => panic!("\nDialogue selected index out of option's bounds."),
    }
}
