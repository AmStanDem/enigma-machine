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

/// Rappresenta il singolo Rotore (Walze)
#[derive(Clone)]
pub struct Rotor {
    forward_wiring: [u8; 26],
    backward_wiring: [u8; 26],
    position: u8,
    ring_setting: u8,
    notch: u8,
}

impl Rotor {
    /// Costruisce il rotore partendo dalla stringa storica del cablaggio
    pub fn new(wiring: &str, notch: char) -> Self {
        todo!("Popola forward_wiring e calcola backward_wiring. Inizializza gli altri campi a 0")
    }

    pub fn set_position(&mut self, pos: char) {
        todo!("Imposta la posizione da 0 a 25")
    }

    pub fn set_ring_setting(&mut self, ring: char) {
        todo!("Imposta il ring setting da 0 a 25")
    }

    pub fn is_at_notch(&self) -> bool {
        todo!("Restituisce true se la position è uguale al notch")
    }

    pub fn step(&mut self) {
        todo!("Incrementa la posizione di 1 (modulo 26)")
    }

    pub fn forward(&self, signal: u8) -> u8 {
        todo!("Applica la formula (signal + pos - ring), passa per forward_wiring, e rimuovi l'offset")
    }

    pub fn backward(&self, signal: u8) -> u8 {
        todo!("Come forward, ma usa backward_wiring")
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
