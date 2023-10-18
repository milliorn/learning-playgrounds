// The first thing we need to do is import got package at the top.
import got from 'got';

// Now we can use the got package to make a GET request to the JSONPlaceholder API.
const getData = async () => {
  try {
    const res = await got
      .get('https://jsonplaceholder.typicode.com/posts/1')
      .json();
    console.log(res);
  } catch (error) {
    console.log(error);
  }
};

getData(); 