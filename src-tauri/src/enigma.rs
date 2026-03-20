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
    pub fn add_swap(&mut self, a: u8, b: u8) {
        let index_a = a as usize;
        let index_b = b as usize;

        self.wiring.swap(index_a, index_b);
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

/// La Macchina Completa
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

    /// Funzione privata per gestire l'avanzamento dei rotori
    fn step_rotors(&mut self) {
        todo!(
            "Implementa la logica di stepping (ricordandoti del double-stepping del middle_rotor!)"
        )
    }

    /// Il metodo principale: premi un tasto (0-25) e ottieni il risultato criptato
    pub fn process_key(&mut self, input: u8) -> u8 {
        todo!(
            "1. Chiama step_rotors()
             2. Passa per la plugboard
             3. Passa per i 3 rotori in forward (right -> middle -> left)
             4. Passa per il reflector
             5. Passa per i 3 rotori in backward (left -> middle -> right)
             6. Passa di nuovo per la plugboard e restituisci il risultato"
        )
    }
}
