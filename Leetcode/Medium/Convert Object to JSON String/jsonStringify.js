function jsonStringify(object) {
  // is a string
  if (typeof object === "string") {
    return '"' + object + '"';
  }

  // is a number, boolean, or null
  else if (
    typeof object === "number" ||
    typeof object === "boolean" ||
    object === null
  ) {
    return String(object);
  }

  // is an array
  else if (Array.isArray(object)) {
    // Convert each element in the array to its JSON string representation
    var elements = object.map((item) => jsonStringify(item));
    // Join the elements with commas and enclose in square brackets to form the JSON array
    return "[" + elements.join(",") + "]";
  }

  // is an object (excluding arrays and null)
  else if (typeof object === "object") {
    var pairs = [];
    // Iterate over each key in the object
    Object.keys(object).forEach((key) => {
      // Convert the key and value to their JSON string representations
      var keyString = jsonStringify(key);
      var valueString = jsonStringify(object[key]);
      // Concatenate the key-value pair with a colon and add to the pairs array
      pairs.push(keyString + ":" + valueString);
    });
    // Join the pairs with commas and enclose in curly braces to form the JSON object
    return "{" + pairs.join(",") + "}";
  }

  // Return an empty string for unsupported values
  else {
    return "";
  }
}
