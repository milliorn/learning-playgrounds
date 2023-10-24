import mongoose from "mongoose";
import dotenv from 'dotenv';
import Blog from "./model/Blog.js";
import User from "./model/User.js";

dotenv.config(); // Load environment variables from .env file

// Connect to MongoDB using environment variables
const username = process.env.DB_USERNAME;
const password = process.env.DB_PASSWORD;
const database = process.env.DB_DATABASE;
const connectionString = `mongodb+srv://${username}:${password}@${database}`;

// Connect to MongoDB using the connection string
mongoose.connect(connectionString, { useNewUrlParser: true, useUnifiedTopology: true })
  .then(() => console.log("MongoDB Connected"))
  .catch(err => console.log(err));

// Create a new blog post
let article = new Blog({
  title: 'Awesome Post!',
  slug: 'awesome-post',
  published: true,
  content: 'This is the best post ever',
  tags: [ 'featured', 'announcement' ],
});

// Insert the article in our MongoDB database
await article.save();

// Find all blog posts
article = await Blog.find();
console.log(article);

// Create a new blog post and insert into database
const article2 = await Blog.create({
  title: 'Awesome Post!',
  slug: 'awesome-post',
  published: true,
  content: 'This is the best post ever',
  tags: [ 'featured', 'announcement' ],
});

console.log(article2);

// Update data
article2.title = "The Most Awesomest Post!!";
await article2.save();
console.log(article);

// Finding data
article = await Blog.findById("65374b23b70e33dd2cd61e78").exec();
console.log(article);

// Finding data with projection
article = await Blog.findById("62472b6ce09e8b77266d6b1b", "title slug content").exec();
console.log(article);

// Deleting data
const blog = await Blog.deleteOne({ author: "Jesse Hall" })
console.log(blog)

const blogs = await Blog.deleteMany({ author: "Jesse Hall" })
console.log(blogs)

const blog1 = await Blog.exists({ author: "Jesse Hall" })
console.log(blog1)

const blogFind = await Blog.findOne({ author: "Jesse Hall" }) // returns the first document that matches the query

const blogWhere = await Blog.where("author").equals("Jesse Hall").exec() // returns all documents that match the query

const blog2 = await Blog.where("author").equals("Jesse Hall").select("title author")
console.log(blog2)

const articles = await Blog.create({
  title: 'Awesome Post!',
  slug: 'Awesome-Post',
  author: User._id,
  content: 'This is the best post ever',
  tags: [ 'featured', 'announcement' ],
});

console.log(articles);

const article3 = await Blog.findOne({ title: "Awesome Post!" }).populate("author");
console.log(article3);
