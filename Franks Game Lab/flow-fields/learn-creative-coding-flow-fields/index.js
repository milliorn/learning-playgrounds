const canvas = document.getElementById('canvas1');
const ctx = canvas.getContext('2d');

// creates full screen canvas
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

// canvas settings
ctx.fillStyle = 'blue';
ctx.strokeStyle = 'white';
ctx.lineWidth = 1;

class Particle {
  constructor(effect) {
    this.effect = effect;
    this.x = Math.floor(Math.random() * this.effect.width);
    this.y = Math.floor(Math.random() * this.effect.height);
    this.speedX;
    this.speedY;
    this.speedModifier = Math.random() * 0.5 + 0.5;
    this.history = [ { x: this.x, y: this.y } ];
    this.maxLength = Math.floor(Math.random() * 400 + 10);
    this.angle = 0;
    this.timer = this.maxLength * 2;
    this.colors = [ 'red',
      'green',
      'blue', 'yellow', 'pink', 'purple', 'orange', 'cyan', 'magenta', 'brown', 'white', 'gray', 'black', ]
    this.color = this.colors[ Math.floor(Math.random() * this.colors.length) ];
  }

  draw(context) {
    context.fillRect(this.x, this.y, 10, 10);
    context.beginPath();
    context.moveTo(this.history[ 0 ].x, this.history[ 0 ].y);

    for (let i = 0; i < this.history.length; i++) {
      context.lineTo(this.history[ i ].x, this.history[ i ].y);
    }
    context.strokeStyle = this.color;
    context.stroke();
  }

  update() {
    this.timer--;

    if (this.timer >= 1) {
      let x = Math.floor(this.x / this.effect.cellSize);
      let y = Math.floor(this.y / this.effect.cellSize);
      let index = y * this.effect.cols + x;
      this.angle = this.effect.flowField[ index ];

      this.speedX = Math.cos(this.angle);
      this.speedY = Math.sin(this.angle);
      this.x += this.speedX * this.speedModifier;
      this.y += this.speedY * this.speedModifier;

      this.history.push({ x: this.x, y: this.y });

      if (this.history.length > this.maxLength) {
        // this.history.splice(0, 1);
        this.history.shift();
        // this.history.push();
      }
    } else if (this.history > 1) {
      this.history.shift();
    } else {
      this.reset();
    }
  }

  reset() {
    this.x = Math.floor(Math.random() * this.effect.width);
    this.y = Math.floor(Math.random() * this.effect.height);
    this.history = [ { x: this.x, y: this.y } ];
    this.timer = this.maxLength * 2;
  }
}

class Effect {
  constructor(canvas) {
    this.canvas = canvas;
    this.width = this.canvas.width;
    this.height = this.canvas.height;
    this.particles = [];
    this.numberOfParticles = 300;
    this.cellSize = 20;
    this.cols;
    this.row;
    this.flowField = [];
    this.curve = 0.5;
    this.zoom = 0.3;
    this.debug = false;
    this.init();

    window.addEventListener('keydown', (e) => {
      // console.log(e);
      if (e.key === 'd') {
        this.debug = !this.debug;
      }
    });

    window.addEventListener('resize', (e) => {
      // console.log(e);
      this.resize(e.target.innerWidth, e.target.innerHeight);
    });
  }

  init() {
    // create flow field
    this.rows = Math.floor(this.height / this.cellSize);
    this.cols = Math.floor(this.width / this.cellSize);
    this.flowField = [];

    for (let y = 0; y < this.rows; y++) {
      for (let x = 0; x < this.cols; x++) {
        let angle = Math.cos(x * this.zoom) + Math.sin(y * this.zoom) * this.curve;
        this.flowField.push(angle);
      }
    }

    // console.log(this.flowField);

    // create particles
    this.particles = [];
    for (let i = 0; i < this.numberOfParticles; i++) {
      this.particles.push(new Particle(this));
    }
  }

  drawGrid(context) {
    context.save();
    context.strokeStyle = 'green';
    context.lineWidth = 0.5;

    for (let c = 0; c < this.cols; c++) {
      context.beginPath();
      context.moveTo(c * this.cellSize, 0);
      context.lineTo(c * this.cellSize, this.height);
      context.stroke();
    }

    for (let r = 0; r < this.rows; r++) {
      context.beginPath();
      context.moveTo(0, r * this.cellSize);
      context.lineTo(this.width, r * this.cellSize);
      context.stroke();
    }
    context.restore();
  }

  resize(width, height) {
    this.canvas.width = width;
    this.canvas.height = height;
    this.width = this.canvas.width;
    this.height = this.canvas.height;
    this.init();
  }

  render(context) {
    if (this.debug) {
      this.drawGrid(context);
    }

    this.particles.forEach(particle => {
      particle.draw(context);
      particle.update();
    });
  }
}

const effect = new Effect(canvas);

console.log(effect);

function animate() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  effect.render(ctx);
  requestAnimationFrame(animate);
}

animate();