# JavaScript Flow Fields - Advanced Experiments

https://www.youtube.com/watch?v=EHwainDAPAw&list=PLYElE_rzEw_uVHNJjZor-b_bJpftEG4tV&t=0s

## Summary

- The code starts by creating a canvas element and getting its 2D rendering context using canvas.getContext('2d'). The context is stored in the ctx variable.
- The canvas is set to have a width and height of 500 pixels.
- The Particle class is defined, which represents a particle in the effect. It takes an effect parameter in its constructor, which is an instance of the Effect class that we will see later.
- Inside the Particle constructor, various properties of the particle are initialized, such as its initial position (randomX and randomY), speed (speedX and speedY), maximum length of the particle's history (randomMaxLength), angle correction factor (randomAngleCorrector), and other variables.
- The Particle class also has a method calculateLookupTable() which creates lookup tables for cosine and sine values. These tables are used to calculate the particle's speed based on its angle.
- The Particle class has methods draw() and update() to draw the particle's history as a line and update its position and angle, respectively.
- The Effect class is defined, which represents the overall effect. It takes the canvas and context as parameters in its constructor.
  Inside the Effect constructor, various properties of the effect are initialized, such as the width and height of the canvas, the number of particles, the cell size for the flow field, etc.
- The Effect class has a method init() which initializes the effect. It first draws text on the canvas using the drawText() method. Then it gets the pixel data from the canvas using context.getImageData() and loops through the pixels to create a flow field. Each cell in the flow field contains information about the position, alpha value, and color angle of the pixel.
- The Effect class also has a method drawGrid() to draw a grid on the canvas for debugging purposes.
- The Effect class has a resize() method to handle window resize events and adjust the canvas size accordingly.
- The Effect class has a render() method which is called in the animation loop. It first clears the canvas using ctx.clearRect(). Then it loops through all the particles, calls their draw() and update() methods, and renders them on the canvas.
- An instance of the Effect class is created with the canvas and context variables.
- The animate() function is defined to start the animation loop. It first clears the canvas, then calls the render() method of the effect instance, and finally requests the next animation frame using requestAnimationFrame(animate).
- The animate() function is called to start the animation.
- This code creates a flow field effect using particles that move in the direction indicated by the flow field. The flow field is created based on the pixel data of the canvas, and particles are updated according to the flow field. The effect creates an interesting visual pattern.
