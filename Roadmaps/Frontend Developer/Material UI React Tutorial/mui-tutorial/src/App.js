import logo from "./logo.svg";
import "./App.css";
import { Button } from "@mui/material";
import { Save } from "@mui/icons-material";

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <Button
          variant="contained"
          color="primary"
          href="#"
          onClick={() => alert("hello")}
          size="small"
          style={{
            fontSize: 14,
          }}
        >
          Hello World
        </Button>
        <img src={logo} className="App-logo" alt="logo" />
      </header>
    </div>
  );
}

export default App;
