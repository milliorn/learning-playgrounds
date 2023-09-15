// Get the canvas element and its 2D rendering context
const canvas = document.getElementById('canvas1');
const ctx = canvas.getContext('2d');

// Set the dimensions of the canvas
canvas.width = 700;
canvas.height = 900;

// Set global variables for the rendering context
ctx.lineWidth = 1;
ctx.shadowOffsetX = 2;
ctx.shadowOffsetY = 2;
ctx.shadowColor = "white";

// Create a linear gradient from top-left to bottom-right of the canvas
const gradient1 = ctx.createLinearGradient(0, 0, canvas.width, canvas.height);
// ... add additional color stops to the gradient ...
gradient1.addColorStop("0.1", 'red');

// Create a radial gradient centered on the canvas
const gradient2 = ctx.createRadialGradient(canvas.width * 0.5, canvas.height * 0.5, 70, canvas.width * 0.5, canvas.height * 0.5, 400);
// ... add additional color stops to the gradient ...
gradient2.addColorStop("0.2", 'green');

// Get the pattern image element and create a pattern using the image
const patternImage = document.getElementById('patternImage');
const pattern1 = ctx.createPattern(patternImage, 'no-repeat');

// Set the stroke style of the context to use the gradient or pattern
// ctx.strokeStyle = gradient1;
// ctx.strokeStyle = gradient2;
ctx.strokeStyle = pattern1;

class Line {
  constructor(canvas) {
    // Initialize the line properties
    this.canvas = canvas;
    this.x = Math.random() * this.canvas.width;
    this.y = Math.random() * this.canvas.height;
    this.history = [ { x: this.x, y: this.y } ];
    this.lineWidth = Math.floor(Math.random() * 25 + 1);
    this.maxLength = Math.floor(Math.random() * 150 + 10);
    this.speedX = Math.random() * 1 - 0.5;
    this.speedY = 7;
    this.lifeSpan = this.maxLength * 2;
    this.breakPoint = this.lifeSpan * 0.85;
    this.timer = 0;
    this.angle = 0;
    this.va = Math.random() * 0.5 - 0.25;
    this.curve = 0.1;
    this.vc = Math.random() * 0.4 - 0.2;

    this.hue = Math.floor(Math.random() * 360);
    this.saturation = Math.floor(Math.random() * 100);
    this.lightness = Math.floor(Math.random() * 100);
  }

  // Method to draw the line on the canvas
  draw(context) {
    // context.strokeStyle = `hsl(${this.hue}, ${this.saturation}%, ${this.lightness}%)`;
    context.lineWidth = this.lineWidth;
    context.beginPath();
    context.moveTo(this.history[ 0 ].x, this.history[ 0 ].y);

    for (let i = 0; i < this.history.length; i++) {
      context.lineTo(this.history[ i ].x, this.history[ i ].y);
    }

    context.stroke();
  }

  // update the line's position and properties
  update() {
    this.timer++;
    this.angle += this.va;
    this.curve += this.vc;

    if (this.timer < this.lifeSpan) {
      if (this.timer > this.breakPoint) {
        this.va *= -1.12;
      }

      this.x += Math.sin(this.angle) * this.curve;
      this.y += Math.cos(this.angle) * this.curve;
      this.history.push({ x: this.x, y: this.y });

      if (this.history.length > this.maxLength) {
        this.history.shift();
      }
    } else if (this.history.length <= 1) {
      this.reset();
    } else {
      this.history.shift();
    }
  }

  // reset the line's properties to a random state
  reset() {
    this.x = Math.random() * this.canvas.width;
    this.y = Math.random() * this.canvas.height;
    this.history = [ { x: this.x, y: this.y } ];
    this.timer = 0;
    this.angle = 0;
    this.curve = 0.0;
    this.va = Math.random() * 0.5 - 0.25;
  }
}

// Create array of Line objects
const linesArray = [];
const numberOfLines = 200;

for (let i = 0; i < numberOfLines; i++) {
  linesArray.push(new Line(canvas));
}

// Print array of lines to the console for debugging
console.log(linesArray);

// animate the canvas by clearing it, drawing and updating the lines, and requesting the next animation frame
function animate() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  linesArray.forEach(line => {
    line.draw(ctx);
    line.update();
  });

  requestAnimationFrame(animate);
}

// Start the animation loop
animate();
