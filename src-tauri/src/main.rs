#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

mod enigma;
use enigma::{EnigmaMachine, Plugboard, Reflector, Rotor};

// 1. State container.
struct AppState {
    enigma: Mutex<EnigmaMachine>,
}

// 2. TAURI command (bridge with frontend)
#[tauri::command]
fn process_keystroke(state: tauri::State<AppState>, key: char) -> Result<char, String> {
    let mut machine = state.enigma.lock().unwrap();

    let input_num = key.to_ascii_uppercase() as u8 - b'A';

    let output_num = machine.process_key(input_num);

    let output_char = (output_num + b'A') as char;

    Ok(output_char)
}

fn main() {
    // Hystorical configuration of the machine
    let plugboard = Plugboard::new();
    let right_rotor = Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", 'V');
    let middle_rotor = Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", 'E');
    let left_rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'Q');
    let reflector = Reflector::new("YRUHQSLDPXNGOKMIEBFZCWVJAT");

    let machine = EnigmaMachine::new(plugboard, right_rotor, middle_rotor, left_rotor, reflector);

    tauri::Builder::default()
        .manage(AppState {
            enigma: Mutex::new(machine),
        })
        .invoke_handler(tauri::generate_handler![process_keystroke])
        .run(tauri::generate_context!())
        .expect("Error during the start of the application!");
}
