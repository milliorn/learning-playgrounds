# Importing necessary settings and modules
from settings import *
from sys import exit

# Main class definition


class Main:
    def __init__(self):
        pygame.init()  # Initialize Pygame
        pygame.display.set_caption('Tetris')  # Set the window caption
        self.clock = pygame.time.Clock()  # Create a Pygame clock instance
        self.display = pygame.display.set_mode(
            (WINDOW_WIDTH, WINDOW_HEIGHT))  # Set the display dimensions

    def run(self):
        # Main game loop
        while True:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    pygame.quit()  # Quit Pygame
                    exit()  # Exit the program

            self.display.fill(GRAY)  # Fill the display with a gray background
            pygame.display.update()  # Update the display
            self.clock.tick(60)  # Cap the frame rate to 60 frames per second


# Entry point of the program
if __name__ == '__main__':
    main = Main()  # Create an instance of the Main class
    main.run()  # Run the game loop
