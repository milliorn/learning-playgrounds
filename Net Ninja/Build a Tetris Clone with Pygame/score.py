# Import necessary modules and settings
from settings import *

# Class for displaying the game score


class Score:
    def __init__(self):
        # Create a surface for the score display using specified dimensions
        self.surface = pygame.Surface(
            (SIDEBAR_WIDTH, GAME_HEIGHT * SCORE_HEIGHT_FRACTION - PADDING))

        # Set the position of the score surface within the window
        self.rect = self.surface.get_rect(
            bottomright=(
                WINDOW_WIDTH - PADDING,
                WINDOW_HEIGHT - PADDING))

        # Get the display surface from Pygame
        self.display = pygame.display.get_surface()

    # Method to run and update the score display
    def run(self):
        # Blit (copy) the score surface onto the display surface at the
        # specified position
        self.display.blit(self.surface, self.rect)
