const https = require('https');

// options object with the request URL and method
const options = {
  host: 'jsonplaceholder.typicode.com', // URL to work with
  path: '/users/1', // endpoint to work with
  method: 'DELETE', // HTTP method
  headers: { // headers object
    'Accept': 'application/json', // type to accept
  }
};

// Create a request object with the callback with the response
const request = https.request(options, (res) => { // called when a response is received from the server.
  if (res.statusCode !== 200) { // 200 is the HTTP status code for successful responses
    console.error(`Did not get an OK from the server. Code: ${res.statusCode}`); // log the error to the console
    res.resume(); // free up the connection
    return; // return early to prevent the rest of the function from running
  }

  let data = ''; // string to store the response data in

  res.on('data', (chunk) => { // called when a data chunk is received.
    data += chunk; // convert Buffer to string
  });

  res.on('close', () => { // called when the complete response is received.
    console.log('Deleted user'); // log the data to the console
    console.log(JSON.parse(data)); // parse the data string to JSON
  });
});

request.end(); // Close the connection

request.on('error', (err) => { // called when an error occurs
  console.error(`Encountered an error trying to make a request: ${err.message}`); // log the error to the console
});