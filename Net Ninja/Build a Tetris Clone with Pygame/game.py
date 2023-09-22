# Import necessary modules and settings
from random import choice

from settings import *
from timer import Timer

# Class for managing the main game


class Game:
    def __init__(self):
        """
        Initialize a Game instance.
        """
        self.surface = pygame.Surface(
            (GAME_WIDTH, GAME_HEIGHT))  # Create a game surface with specified dimensions
        # Get the display surface from Pygame
        self.display = pygame.display.get_surface()

        # Define the position of the game surface within the window
        self.rect = self.surface.get_rect(topleft=(PADDING, PADDING))
        # Create a sprite group to hold game sprites
        self.sprites = pygame.sprite.Group()

        # Create a surface for drawing grid lines
        self.line_surface = self.surface.copy()
        # Fill the line surface with a color
        self.line_surface.fill((0, 255, 0))
        # Set the colorkey for transparency
        self.line_surface.set_colorkey((0, 255, 0))
        self.line_surface.set_alpha(51)  # Set the alpha value for transparency

        # Create a random tetromino
        self.tetromino = Tetromino(
            choice(
                list(
                    TETROMINOES.keys())),
            self.sprites)

        # Create timers for game actions
        self.timers = {
            'vertical move': Timer(UPDATE_START_SPEED, True, self.move_down),
            'horizontal move': Timer(MOVE_WAIT_TIME),
            'rotate': Timer(ROTATE_WAIT_TIME)
        }

        # Activate the vertical move timer
        self.timers['vertical move'].activate()

    def timer_update(self):
        """Update all timers."""
        for timer in self.timers.values():
            timer.update()

    def move_down(self):
        """Move the tetromino down by one block."""
        self.tetromino.move_down()

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

    def input(self):
        keys = pygame.key.get_pressed()  # Get the state of all keyboard keys

        # Check if horizontal movement is allowed
        if not self.timers['horizontal move'].active:
            if keys[pygame.K_LEFT]:
                print('Left key pressed')
                self.tetromino.move_horizontal(-1)  # Move the tetromino left
                # Activate the horizontal move timer
                self.timers['horizontal move'].activate()
            elif keys[pygame.K_RIGHT]:
                print('Right key pressed')
                self.tetromino.move_horizontal(1)  # Move the tetromino right
                # Activate the horizontal move timer
                self.timers['horizontal move'].activate()

    def run(self):
        """
        Run and update the game display.
        """
        self.input()  # Get user input
        self.timer_update()  # Update the timers
        self.sprites.update()  # Update all sprites
        self.surface.fill(GRAY)  # Fill the game surface with a gray background
        # Draw all sprites onto the game surface
        self.sprites.draw(self.surface)

        # Blit (copy) the game surface onto the display surface at the
        # specified position (PADDING, PADDING)
        self.draw_grid()  # Draw the grid lines on the game surface
        # Blit the game surface to the display
        self.display.blit(self.surface, (PADDING, PADDING))
        # Draw a rectangle around the game surface
        pygame.draw.rect(self.display, LINE_COLOR, self.rect, 1, 1)


# Class representing a tetromino


class Tetromino:
    def __init__(self, shape, group):
        """
        Initialize a Tetromino instance.

        Args:
            shape (str): Shape of the tetromino ('T', 'O', 'J', etc.).
            group (pygame.sprite.Group): Sprite group to which the blocks of the tetromino belong.
        """
        self.block_positions = TETROMINOES[shape]['shape']  # Get the positions of blocks for the given shape
        # Get the color of the tetromino
        self.color = TETROMINOES[shape]['color']

        # Create blocks for the tetromino using the specified positions and
        # color
        self.blocks = [Block(group, pos, self.color)
                       for pos in self.block_positions]

    def move_horizontal(self, amount):
        """
        Move the tetromino horizontally by one block.

        Args:
            direction (int): Direction of movement (-1 for left, 1 for right).
        """
        for block in self.blocks:
            block.pos.x += amount  # Move the block horizontally by the specified amount

    def move_down(self):
        """Move all blocks of the tetromino down by one block."""
        for block in self.blocks:
            block.pos.y += 1

# Class representing a block (part of a tetromino)


class Block(pygame.sprite.Sprite):
    def __init__(self, group, pos, color):
        """
        Initialize a Block instance.

        Args:
            group (pygame.sprite.Group): Sprite group to which the block belongs.
            pos (tuple): Position of the block.
            color (str): Color of the block.
        """
        super().__init__(group)  # Initialize the sprite using the provided group

        # Create the image (surface) for the block with specified color and
        # dimensions
        self.image = pygame.Surface((CELL_SIZE, CELL_SIZE))
        self.image.fill(color)  # Fill the block with the specified color

        # Set the position of the block based on the provided position and
        # offset
        self.pos = pygame.Vector2(pos) + BLOCK_OFFSET
        y = self.pos.y * CELL_SIZE
        x = self.pos.x * CELL_SIZE

        # Get the block's rectangle (position and size)
        self.rect = self.image.get_rect(topleft=(x, y))

    def update(self):
        """Update the block's position based on its current position."""
        self.rect.topleft = self.pos * \
            CELL_SIZE  # Update the block's rectangle position based on the current position
