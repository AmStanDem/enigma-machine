import { useState } from "react";
import { invoke } from "@tauri-apps/api/core"; 
import { Rotor } from "./components/Rotor/Rotor";

interface EnigmaResponse {
  letter: string;
  positions: [number, number, number];
}


function App() {
  const [inputText, setInputText] = useState("");
  const [outputText, setOutputText] = useState("");
  const [litLetter, setLitLetter] = useState<string>("");
  const [rotorPositions, setRotorPositions] = useState<[number, number, number]>([0, 0, 0]);

  const alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".split("");

  const pressKey = async (letter: string) => {
    try {
      const response = await invoke<EnigmaResponse>("process_keystroke", {
        key: letter,
      });

      const encryptedLetter = response.letter;
      const newPositions = response.positions;

      setInputText((prev) => prev + letter);
      setOutputText((prev) => prev + encryptedLetter);

      setLitLetter(encryptedLetter);
      setTimeout(() => setLitLetter(""), 500);

      console.log("Positions: " + newPositions);

      setRotorPositions(newPositions);

    } catch (error) {
      // TODO Better error handling.
      console.error("Error inside the enigma machine:", error);
    }
  };

  return (
    <div style={{ textAlign: "center", padding: "2rem", fontFamily: "monospace", backgroundColor: "#1a1a1a", color: "#fff", minHeight: "100vh" }}>
      <h1>Enigma machine</h1>

      {/*rotors */}
      <div style={{ display: "flex", justifyContent: "center", gap: "20px", marginBottom: "2rem" }}>
        <Rotor position={rotorPositions[0]} />
        <Rotor position={rotorPositions[1]} />
        <Rotor position={rotorPositions[2]} />
      </div>

      {/* text ribbons*/}
      <div style={{ marginBottom: "3rem", fontSize: "1.5rem" }}>
        <p style={{ color: "#aaa" }}><strong>Input:</strong> {inputText}</p>
        <p style={{ color: "#4caf50" }}><strong>Output:</strong> {outputText}</p>
      </div>

      {/*TODO refactor in the Lightboard component */}
      {/* Lightboard (bulb that can be on/off) */}
      <div style={{ marginBottom: "2rem", display: "flex", flexWrap: "wrap", justifyContent: "center", gap: "10px", maxWidth: "600px", margin: "0 auto" }}>
        {alphabet.map((char) => (
          <div
            key={`lamp-${char}`}
            style={{
              width: "45px",
              height: "45px",
              borderRadius: "50%",
              backgroundColor: litLetter === char ? "#ffcc00" : "#333",
              color: litLetter === char ? "#000" : "#666",
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              fontWeight: "bold",
              fontSize: "1.2rem",
              boxShadow: litLetter === char ? "0 0 15px #ffcc00" : "inset 0 0 5px #000",
              transition: "all 0.1s ease-in-out"
            }}
          >
            {char}
          </div>
        ))}
      </div>

      <hr style={{ borderColor: "#333", margin: "2rem 0" }} />

      {/* TODO Keyboard component */}
      <div style={{ display: "flex", flexWrap: "wrap", justifyContent: "center", gap: "10px", maxWidth: "600px", margin: "0 auto" }}>
        {alphabet.map((char) => (
          <button
            key={`btn-${char}`}
            onClick={() => pressKey(char)}
            style={{
              width: "45px",
              height: "45px",
              fontSize: "1.2rem",
              cursor: "pointer",
              borderRadius: "5px",
              backgroundColor: "#e0e0e0",
              border: "2px solid #999",
              color: "#000",
              fontWeight: "bold"
            }}
          >
            {char}
          </button>
        ))}
      </div>
    </div>
  );
}

export default App;
