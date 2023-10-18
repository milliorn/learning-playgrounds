const https = require('https');

const options = { // options object to be passed to https.request()
  method: 'GET', // GET is the default method, so this is optional
  headers: { // headers object to be passed to https.request()
    'Content-Type': 'application/json' // tells the server we want JSON back
  }
};

let request = https.request('https://jsonplaceholder.typicode.com/users?_limit=2', options, (res) => {
  if (res.statusCode !== 200) {
    console.error(`Did not get an OK from the server. Code: ${res.statusCode}`);
    res.resume();
    return;
  }

  let data = '';

  res.on('data', (chunk) => { // called when a data chunk is received.
    data += chunk; // convert Buffer to string
  });

  res.on('close', () => { // called when the complete response is received.
    console.log('Retrieved all data');
    console.log(JSON.parse(data)); // parse the data string to JSON
  });
});

request.end(); // important: always close the request when finished with it

request.on('error', (err) => { // called in case of a malformed request or a network error.
  console.error(`Encountered an error trying to make a request: ${err.message}`);
});