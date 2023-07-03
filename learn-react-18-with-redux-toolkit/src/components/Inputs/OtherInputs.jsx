import React, { useState } from "react";

// Array of frameworks
const frameworks = ["react", "angular", "vue", "svelte"];

// OtherInputs component
const OtherInputs = () => {
  // State variables for shipping and framework
  const [shipping, setShipping] = useState(false);
  const [framework, setFramework] = useState("react");

  // Handle shipping checkbox change
  const handleShipping = (e) => {
    console.log(e.target.checked);
    // Update the shipping state with the checkbox value
    setShipping(e.target.checked);
  };

  // Handle framework select change
  const handleFramework = (e) => {
    // Update the framework state with the selected option value
    setFramework(e.target.value);
  };

  // Render the component
  return (
    <div>
      <form className="form">
        <h4>Other Inputs</h4>
        {/* Shipping checkbox */}
        <div className="form-row" style={{ textAlign: "left" }}>
          <label htmlFor="shipping"> Free Shipping </label>
          <input
            type="checkbox"
            checked={shipping}
            id="shipping"
            name="shipping"
            onChange={handleShipping}
          />
        </div>
        {/* Framework select */}
        <div className="form-row" style={{ textAlign: "left" }}>
          <label htmlFor="framework" className="form-label">
            Framework{" "}
          </label>
          <select
            name="framework"
            id="framework"
            value={framework}
            onChange={handleFramework}
          >
            {/* Generate options based on frameworks array */}
            {frameworks.map((framework) => {
              return <option key={framework}>{framework}</option>;
            })}
          </select>
        </div>
        {/* Submit button */}
        <button type="submit" className="btn btn-block">
          submit
        </button>
      </form>
    </div>
  );
};

export default OtherInputs;
