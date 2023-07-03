import React from "react";

const Events = () => {
  // Event handler for input change
  const handleFormInput = ({ target }) => {
    console.log(`Input Name: ${target.name}`);
    console.log(`Input Value: ${target.value}`);
  };

  // Event handler for button click
  const handleButtonClick = () => {
    alert("handle button click");
  };

  // Event handler for form submission
  const handleFormSubmission = (e) => {
    e.preventDefault();
    console.log("Form Submitted");
  };

  return (
    <section>
      {/* Form with onSubmit event handler */}
      <form onSubmit={handleFormSubmission}>
        <h2>Typical Form</h2>
        {/* Input field with associated label */}
        <label>
          Example:{" "}
          <input
            type="text"
            name="example"
            onChange={handleFormInput}
            className="form-input"
          />
        </label>{" "}
        {/* Button for form submission */}
        <button type="submit" className="submit-button">
          Submit Form
        </button>
      </form>
      {/* Button with onClick event handler */}
      <button onClick={handleButtonClick} className="click-button">
        Click Me
      </button>
    </section>
  );
};

export default Events;
