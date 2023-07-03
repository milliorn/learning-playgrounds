import React from "react";
import styled from "styled-components";
import Person from "./Person";

// Styled component to center the content vertically and horizontally
const CenteredContainer = styled.div`
  align-items: center;
  display: flex;
  height: 2rem; /* 32px */
  justify-content: center;
  margin: 0 auto;
  padding: 1rem 0; /* 16px */
  width: 100%;
`;

const Greeting = () => {
  return (
    <div>
      {/* Container for the greeting */}
      <CenteredContainer>
        <h1>Greeting</h1>
      </CenteredContainer>
      {/* Container for the person component */}
      <CenteredContainer>
        <Person />
      </CenteredContainer>
    </div>
  );
};

export default Greeting;
