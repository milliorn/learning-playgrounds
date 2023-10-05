const fs = require('fs');
const util = require('util');
const { EventEmitter } = require('events'); //importing module

try {
  const data = fs.readFile('node.txt', (err, result) => {
    if (err) {
      console.error(err);
      return;
    }

    console.log(result);
  });

} catch (err) {
  console.log(err)
}

console.log("an important piece of code that should be run at the end")

const udf_double = num => {
  return new Promise((resolve, reject) => {
    setTimeout(() => {
      if (typeof num !== 'number') {
        reject(new TypeError(`Expected number, got: ${typeof num}`));
      }

      const result = num * 2;
      resolve(result);
    }, 100);
  });
}

udf_double(2)
  .then(result => {
    console.log(result);
  })
  .catch(err => {
    console.error(err);
  });

const readFile = util.promisify(fs.readFile);

const read = async () => {
  try {
    const result = await readFile('node.txt');
    console.log(result);
  } catch (err) {
    console.error(err);
  }
};

read()

const getLetter = (index) => {
  let cypher = "*12345K%^*^&*" //will be a fetch function in a real scenario which will fetch a new cypher every time
  let cipher_split = cipher.split('')
  return cipher_split[ index ]
}

const emitterFn = () => {
  const emitter = new EventEmitter(); //initializing new emitter
  let counter = 0;
  const interval = setInterval(() => {
    counter++;

    if (counter === 7) {
      clearInterval(interval);
      emitter.emit('end');
    }

    let letter = getLetter(counter)

    if (isNaN(letter)) { //Check if the received value is a number
      (counter < 7) && emitter.emit(
        'error',
        new Error(`The index ${counter} needs to be a digit`)
      );
      return;
    }
    (counter < 7) && emitter.emit('success', counter);

  }, 1000);

  return emitter;
}

const listner = emitterFn();

listner.on('end', () => {
  console.info('All six indexes have been checked');
});

listner.on('success', (counter) => {
  console.log(`${counter} index is an integer`);
});

listner.on('error', (err) => {
  console.error(err.message);
});