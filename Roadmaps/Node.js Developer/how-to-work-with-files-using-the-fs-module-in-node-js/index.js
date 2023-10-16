const fs = require('fs').promises;

async function readFile(filePath) {
  try {
    const data = await fs.readFile(filePath, 'utf8');
    console.log(data);
  } catch (error) {
    console.error(`Got an error trying to read the file: ${error.message}`);
  }
}


async function openFile() {
  try {
    const csvHeaders = 'name,species,area';
    await fs.writeFile('animals.csv', csvHeaders);
  } catch (error) {
    console.error(`Got an error trying to write to a file: ${error.message}`);
  }
}


async function addGroceryItem(name, quantity, price) {
  try {
    const csvLine = `${name},${quantity},${price}`;
    await fs.writeFile('groceries.csv', csvLine, { flag: 'a' });
  } catch (error) {
    console.error(`Got an error trying to write to a file: ${error.message}`);
  }
}


async function deleteFile(filePath) {
  try {
    await fs.unlink(filePath);
    console.log(`Deleted ${filePath}`);
  } catch (error) {
    console.log(`Got an error trying to delete the file: ${error.message}`);
  }
}

async function moveFile(source, destination) {
  try {
    await fs.rename(source, destination);
    console.log(`Moved ${source} -> ${destination}`);
  } catch (error) {
    console.log(`Got an error trying to move the file: ${error.message}`);
  }
}

readFile('greetings.txt');
openFile();
addGroceryItem('eggs', 12, 1.5);
deleteFile('groceries.csv');
moveFile('animals.csv', 'zoo.csv');