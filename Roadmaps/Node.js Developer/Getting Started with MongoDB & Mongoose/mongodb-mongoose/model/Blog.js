import mongoose from "mongoose";
const { Schema, model } = mongoose;

const blogSchema = new Schema({
  title: String,
  slug: String,
  published: Boolean,
  author: String,
  content: String,
  tags: [ String ],
  createdAt: Date,
  updatedAt: Date,
  comments: [ {
    user: String,
    content: String,
    votes: Number
  } ]
});

// Validation
/*
const blogSchema = new Schema({
  title:  {
    type: String,
    required: true,
  },
  slug:  {
    type: String,
    required: true,
    lowercase: true,
  },
  published: {
    type: Boolean,
    default: false,
  },
  author: {
    type: String,
    required: true,
  },
  content: String,
  tags: [String],
  createdAt: {
    type: Date,
    default: () => Date.now(),
    immutable: true,
  },
  updatedAt: Date,
  comments: [{
    user: String,
    content: String,
    votes: Number
  }]
});
*/

const Blog = model("Blog", blogSchema);

// Middleware
blogSchema.pre('save', function (next) {
  this.updated = Date.now(); // update the date every time a blog post is saved
  next();
});

export default Blog; 