// Import necessary dependencies
import React, { useState } from "react";
import styled from "styled-components";

// Styling using styled-components
const Container = styled.div`
  // CSS styles for the container
`;

// Create the ControlledInputs component
const ControlledInputs = () => {
  // State variables for name and email
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");

  // Handle form submission
  const handleSubmit = (e) => {
    e.preventDefault();
    console.log(name, email);
  };

  // Render the component
  return (
    <Container>
      <form className="form" onSubmit={handleSubmit}>
        <h4>controlled inputs</h4>
        {/* Name input field */}
        <div className="form-row">
          <label htmlFor="name">name </label>
          <input
            type="text"
            className="form-input"
            value={name}
            onChange={(e) => setName(e.target.value)}
            id="name"
          />
        </div>
        {/* Email input field */}
        <div>
          <label htmlFor="email">Email </label>
          <input
            type="email"
            id="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
          />
        </div>
        {/* Submit button */}
        <button type="submit">Submit</button>
      </form>
    </Container>
  );
};

// Export the ControlledInputs component
export default ControlledInputs;
