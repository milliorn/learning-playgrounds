# Import necessary modules and custom settings
from sys import exit

from game import Game  # Import the Game class
from preview import Preview  # Import the Preview class
from score import Score  # Import the Score class
from settings import *
from random import choice

# Main class definition


class Main:
    def __init__(self):
        pygame.init()  # Initialize Pygame
        pygame.display.set_caption('Tetris')  # Set the window caption
        self.clock = pygame.time.Clock()  # Create a Pygame clock instance
        self.display = pygame.display.set_mode(
            (WINDOW_WIDTH, WINDOW_HEIGHT))  # Set the display dimensions

        # List of next shapes to be displayed
        self.next_shapes = [choice(list(TETROMINOES.keys()))
                            for shape in range(3)]
        # print(self.next_shapes)

        # Create instances of the classes
        self.game = Game(self.get_next_shape)
        self.preview = Preview()
        self.score = Score()

    def get_next_shape(self):
        # Get the next shape in the list and append a new random shape
        next_shape = self.next_shapes.pop(0) # Get the next shape in the list
        self.next_shapes.append(choice(list(TETROMINOES.keys()))) # Append a new random shape
        return next_shape # Return the next shape

    def run(self):
        # Main game loop
        while True: # Loop forever
            for event in pygame.event.get(): # Get all the events from the event queue
                if event.type == pygame.QUIT: # If the user clicked the close button
                    pygame.quit()  # Quit Pygame
                    exit()  # Exit the program

            self.display.fill(GRAY)  # Fill the display with a gray background

            self.game.run()  # Run the game loop for Game
            self.score.run()  # Run the score update
            self.preview.run(self.next_shapes)  # Run the preview display

            pygame.display.update()  # Update the display
            self.clock.tick(60)  # Cap the frame rate to 60 frames per second


# Entry point of the program
if __name__ == '__main__':
    main = Main()  # Create an instance of the Main class
    main.run()  # Run the game loop
