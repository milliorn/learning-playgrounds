# Import necessary modules and settings
from settings import *

# Class for managing the main game


class Game:
    def __init__(self):
        # Create a surface for the game using specified dimensions
        self.surface = pygame.Surface((GAME_WIDTH, GAME_HEIGHT))

        # Get the display surface from Pygame
        self.display = pygame.display.get_surface()

        # Define the position of the game surface within the window
        self.rect = self.surface.get_rect(topleft=(PADDING, PADDING))

        # Create a surface for drawing grid lines
        self.line_surface = self.surface.copy()
        self.line_surface.fill((0, 255, 0))
        self.line_surface.set_colorkey((0, 255, 0))
        self.line_surface.set_alpha(51)

    def draw_grid(self):
        """
        Draw grid lines on a separate surface to overlay on the game surface.
        """
        # Draw vertical lines for the grid
        for col in range(1, COLUMNS):
            x = col * CELL_SIZE
            pygame.draw.line(self.line_surface, LINE_COLOR,
                             (x, 0), (x, self.surface.get_height()), 1)

        # Draw horizontal lines for the grid
        for row in range(1, ROWS):
            y = row * CELL_SIZE
            pygame.draw.line(self.line_surface, LINE_COLOR,
                             (0, y), (self.surface.get_width(), y), 1)

        self.surface.blit(self.line_surface, (0, 0))

    def run(self):
        """
        Run and update the game display.
        """
        self.surface.fill(GRAY)  # Fill the game surface with a gray background

        # Blit (copy) the game surface onto the display surface at the
        # specified position (PADDING, PADDING)
        self.draw_grid()  # Draw the grid lines on the game surface
        # Blit the game surface to the display
        self.display.blit(self.surface, (PADDING, PADDING))
        # Draw a rectangle around the game surface
        pygame.draw.rect(self.display, LINE_COLOR, self.rect, 1, 1)
