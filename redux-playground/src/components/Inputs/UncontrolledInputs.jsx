import React, { useState } from "react";

// UncontrolledInputs component
const UncontrolledInputs = () => {
  // State variable for value
  const [value, setValue] = useState(0);

  // Handle form submission
  const handleSubmit = (e) => {
    e.preventDefault();

    // Create a new FormData object from the form
    const formData = new FormData(e.currentTarget);

    // Get values one by one
    const name = formData.get("name");
    console.log(name);

    // Get all form values as an object
    const newUser = Object.fromEntries(formData);
    console.log(newUser);

    // Gotcha: Re-rendering won't clear out the form values
    setValue(value + 1);

    // Reset form values
    e.currentTarget.reset();
  };

  // Render the component
  return (
    <div>
      <form className="form" onSubmit={handleSubmit}>
        <h4>Form Data API</h4>
        {/* Name input */}
        <div className="form-row">
          <label htmlFor="name" className="form-label">
            Name
          </label>
          <input type="text" className="form-input" id="name" name="name" />
        </div>
        {/* Email input */}
        <div className="form-row">
          <label htmlFor="email" className="form-label">
            Email
          </label>
          <input type="email" className="form-input" id="email" name="email" />
        </div>
        {/* Password input */}
        <div className="form-row">
          <label htmlFor="password" className="form-label">
            Password
          </label>
          <input
            type="password"
            className="form-input"
            id="password"
            name="password"
          />
        </div>

        <button type="submit" className="btn btn-block">
          Submit
        </button>
      </form>
    </div>
  );
};

export default UncontrolledInputs;
