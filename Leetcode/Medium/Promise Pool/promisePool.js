function promisePool(functions, n) {
  return new Promise((resolve) => {
    // Check if the input array of functions is empty
    if (functions.length === 0) {
      // If empty, resolve the promise immediately with an empty array and a value of 0
      resolve([[], 0]);
      return;
    }

    // Determine the initial pool size based on the minimum of functions.length and n
    let poolSize = Math.min(functions.length, n);
    let currentIndex = 0;
    const results = [];

    // Recursive function to execute the functions
    const executeFunction = async (index) => {
      // Execute the function at the current index
      const result = await functions[index]();
      // Store the result in the corresponding index of the results array
      results[index] = result;

      // Check if there are more functions to execute
      if (currentIndex < functions.length) {
        // If yes, execute the next function and increment the current index
        executeFunction(currentIndex++);
      } else {
        // If no, decrement the poolSize as the current function has resolved
        poolSize--;
        // Check if all functions have resolved
        if (poolSize === 0) {
          // If yes, resolve the promise with the results array and the last resolved result
          resolve([results, result]);
        }
      }
    };

    // Start executing the functions in the pool
    for (let i = 0; i < poolSize; i++) {
      executeFunction(currentIndex++);
    }
  });
}
