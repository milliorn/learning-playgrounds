import React from 'react';
import ReactDOM from 'react-dom/client';
import styled, { createGlobalStyle } from "styled-components";
import App from './App';

// GlobalStyle component
const GlobalStyle = createGlobalStyle`
  html, body, #root {
    height: 100%; // Set the height of the root element to 100% of the viewport
    margin: 0; // Remove margin
    padding: 0; // Remove padding
    box-sizing: border-box; // Use border-box box-sizing model
  }
`;

// StyledDiv styled component
const StyledDiv = styled.div`
  background-color: #282c34; // Background color
  padding: 0; // Remove padding
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen',
    'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue',
    sans-serif; // Set the font family
  -webkit-font-smoothing: antialiased; // Improve font rendering on webkit browsers
  -moz-osx-font-smoothing: grayscale; // Improve font rendering on macOS
  min-height: 100vh; // Set the minimum height of the div to 100% of the viewport height
`;

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
  <React.StrictMode>
    <GlobalStyle /> {/* Apply global styles */}
    <StyledDiv> {/* Render the StyledDiv component */}
      <App /> {/* Render the App component */}
    </StyledDiv>
  </React.StrictMode>
);
