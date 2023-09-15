const canvas = document.getElementById('canvas1');
const ctx = canvas.getContext('2d');

// creates full screen canvas
// Set the size of the canvas to match the window or use specific values
canvas.width = 500; //window.innerWidth;
canvas.height = 500; //window.innerHeight;

// canvas settings
// Set the fill and stroke styles for the canvas
ctx.fillStyle = 'white';
ctx.strokeStyle = 'white';
ctx.lineWidth = 1;

class Particle {
  constructor(effect) {
    this.effect = effect;
    // Generate random initial values for the particle
    const randomX = Math.random() * this.effect.width | 0;
    const randomY = Math.random() * this.effect.height | 0;
    const randomSpeedModifier = (Math.random() * 5 + 1) | 0;
    const randomMaxLength = (Math.random() * 200 + 10) | 0;
    const randomAngleCorrector = (Math.random() * 0.5 + 0.01) | 0;

    // Inside Particle constructor:
    this.x = randomX;
    this.y = randomY;
    this.speedX;
    this.speedY;
    this.speedModifier = randomSpeedModifier;
    this.history = [ { x: this.x, y: this.y } ];
    this.maxLength = randomMaxLength;
    this.angle = 0;
    this.newAngle = 0;
    this.angleCorrector = randomAngleCorrector;
    this.timer = this.maxLength * 2;

    // Lookup tables for precalculated cos and sin values
    this.cosLookup = [];
    this.sinLookup = [];
    this.calculateLookupTable();

    // Array of colors and random color selection for the particle
    this.colors = [ 'red',
      'green',
      'blue', 'yellow', 'pink', 'purple', 'orange', 'cyan', 'magenta', 'brown', 'white', 'gray', 'black', ]
    this.color = this.colors[ Math.floor(Math.random() * this.colors.length) ];
  }


  // Calculate the lookup table for cos and sin values
  calculateLookupTable() {
    const lookupTableSize = 360;
    for (let i = 0; i < lookupTableSize; i++) {
      const angle = i * (Math.PI / 180);
      this.cosLookup[ i ] = Math.cos(angle);
      this.sinLookup[ i ] = Math.sin(angle);
    }
  }

  // Draw the particle's history as a line
  draw(context) {
    context.beginPath();
    context.moveTo(this.history[ 0 ].x, this.history[ 0 ].y);

    for (let i = 0; i < this.history.length; i++) {
      context.lineTo(this.history[ i ].x, this.history[ i ].y);
    }
    context.strokeStyle = this.color;
    context.stroke();
  }

  // Update the particle's position and angle
  update() {
    this.timer--;

    if (this.timer >= 1) {
      let x = Math.floor(this.x / this.effect.cellSize);
      let y = Math.floor(this.y / this.effect.cellSize);
      let index = y * this.effect.cols + x;

      if (this.effect.flowField[ index ]) {
        this.angle = this.effect.flowField[ index ].colorAngle;

        if (this.angle > this.newAngle) {
          this.angle -= this.angleCorrector;
        } else if (this.angle < this.newAngle) {
          this.angle += this.angleCorrector;
        } else {
          this.angle = this.newAngle;
        }
      }

      const angleIndex = Math.floor(this.angle * (180 / Math.PI));
      this.speedX = this.cosLookup[ angleIndex ];
      this.speedY = this.sinLookup[ angleIndex ];

      this.x += this.speedX * this.speedModifier;
      this.y += this.speedY * this.speedModifier;

      this.history.push({ x: this.x, y: this.y });

      if (this.history.length > this.maxLength) {
        this.history.shift();
      }
    } else if (this.history > 1) {
      this.history.shift();
    } else {
      this.reset();
    }
  }

  // Reset the particle's position and history
  reset() {
    let attempts = 0;
    let resetSuccess = false;

    while (attempts < 5 && !resetSuccess) {
      attempts++;
      let testindex = Math.floor(Math.random() * this.effect.flowField.length);

      if (this.effect.flowField[ testindex ].alpha > 0) {
        this.x = this.effect.flowField[ testindex ].x;
        this.y = this.effect.flowField[ testindex ].y;
        this.history = [ { x: this.x, y: this.y } ];
        this.timer = this.maxLength * 2;
        resetSuccess = true;
      }
    }

    if (!resetSuccess) {
      this.x = Math.floor(Math.random() * this.effect.width);
      this.y = Math.floor(Math.random() * this.effect.height);
      this.history = [ { x: this.x, y: this.y } ];
      this.timer = this.maxLength * 2;
    }
  }
}

class Effect {
  constructor(canvas, ctx) {
    this.canvas = canvas;
    this.context = ctx;
    this.width = this.canvas.width;
    this.height = this.canvas.height;
    this.particles = [];
    this.numberOfParticles = 1000; // 2000;
    this.cellSize = 20; //5;
    this.cols;
    this.row;
    this.flowField = [];
    // add curve, zoom back with window.innerWidth/Height
    // this.curve = 5;
    // this.zoom = 0.07;
    this.debug = false;
    this.init();

    // Toggle debug mode with the 'd' key
    window.addEventListener('keydown', (e) => {
      // console.log(e);
      if (e.key === 'd') {
        this.debug = !this.debug;
      }
    });

    // Handle window resize event
    window.addEventListener('resize', (e) => {
      // console.log(e);
      // add this back if you revert to window.innerWidth/Height
      // this.resize(e.target.innerWidth, e.target.innerHeight);
    });
  }

  // Draw text on the canvas
  drawText() {
    this.context.font = '450px Impact';
    this.context.textAlign = 'center';
    this.context.textBaseline = 'middle';

    const gradient1 = this.context.createLinearGradient(0, 0, this.width, this.height);
    gradient1.addColorStop(0.2, `rgb(255, 0, 0)`);
    gradient1.addColorStop(0.4, `rgb(0, 255, 0)`);
    gradient1.addColorStop(0.6, `rgb(150, 100, 100)`);
    gradient1.addColorStop(0.8, `rgb(0, 255, 255)`);

    this.context.fillStyle = gradient1;
    this.context.fillText('🤘', this.width * 0.5, this.height * 0.5, this.width * 0.8);
  }

  // Initialize the effect
  init() {
    // create flow field
    this.rows = Math.floor(this.height / this.cellSize);
    this.cols = Math.floor(this.width / this.cellSize);
    this.flowField = [];

    // Draw text on the canvas
    this.drawText();

    // Get pixel data from the canvas
    const pixels = this.context.getImageData(0, 0, this.width, this.height).data;
    // console.log(pixels);

    // Loop through pixels to create flow field
    for (let y = 0; y < this.height; y += this.cellSize) {
      for (let x = 0; x < this.width; x += this.cellSize) {
        const index = (y * this.width + x) * 4;
        const red = pixels[ index ];
        const green = pixels[ index + 1 ];
        const blue = pixels[ index + 2 ];
        const alpha = pixels[ index + 3 ];
        const grayscale = (red + 76 + green + blue) / 3;
        const colorAngle = ((grayscale / 255) * Math.PI * 2).toFixed(2);

        this.flowField.push({ x: x, y: y, alpha: alpha, colorAngle: colorAngle });
      }
    }

    // console.log(this.flowField);

    // create particles
    this.particles = [];
    for (let i = 0; i < this.numberOfParticles; i++) {
      this.particles.push(new Particle(this));
    }

    // Reset particles
    this.particles.forEach((particle) => { particle.reset() });
  }

  // Draw a grid on the canvas for debugging
  drawGrid() {
    this.context.save();
    this.context.strokeStyle = 'white';
    this.context.lineWidth = 0.3;

    for (let c = 0; c < this.cols; c++) {
      this.context.beginPath();
      this.context.moveTo(c * this.cellSize, 0);
      this.context.lineTo(c * this.cellSize, this.height);
      this.context.stroke();
    }

    for (let r = 0; r < this.rows; r++) {
      this.context.beginPath();
      this.context.moveTo(0, r * this.cellSize);
      this.context.lineTo(this.width, r * this.cellSize);
      this.context.stroke();
    }
    this.context.restore();
  }

  // Handle window resize
  resize(width, height) {
    this.canvas.width = width;
    this.canvas.height = height;
    this.width = this.canvas.width;
    this.height = this.canvas.height;
    this.init();
  }

  // Render the effect on the canvas
  render() {
    if (this.debug) {
      this.drawGrid();
      this.drawText();
    }

    this.particles.forEach(particle => {
      particle.draw(this.context);
      particle.update();
    });
  }
}

// Create an instance of the Effect class
const effect = new Effect(canvas, ctx);

// console.log(effect);

// Call the animate function to start the animation loop
function animate() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  // effect.drawText();

  effect.render();
  requestAnimationFrame(animate);
}

animate();