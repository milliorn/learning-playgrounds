# Serde by Example 2: OpenStreetMap

source -> <https://blog.dzejkop.space/posts/serde-by-example-2.html>

Creating a New Package -> <https://doc.rust-lang.org/cargo/guide/creating-a-new-project.html>

Rust.gitignore ->  <https://github.com/github/gitignore/blob/main/Rust.gitignore>

OpenStreetMap API -> <https://api.openstreetmap.org>

openstreetmap wiki -> <https://wiki.openstreetmap.org/wiki/API_v0.6>

## OpenStreetMap api supports the following actions regarding the map data

- Retrieving map data by bounding box - GET /api/0.6/map
- Reading element data - GET /api/0.6/[node|way|relation]/#id
- Reading element history - GET /api/0.6/[node|way|relation]/#id/history
- Reading element version - GET /api/0.6/[node|way|relation]/#id/#version
- Fetching multiple elements - GET /api/0.6/[nodes|ways|relations]?#parameters
- Fetching relations for of a given element - GET /api/0.6/[node|way|relation]/#id/relations
- Fetching ways for a given node - GET /api/0.6/node/#id/ways
- Reading full data of an element - GET /api/0.6/[way|relation]/#id/full

### OSM deals with 3 element types

- Node - a single element like a statue
- Way - a collection of nodes forming a way
- Relation - some relation of nodes, for example a number of ways can make up a city square
- And the response of each endpoint simply returns a collection of these elements.

#### And the response of each endpoint simply returns a collection of these elements

## Inspecting the data

...ignoring the 'elements' field, we have the header:

```json
{
  "attribution": "http://www.openstreetmap.org/copyright",
  "bounds": {
    "maxlat": 51.1105513,
    "maxlon": 17.0312816,
    "minlat": 51.1102279,
    "minlon": 17.0308739
  },
  "copyright": "OpenStreetMap and contributors",
  "generator": "CGImap 0.8.8 (3729426 spike-07.openstreetmap.org)",
  "license": "http://opendatacommons.org/licenses/odbl/1-0/",
  "version": "0.6"
}
```

...you'll see that the nodes actually make up the majority of the response. We know that these are nodes because of the "type": "node" field. Every element will have a set of common attributes. These are, in no particular order:

```json
[
  {
    "changeset": 126142387,
    "id": 309274935,
    "lat": 51.1104557,
    "lon": 17.0312014,
    "tags": {
      "artwork_type": "statue",
      "brand:wikidata": "Q910242",
      "brand:wikipedia": "pl:Wrocławskie krasnale",
      "name": "Życzliwek",
      "network": "Wrocławskie krasnale",
      "source": "survey",
      "tourism": "artwork",
      "wheelchair": "yes"
    },
    "timestamp": "2022-09-13T16:31:04Z",
    "type": "node",
    "uid": 15262530,
    "user": "Pavol33",
    "version": 9
  },
  {
    "changeset": 96980809,
    "id": 2819200844,
    "lat": 51.1103102,
    "lon": 17.0310379,
    "timestamp": "2021-01-05T12:45:17Z",
    "type": "node",
    "uid": 8826419,
    "user": "Mordechai23",
    "version": 2
  },
  {
    "changeset": 96980809,
    "id": 2819200858,
    "lat": 51.1105131,
    "lon": 17.0310927,
    "timestamp": "2021-01-05T12:45:17Z",
    "type": "node",
    "uid": 8826419,
    "user": "Mordechai23",
    "version": 2
  }
]
```

- id - A 64-bit signed integer which identifies the given element. Negative values are used when creating new elements, but elements which are already created will always have a positive id. Notably
it's possible for two elements to share the same id if they're of a different type, i.e. there can exist a node with id equal to 2819200844 and there can also exist a way with that same id.
- user - The display name of the user that last modified this object.
- uid - The id of said user.
- timestamp - Time (in a W3C timestamp format) of the last modification.
- visible - Not present in the response, but listed in the wiki. Would have a value of false if the element was deleted.
- version - The version of the object. All elements start with a version 1 and this value is increment with each change.
- changeset - The changeset id, that contained the latest change to this element. Changesets are out of scope of this article since I only want to describe reading data from OSM not writing.
- tags - All element types can have tags, which are a text-to-text mapping, used to describe certain features of a given element, e.g. in the data above we see the first element has a tag "artwork_type": "statue", signalling that it's a statue.

Next up, we have some ways. Ways are collections of nodes that comprise, well... a way of sorts. The first way in the above data is a stone paved path. The second is a fountain. Or rather it describes the outline of the fountain.

```json
[
  {
    "changeset": 94262272,
    "id": 21587265,
    "nodes": [5505422911, 5505422888],
    "tags": {
      "highway": "footway",
      "lit": "yes",
      "surface": "paving_stones"
    },
    "timestamp": "2020-11-17T09:30:38Z",
    "type": "way",
    "uid": 2455523,
    "user": "StalkerOSM",
    "version": 10
  },
  {
    "changeset": 119788184,
    "id": 277426784,
    "nodes": [3851723164, 2819200858, 2819200861, 2819200844, 3851723164],
    "tags": {
      "amenity": "fountain",
      "image": "https://photos.app.goo.gl/QRNBitJYADTAzSLp7",
      "name": "Fontanna \"ZdrĂłj\"",
      "natural": "water",
      "operator": "ZDiUM WrocĹaw",
      "url": "https://polska-org.pl/510091,Wroclaw,Fontanna_Zdroj.html",
      "url:0": "https://wroclaw.fotopolska.eu/12146,obiekt.html"
    },
    "timestamp": "2022-04-16T15:17:20Z",
    "type": "way",
    "uid": 3476229,
    "user": "maro21",
    "version": 7
  }
]
```

And finally, the third element type is as relation. There's only one relation within this bounding box and it's the one that describes the city square. Relations have a field members, each member is an object composed of the following fields:

```json
{
  "changeset": 97517773,
  "id": 21981,
  "members": [
    {
      "ref": 277426784,
      "role": "inner",
      "type": "way"
    },
    {
      "ref": 892961287,
      "role": "inner",
      "type": "way"
    },
    {
      "ref": 892961288,
      "role": "inner",
      "type": "way"
    },
    {
      "ref": 892961289,
      "role": "inner",
      "type": "way"
    },
    {
      "ref": 892961290,
      "role": "inner",
      "type": "way"
    },
    {
      "ref": 101129503,
      "role": "inner",
      "type": "way"
    },
    {
      "ref": 277426794,
      "role": "inner",
      "type": "way"
    },
    {
      "ref": 25708300,
      "role": "inner",
      "type": "way"
    },
    {
      "ref": 25708301,
      "role": "outer",
      "type": "way"
    },
    {
      "ref": 743442967,
      "role": "outer",
      "type": "way"
    }
  ],
  "tags": {
    "area": "yes",
    "bicycle:conditional": "yes @ (05:00-09:00)",
    "highway": "pedestrian",
    "lit": "yes",
    "name": "Rynek",
    "place": "square",
    "type": "multipolygon"
  },
  "timestamp": "2021-01-14T22:05:21Z",
  "type": "relation",
  "uid": 553541,
  "user": "B_KSL",
  "version": 18
}
```

- ref - An id of an element that's a part of this relation.
- role - An optional text field which describes the role of the element in the relation.
- type - The type of the given element - this is necessary since different element types don't share the same id space.
