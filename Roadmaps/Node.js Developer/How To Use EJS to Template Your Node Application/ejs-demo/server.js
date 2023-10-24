const express = require('express'); // import express
const app = express(); // initialize express
const port = 8080;

app.set('view engine', 'ejs'); // set view engine to ejs

app.get('/', (req, res) => {

  const mascots = [
    { name: 'Sammy', organization: "DigitalOcean", birth_year: 2012 },
    { name: 'Tux', organization: "Linux", birth_year: 1996 },
    { name: 'Moby Dock', organization: "Docker", birth_year: 2013 },
    { name: 'Ferris the crab', organization: "Rust", birth_year: 2015 },
    { name: 'Docker Captain', organization: "Docker", birth_year: 2014 },
    { name: 'Octocat', organization: "GitHub", birth_year: 2007 },
    { name: 'Suse the lizard', organization: "Suse", birth_year: 1996 },
  ];

  const tagline = "No programming concept is complete without a cute animal mascot.";

  res.render('pages/index', {
    mascots: mascots,
    tagline: tagline
  });
});

app.get('/about', (req, res) => {
  res.render('pages/about');
});

// app.get('/contact', (req, res) => {
//   res.render('pages/contact');
// });

// app.get('/projects', (req, res) => {
//   res.render('pages/projects');
// });

// app.get('/services', (req, res) => {
//   res.render('pages/services');
// });

// app.get('/team', (req, res) => {
//   res.render('pages/team');
// });

// app.get('/testimonials', (req, res) => {
//   res.render('pages/testimonials');
// });

// start the server listening for requests
app.listen(port, () => console.log(`Server running at http://localhost:${port}`));
