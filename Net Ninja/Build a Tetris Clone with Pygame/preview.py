# Import necessary modules and settings
from settings import *

# Class for displaying a preview of the game


class Preview:
    def __init__(self):
        # Get the display surface from Pygame
        self.display_surface = pygame.display.get_surface()

        # Create a surface for the preview using specified dimensions
        self.surface = pygame.Surface(
            (SIDEBAR_WIDTH, GAME_HEIGHT * PREVIEW_HEIGHT_FRACTION))

        # Set the position of the preview surface within the window
        self.rect = self.surface.get_rect(
            topright=(WINDOW_WIDTH - PADDING, PADDING))

    # Method to run and update the preview display
    def run(self):
        # Blit (copy) the preview surface onto the display surface at the
        # specified position
        self.display_surface.blit(self.surface, self.rect)
