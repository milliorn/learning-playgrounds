// Import necessary dependencies
import React, { useState } from "react";
import styled from "styled-components";

// Styling using styled-components
const Container = styled.div`
  // CSS styles for the container
`;

// Create the MultipleInputs component
const MultipleInputs = () => {
  // State variable for user data
  const [user, setUser] = useState({
    name: "",
    email: "",
    password: "",
  });

  // Handle input change
  const handleChange = (e) => {
    console.log(e.target.name);
    // Update the corresponding user data field with the new value
    setUser({ ...user, [e.target.name]: e.target.value });
  };

  // Handle form submission
  const handleSubmit = () => {};

  // Render the component
  return (
    <Container>
      <form className="form" onSubmit={handleSubmit}>
        <h4>Multiple Inputs</h4>
        {/* Name input field */}
        <div className="form-row">
          <label htmlFor="name" className="form-label">
            name{" "}
          </label>
          <input
            type="text"
            className="form-input"
            id="name"
            name="name"
            value={user.name}
            onChange={handleChange}
          />
        </div>
        {/* Email input field */}
        <div className="form-row">
          <label htmlFor="email" className="form-label">
            Email{" "}
          </label>
          <input
            type="email"
            className="form-input"
            id="email"
            name="email"
            value={user.email}
            onChange={handleChange}
          />
        </div>
        {/* Password input field */}
        <div className="form-row">
          <label htmlFor="password" className="form-label">
            Password{" "}
          </label>
          <input
            type="password"
            className="form-input"
            id="password"
            name="password"
            value={user.password}
            onChange={handleChange}
          />
        </div>

        {/* Submit button */}
        <button type="submit" className="btn btn-block">
          Submit
        </button>
      </form>
    </Container>
  );
};

// Export the MultipleInputs component
export default MultipleInputs;
