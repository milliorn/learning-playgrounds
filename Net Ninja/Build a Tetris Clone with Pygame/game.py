# Import necessary modules and settings
from settings import *

# Class for managing the main game


class Game:
    def __init__(self):
        # Create a surface for the game using specified dimensions
        self.surface = pygame.Surface((GAME_WIDTH, GAME_HEIGHT))

        # Get the display surface from Pygame
        self.display = pygame.display.get_surface()

    # Method to run and update the game display
    def run(self):
        # Blit (copy) the game surface onto the display surface at the
        # specified position (PADDING, PADDING)
        self.display.blit(self.surface, (PADDING, PADDING))
