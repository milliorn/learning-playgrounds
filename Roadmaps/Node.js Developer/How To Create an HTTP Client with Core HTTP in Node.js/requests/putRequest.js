const https = require('https');

const options = { // options object with the request URL and method
  host: 'jsonplaceholder.typicode.com', // URL to work with
  path: '/users/1', // endpoint to work with
  method: 'PUT', // HTTP method
  headers: { // headers object
    'Accept': 'application/json', // type to accept
    'Content-Type': 'application/json; charset=UTF-8' // type to send
  }
};

// Create a request object with the callback with the response
const request = https.request(options, (res) => { // called when a response is received from the server.
  if (res.statusCode !== 200) { // 200 is the HTTP status code for successful responses
    console.error(`Did not get an OK from the server. Code: ${res.statusCode}`); // log the error to the console
    res.resume(); // free up the connection
    return;
  }

  let data = ''; // string to store the response data in

  res.on('data', (chunk) => { // called when a data chunk is received.
    data += chunk; // convert Buffer to string
  });

  res.on('close', () => { // called when the complete response is received.
    console.log('Updated data');
    console.log(JSON.parse(data)); // parse the data string to JSON
  });
});

const requestData = { // JSON data to send
  username: 'digitalocean' // new username
};

request.write(JSON.stringify(requestData)); // write the JSON data to the request

request.end(); // Close the connection

request.on('error', (err) => { // called when an error occurs
  console.error(`Encountered an error trying to make a request: ${err.message}`);
});