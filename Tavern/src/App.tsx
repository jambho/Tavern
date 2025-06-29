import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import GameBoard from "./components/Canvas/GameBoard";
import { ThemeProvider } from "./contexts/ThemeContext";

function App() {
  return (
    <ThemeProvider>
      <GameBoard />
    </ThemeProvider>
  );
}

export default App;
