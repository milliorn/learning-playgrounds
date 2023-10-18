const https = require('https');

const options = {
  host: 'jsonplaceholder.typicode.com',
  path: '/users',
  method: 'POST',
  headers: {
    'Accept': 'application/json',
    'Content-Type': 'application/json; charset=UTF-8'
  }
};

// Create a request object with the callback with the response
const request = https.request(options, (response) => { // called when a response is received from the server.
  if (response.statusCode !== 201) { // 201 is the HTTP status code for an item being created
    console.error(`Did not get a Created from the server. Code: ${response.statusCode}`);
    response.resume(); // free up the connection
    return;
  }

  let data = ''; // string to store the response data in

  response.on('data', (chunk) => { // called when a data chunk is received.
    data += chunk; // convert Buffer to string
  });

  response.on('close', () => { // called when the complete response is received.
    console.log('Added new user');
    console.log(JSON.parse(data)); // parse the data string to JSON
  });

});

// Create the JSON data to send
const requestData = {
  name: 'New User',
  username: 'digitalocean',
  email: 'user@digitalocean.com',
  address: {
    street: 'North Pole',
    city: 'Murmansk',
    zipcode: '12345-6789',
  },
  phone: '555-1212',
  website: 'digitalocean.com',
  company: {
    name: 'DigitalOcean',
    catchPhrase: 'Welcome to the developer cloud',
    bs: 'cloud scale security'
  }
};

request.write(JSON.stringify(requestData)); // write the JSON data to the request

request.end(); // Close the connection

request.on('error', (err) => { // called when an error occurs
  console.error(`Encountered an error trying to make a request: ${err.message}`);
});