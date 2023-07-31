/**
 * @param {Object} context
 * @param {any[]} args
 * @return {any}
 */
Function.prototype.callPolyfill = function (context, ...args) {
  const key = Symbol(); // Create a unique key as a property name
  context[key] = this; // Assign the function to the context object using the unique key
  const result = context[key](...args); // Call the function with the provided arguments
  delete context[key]; // Remove the function from the context object
  return result; // Return the result of the function call
};
