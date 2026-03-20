/// Plugboard (Steckerbrett)
pub struct Plugboard {
    wiring: [u8; 26],
}

impl Plugboard {
    /// Creates an empty plugboard (A->A, B->B, etc.)
    pub fn new() -> Self {
        let mut tmp: [u8; 26] = [0; 26];
        for i in 0..26 {
            tmp[i] = i as u8;
        }
        Self { wiring: tmp }
    }

    /// Adds a swap between two letters (ex. A e B)
    pub fn add_swap(&mut self, a: char, b: char) {
        let a_num = a as u8 - b'A';
        let b_num = b as u8 - b'A';
        self.wiring[a_num as usize] = b_num;
        self.wiring[b_num as usize] = a_num;
    }

    /// Pass the signal inside the plugboard
    pub fn forward(&self, signal: u8) -> u8 {
        self.wiring[signal as usize]
    }
}

/// Rotor (Walze)
#[derive(Clone)]
pub struct Rotor {
    forward_wiring: [u8; 26],
    backward_wiring: [u8; 26],
    position: u8,
    ring_setting: u8,
    notch: u8,
}

impl Rotor {
    /// Builds a new Rotor
    pub fn new(wiring: &str, notch: char) -> Self {
        let mut forward_wiring: [u8; 26] = [0; 26];
        let mut backward_wiring: [u8; 26] = [0; 26];

        for (index, character) in wiring.chars().enumerate() {
            let number: u8 = character as u8 - b'A';
            forward_wiring[index] = number;
            backward_wiring[number as usize] = index as u8;
        }
        let notch_number: u8 = notch as u8 - b'A';
        Self {
            forward_wiring,
            backward_wiring,
            position: 0,
            ring_setting: 0,
            notch: notch_number,
        }
    }

    pub fn set_position(&mut self, pos: char) {
        let position: u8 = pos as u8 - b'A';
        self.position = position;
    }

    pub fn set_ring_setting(&mut self, ring: char) {
        self.ring_setting = ring as u8 - b'A';
    }

    pub fn is_at_notch(&self) -> bool {
        self.notch == self.position
    }

    pub fn step(&mut self) {
        self.position = (self.position + 1) % 26;
    }

    pub fn forward(&self, signal: u8) -> u8 {
        let offset = (self.position + 26 - self.ring_setting) % 26;
        let signal_entry = (signal + offset) % 26;
        let cable_exit = self.forward_wiring[signal_entry as usize];
        let result = (cable_exit + 26 - offset) % 26;
        result
    }

    pub fn backward(&self, signal: u8) -> u8 {
        let offset = (self.position + 26 - self.ring_setting) % 26;
        let signal_entry = (signal + offset) % 26;
        let cable_exit = self.backward_wiring[signal_entry as usize];
        let result = (cable_exit + 26 - offset) % 26;
        result
    }
}

/// Reflector (Umkehrwalze)
pub struct Reflector {
    wiring: [u8; 26],
}

impl Reflector {
    pub fn new(wiring: &str) -> Self {
        let mut tmp: [u8; 26] = [0; 26];

        for (index, character) in wiring.chars().enumerate() {
            let number: u8 = character as u8 - b'A';
            tmp[index] = number;
        }
        Self { wiring: tmp }
    }

    pub fn reflect(&self, signal: u8) -> u8 {
        self.wiring[signal as usize]
    }
}

/// The enigma machine
pub struct EnigmaMachine {
    plugboard: Plugboard,
    right_rotor: Rotor,
    middle_rotor: Rotor,
    left_rotor: Rotor,
    reflector: Reflector,
}

impl EnigmaMachine {
    pub fn new(
        plugboard: Plugboard,
        right_rotor: Rotor,
        middle_rotor: Rotor,
        left_rotor: Rotor,
        reflector: Reflector,
    ) -> Self {
        Self {
            plugboard,
            right_rotor,
            middle_rotor,
            left_rotor,
            reflector,
        }
    }

    /// Private function for advance the rotors.
    fn step_rotors(&mut self) {
        let middle_at_notch = self.middle_rotor.is_at_notch();
        let right_at_notch = self.right_rotor.is_at_notch();

        if middle_at_notch {
            self.left_rotor.step();
            self.middle_rotor.step();
        } else if right_at_notch {
            self.middle_rotor.step();
        }
        self.right_rotor.step();
    }

    /// Main method: Enter a letter and encrypts it.
    pub fn process_key(&mut self, input: u8) -> u8 {
        self.step_rotors();
        let signal = self.plugboard.forward(input);
        let signal = self.right_rotor.forward(signal);
        let signal = self.middle_rotor.forward(signal);
        let signal = self.left_rotor.forward(signal);

        let signal = self.reflector.reflect(signal);

        let signal = self.left_rotor.backward(signal);
        let signal = self.middle_rotor.backward(signal);
        let signal = self.right_rotor.backward(signal);

        self.plugboard.forward(signal)
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    fn setup_historical_enigma() -> EnigmaMachine {
        let plugboard = Plugboard::new();
        let right_rotor = Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", 'V'); // III
        let middle_rotor = Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", 'E'); // II
        let left_rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'Q'); // I
        let reflector = Reflector::new("YRUHQSLDPXNGOKMIEBFZCWVJAT"); // B

        EnigmaMachine::new(plugboard, right_rotor, middle_rotor, left_rotor, reflector)
    }

    #[test]
    fn test_full_enigma_encryption_and_decryption() {
        // Source
        let mut machine_a = setup_historical_enigma();

        // 1. Add the cables
        machine_a.plugboard.add_swap('A', 'Z');
        machine_a.plugboard.add_swap('M', 'Q');
        machine_a.plugboard.add_swap('H', 'T');

        // 2. Ring Setting
        machine_a.left_rotor.set_ring_setting('F');
        machine_a.middle_rotor.set_ring_setting('O');
        machine_a.right_rotor.set_ring_setting('O');

        // 3. Set the initial position (key of the message)
        machine_a.left_rotor.set_position('B');
        machine_a.middle_rotor.set_position('A');
        machine_a.right_rotor.set_position('R');

        // Destination
        let mut machine_b = setup_historical_enigma();

        // Same configuration
        machine_b.plugboard.add_swap('A', 'Z');
        machine_b.plugboard.add_swap('M', 'Q');
        machine_b.plugboard.add_swap('H', 'T');

        machine_b.left_rotor.set_ring_setting('F');
        machine_b.middle_rotor.set_ring_setting('O');
        machine_b.right_rotor.set_ring_setting('O');

        machine_b.left_rotor.set_position('B');
        machine_b.middle_rotor.set_position('A');
        machine_b.right_rotor.set_position('R');

        // --- TEST ---
        // Letter 'A' (number 0)
        let input: u8 = 0;
        let cipher_text = machine_a.process_key(input);

        // 1: Enigma cannot encrypt a letter into itself
        assert_ne!(cipher_text, input, "Error: The letter has not changed!");

        // Decrypt the result with the receiver machine
        let decrypted_text = machine_b.process_key(cipher_text);

        // 2: The result must be the original input
        assert_eq!(
            decrypted_text, input,
            "Errore: Failed symmetry! Obtained {}",
            decrypted_text
        );
    }
}
