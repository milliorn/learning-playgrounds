const https = require('https');

// GET request with https.get()
let request = https.get('https://jsonplaceholder.typicode.com/users?_limit=2', (res) => { // called when the response is received.
  if (res.statusCode !== 200) {
    console.error(`Did not get an OK from the server. Code: ${res.statusCode}`);
    res.resume(); // consume response data to free up memory
    return;
  }

  let data = ''; // var to store the response chunks

  // called when a data chunk is received.
  res.on('data', (chunk) => {
    data += chunk.toString();
  });

  // called when the complete response is received.
  res.on('close', () => {
    console.log('Retrieved all data');
    console.log(JSON.parse(data));
  });

  request.on('error', (err) => {  // called in case of a malformed request or a network error.
    console.error(`Encountered an error trying to make a request: ${err.message}`);
  });
});

