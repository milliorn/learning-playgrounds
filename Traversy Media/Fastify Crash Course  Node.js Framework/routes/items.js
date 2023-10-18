const {
  getItems,
  getItem,
  addItem,
} = require('../controllers/items')

// Item schema
const Item = {
  type: 'object',
  properties: {
    id: { type: 'string' },
    name: { type: 'string' },
  },
}

// Options for get all items
const getItemsOpts = {
  schema: {
    response: {
      200: {
        type: 'array',
        items: Item,
      },
    },
  },
  handler: getItems,
}

const getItemOpts = {
  schema: {
    response: {
      200: Item,
    },
  },
  handler: getItem,
}

const postItemOpts = {
  schema: {
    body: {
      type: 'object',
      required: [ 'name' ],
      properties: {
        name: { type: 'string' },
      },
    },
    response: {
      201: Item,
    },
  },
  handler: addItem,
}

function itemRoutes(fastify, options, done) {
  // Get all items
  fastify.get('/items', getItemsOpts)

  // Get single items
  fastify.get('/items/:id', getItemOpts)

  // Add item
  fastify.post('/items', postItemOpts)

  done()
}

module.exports = itemRoutes