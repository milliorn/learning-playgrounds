import logo from "./logo.svg";
import "./App.css";
import Button from "@material-ui/core/Button";
import SaveIcon from "@material-ui/icons/Save";
import { ButtonGroup } from "@material-ui/core";
import DeleteIcon from "@material-ui/icons/Delete";
import Checkbox from "@material-ui/core/Checkbox";
import { useState } from "react";
import FormControlLabel from "@material-ui/core/FormControlLabel";
import TextField from "@material-ui/core/TextField";
import {
  makeStyles,
  ThemeProvider,
  createTheme,
} from "@material-ui/core/styles";
import { green, red } from "@material-ui/core/colors";
import "fontsource-roboto";
import Typography from "@material-ui/core/Typography";
import Container from "@material-ui/core/Container";
import Paper from "@material-ui/core/Paper";
import Grid from "@material-ui/core/Grid";
import AppBar from "@material-ui/core/AppBar";
import Toolbar from "@material-ui/core/Toolbar";
import IconButton from "@material-ui/core/IconButton";
import MenuIcon from "@material-ui/icons/Menu";

const useStyles = makeStyles({
  root: {
    background: `linear-gradient(45deg, #333, #999)`,
    border: 0,
    marginBottom: 16,
    borderRadius: 16,
    color: "white",
    padding: "4px 32px",
  },
});

const theme = createTheme({
  typography: {
    h2: {
      fontSize: 36,
      marginBottom: 16,
    },
  },
  palette: {
    primary: {
      main: green[500],
    },
    secondary: {
      main: red[500],
    },
  },
});

function ButtonStyled() {
  const classes = useStyles();
  return <Button className={classes.root}>Test Styled Button</Button>;
}

function CheckBoxTest() {
  const [checked, setChecked] = useState(true);

  return (
    <FormControlLabel
      control={
        <Checkbox
          checked={checked}
          icon={<DeleteIcon />}
          checkedIcon={<SaveIcon />}
          onChange={(e) => setChecked(e.target.checked)}
          color="primary"
          inputProps={{ "aria-label": "secondary checkbox" }}
        />
      }
      label="Testing Checkbox"
    />
  );
}

function App() {
  return (
    <ThemeProvider theme={theme}>
      <Container maxWidth="lg">
        <div className="App">
          <header className="App-header">
            <AppBar color="secondary">
              <Toolbar>
                <IconButton>
                  <MenuIcon />
                </IconButton>
                <Typography variant="h6">MUI Theme</Typography>
                <Button>Login</Button>
              </Toolbar>
            </AppBar>
            <Typography variant="h1">Hello MUI</Typography>
            <Typography variant="h2" component="div">
              Learn MUI
            </Typography>
            <ButtonStyled />
            <Grid container spacing={4} justifyContent="center">
              <Grid item xs={4} sm={6}>
                <Paper style={{ height: 80, width: 60 }}></Paper>
              </Grid>
              <Grid item>
                <Paper style={{ height: 80, width: 60 }}></Paper>
              </Grid>
              <Grid item>
                <Paper style={{ height: 80, width: 60 }}></Paper>
              </Grid>
            </Grid>
            <TextField
              variant="outlined"
              color="secondary"
              type="email"
              label="Your Email"
              placeholder="email@email.com"
            />
            <CheckBoxTest />
            <ButtonGroup variant="contained" size="large">
              <Button
                startIcon={<SaveIcon />}
                color="primary"
                href="#"
                onClick={() => alert("hello")}
              >
                Save
              </Button>
              <Button
                endIcon={<DeleteIcon />}
                color="secondary"
                href="#"
                onClick={() => alert("bye")}
              >
                Delete
              </Button>
            </ButtonGroup>
            <img src={logo} className="App-logo" alt="logo" />
          </header>
        </div>
      </Container>
    </ThemeProvider>
  );
}

export default App;
