import "./App.css";
import PixiDisplay from "./components/Canvas/PixiDisplay";
import { ThemeProvider } from "./contexts/ThemeContext";

function App() {
  return (
    <ThemeProvider>
      <PixiDisplay />
    </ThemeProvider>
  );
}

export default App;
