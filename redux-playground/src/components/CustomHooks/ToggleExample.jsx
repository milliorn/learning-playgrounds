import React, { useState } from "react";
import styled from "styled-components";

const Container = styled.div`
  border-bottom: 1px solid #ccc;
  border-top: 1px solid #ccc;
  margin: 2rem 0;
  padding: 2rem 0;
  text-transform: capitalize;
`;

const ToggleExample = () => {
  const [show, setShow] = useState(false);
  return (
    <Container>
      <h4>toggle custom hook</h4>
      <button className="btn" onClick={() => setShow(!show)}>
        Toggle
      </button>
      {show && <h4>some stuff</h4>}
    </Container>
  );
};
export default ToggleExample;
