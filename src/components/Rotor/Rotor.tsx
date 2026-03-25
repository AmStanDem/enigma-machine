import React from 'react';
import './Rotor.css'; // Creeremo questo file subito dopo

interface RotorProps {
  position: number; // da 0 a 25
}

// Generiamo un array con l'alfabeto ['A', 'B', 'C', ..., 'Z']
const ALPHABET = Array.from({ length: 26 }, (_, i) => String.fromCharCode(65 + i));

export const Rotor: React.FC<RotorProps> = ({ position }) => {
  // Se ogni lettera è alta 40px, spostiamo il nastro in alto di (posizione * 40)
  const LETTER_HEIGHT = 40; 
  const translateY = -(position * LETTER_HEIGHT);

  return (
    <div className="rotor-window">
      <div 
        className="rotor-strip" 
        style={{ transform: `translateY(${translateY}px)` }}
      >
        {ALPHABET.map((letter, index) => (
          <div key={index} className="rotor-letter">
            {letter}
          </div>
        ))}
      </div>
    </div>
  );
};