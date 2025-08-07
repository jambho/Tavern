import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import GameBoard from "./components/Canvas/GameBoard";
import DatabaseDisplay from "./components/Canvas/DatabaseDisplay";
import TokenCreator from "./components/Character/TokenCreator";
import { ThemeProvider } from "./contexts/ThemeContext";

function App() {
  return (
    <ThemeProvider>
      <GameBoard />
      <TokenCreator />
      <DatabaseDisplay />
    </ThemeProvider>
  );
}

export default App;
