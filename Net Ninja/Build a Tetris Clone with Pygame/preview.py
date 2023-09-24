# Import necessary modules and settings
from settings import *
from pygame.image import load
from os import path

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

        # Dictionary to store shape surfaces for preview
        self.shape_surfaces = {
            shape: load(
                path.join(
                    'assets',
                    'graphics',
                    f'{shape}.png')).convert_alpha() for shape in TETROMINOES.keys()}

        self.increment_height = self.surface.get_height() / 3  # Height of each fragment

    def display_pieces(self, shapes):
        """
        Display the preview of next shapes on the preview surface.

        Args:
            shapes (list): List of next shapes to display.
        """
        for i, shape in enumerate(shapes):
            shape_surface = self.shape_surfaces[shape]  # Get the shape surface

            x = self.surface.get_width() / 2  # Calculate the x position of the shape
            # Calculate the y position of the shape
            y = self.increment_height / 2 + i * self.increment_height
            rect = shape_surface.get_rect(center=(x, y))  # Get the shape rect

            # Blit the shape surface onto the preview surface
            self.surface.blit(shape_surface, rect)

    def run(self, next_shapes):
        """
        Run and update the preview display.

        Args:
            next_shapes (list): List of next shapes to display.
        """
        self.surface.fill(
            GRAY)  # Fill the preview surface with gray background
        # Display the next shapes on the preview surface
        self.display_pieces(next_shapes)

        # Blit (copy) the preview surface onto the display surface at the
        # specified position
        self.display_surface.blit(self.surface, self.rect)
        # Draw a rectangle around the preview surface
        pygame.draw.rect(self.display_surface, LINE_COLOR, self.rect, 2, 2)
