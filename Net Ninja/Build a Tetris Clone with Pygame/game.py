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
        # Create a game surface with specified dimensions
        self.surface = pygame.Surface((GAME_WIDTH, GAME_HEIGHT))
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
        # Set the alpha value for transparency
        self.line_surface.set_alpha(51)

        # Create a 2D list for storing field data
        self.field_data = [[0 for x in range(COLUMNS)] for y in range(ROWS)]

        # Create a random tetromino
        self.tetromino = Tetromino(
            choice(list(TETROMINOES.keys())),
            self.sprites,
            self.create_new_tetromino,
            self.field_data)

        # Create timers for game actions
        self.timers = {
            'vertical move': Timer(UPDATE_START_SPEED, True, self.move_down),
            'horizontal move': Timer(MOVE_WAIT_TIME),
            'rotate': Timer(ROTATE_WAIT_TIME)
        }

        # Activate the vertical move timer
        self.timers['vertical move'].activate()

    def create_new_tetromino(self):
        """Create a new tetromino."""

        # Check for completed rows
        self.check_finished_rows()

        # Create a random tetromino
        self.tetromino = Tetromino(
            choice(list(TETROMINOES.keys())),
            self.sprites, self.create_new_tetromino, self.field_data)

    def timer_update(self):
        """Update all timers."""

        # Loop through the timers
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
            x = col * CELL_SIZE  # Calculate the x-coordinate

            # Draw a line on the line surface
            pygame.draw.line(self.line_surface, LINE_COLOR,
                             (x, 0), (x, self.surface.get_height()), 1)

        # Draw horizontal lines for the grid
        for row in range(1, ROWS):
            y = row * CELL_SIZE  # Calculate the y-coordinate

            # Draw a line on the line surface
            pygame.draw.line(self.line_surface, LINE_COLOR,
                             (0, y), (self.surface.get_width(), y), 1)

        # Blit (copy) the line surface onto the game surface at the specified
        # position (0, 0)
        self.surface.blit(self.line_surface, (0, 0))

    def input(self):
        """
        Process user input and perform respective actions.
        """
        keys = pygame.key.get_pressed()  # Get the state of all keyboard keys

        # Check if horizontal movement is allowed
        if not self.timers['horizontal move'].active:
            if keys[pygame.K_LEFT]:
                self.tetromino.move_horizontal(-1)  # Move the tetromino left
                # Activate the horizontal move timer
                self.timers['horizontal move'].activate()
            elif keys[pygame.K_RIGHT]:
                self.tetromino.move_horizontal(1)  # Move the tetromino right
                # Activate the horizontal move timer
                self.timers['horizontal move'].activate()

        if not self.timers['rotate'].active:  # Check if rotation is allowed
            if keys[pygame.K_UP]:
                self.tetromino.rotate()  # Rotate the tetromino
                self.timers['rotate'].activate()  # Activate the rotate timer

    def run(self):
        """
        Run the game loop and update the game display.
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

    def check_finished_rows(self):
        """
        Check and clear completed rows, shifting above blocks down.
        """
        deleted_rows = []  # List for storing the rows to delete

        for i, row in enumerate(self.field_data):  # Loop through the rows
            if all(row):  # Check if all blocks in the row are filled
                deleted_rows.append(i)  # Add the row index to the list

        if deleted_rows:  # Check if there are any rows to delete
            for delete_row in deleted_rows:  # Loop through the rows to delete
                # Loop through the blocks in the row
                for block in self.field_data[delete_row]:
                    block.kill()  # Remove the block from the sprite group

                for row in self.field_data:  # Loop through the rows
                    for block in row:  # Loop through the blocks in the row
                        if block and block.pos.y < delete_row:  # Check if the block is above the deleted row
                            block.pos.y += 1  # Move the block down by one row

        # Reset the field data
        self.field_data = [[0 for x in range(COLUMNS)] for y in range(ROWS)]

        for block in self.sprites:  # Loop through the blocks
            # Add the block to the field data
            self.field_data[int(block.pos.y)][int(block.pos.x)] = block

# Class representing a tetromino


class Tetromino:
    def __init__(self, shape, group, create_new_tetromino, field_data):
        """
        Initialize a Tetromino instance.

        Args:
            shape (str): Shape of the tetromino ('T', 'O', 'J', etc.).
            group (pygame.sprite.Group): Sprite group to which the blocks of the tetromino belong.
        """
        self.shape = shape  # Set the shape of the tetromino
        # Get the positions of blocks for the given shape
        self.block_positions = TETROMINOES[shape]['shape']
        # Get the color of the tetromino
        self.color = TETROMINOES[shape]['color']

        self.create_new_tetromino = create_new_tetromino
        self.field_data = field_data

        # Create blocks for the tetromino using the specified positions and
        # color
        self.blocks = [Block(group, pos, self.color)
                       for pos in self.block_positions]

    def next_move_horizontal_collide(self, blocks, amount):
        """
        Check if the tetromino collides with another block horizontally.

        Args:
            blocks (list): List of blocks to check for collision.
            amount (int): Amount of movement.

        Returns:
            bool: True if there is a collision, False otherwise.
        """

        # Check if the tetromino will collide with another block horizontally
        collision_list = [block.horizontal_collide(
            int(block.pos.x + amount), self.field_data) for block in self.blocks]

        return True if any(collision_list) else False

    def next_move_vertical_collide(self, blocks, amount):
        """
        Check if the tetromino collides with another block vertically.

        Args:
            blocks (list): List of blocks to check for collision.
            amount (int): Amount of movement.

        Returns:
            bool: True if there is a collision, False otherwise.
        """
        collision_list = [block.vertical_collide(
            int(block.pos.y + amount), self.field_data) for block in self.blocks]

        return True if any(collision_list) else False

    def move_horizontal(self, amount):
        """
        Move the tetromino horizontally by the specified amount.

        Args:
            amount (int): Amount of movement (-1 for left, 1 for right).
        """
        # Check if the tetromino will collide with another block horizontally
        if not self.next_move_horizontal_collide(self.blocks, amount):
            for block in self.blocks:
                block.pos.x += amount  # Move the block horizontally by the specified amount

    def move_down(self):
        """Move all blocks of the tetromino down by one block."""
        if not self.next_move_vertical_collide(self.blocks, 1):
            for block in self.blocks:
                block.pos.y += 1
        else:
            for block in self.blocks:
                # Add the block to the field data
                self.field_data[int(block.pos.y)][int(block.pos.x)] = block

            self.create_new_tetromino()

            for row in self.field_data:
                print(row)  # Print the field data

    def rotate(self):
        """Rotate the tetromino."""
        # print('rotate')
        if self.shape != 'O':
            # Get the position of the pivot block
            pivot_pos = self.blocks[0].pos

            new_block_positions = [
                block.rotate(pivot_pos) for block in self.blocks]  # Rotate each block

            for pos in new_block_positions:  # Loop through the new block positions
                if pos.x < 0 or pos.x >= COLUMNS:  # Check if the block is out of bounds
                    return  # Return if the block is out of bounds

                if self.field_data[int(pos.y)][int(
                        pos.x)]:  # Check if the block collides with another block
                    return  # Return if the block collides with another block

                if pos.y > ROWS:  # Check if the block is below the field
                    return  # Return if the block is below the field

            for i, block in enumerate(self.blocks):  # Loop through the blocks
                # Set the block position to the new position
                block.pos = new_block_positions[i]

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
        # Convert the position to a vector
        self.pos = pygame.Vector2(pos) + BLOCK_OFFSET
        y = self.pos.y * CELL_SIZE  # Calculate the y-coordinate
        x = self.pos.x * CELL_SIZE  # Calculate the x-coordinate

        # Get the block's rectangle (position and size)
        self.rect = self.image.get_rect(topleft=(x, y))

    def rotate(self, pivot_pos):
        """
        Rotate the block around the specified pivot point.

        Args:
            pivot_pos (tuple): Position of the pivot point.

        Returns:
            tuple: New position of the block.
        """
        # distance = self.pos - pivot_pos  # Get the distance from the pivot
        # rotated = distance.rotate(90)  # Rotate the distance by 90 degrees
        # new_pos = rotated + pivot_pos  # Get the new position of the block
        return pivot_pos + (self.pos - pivot_pos).rotate(90)

    def horizontal_collide(self, x, field_data):
        """
        Check if the block collides with another block horizontally.

        Args:
            x (int): X-coordinate of the block to check against.

        Returns:
            bool: True if the block collides with another block horizontally, False otherwise.
        """
        if not 0 <= x < COLUMNS:  # Check if the block is out of bounds
            return True  # Return True if the block is out of bounds

        if field_data[int(
                self.pos.y)][x]:  # Check if the block collides with another block horizontally
            return True  # Return True if the block collides with another block horizontally

    def vertical_collide(self, y, field_data):
        """
        Check if the block collides with another block vertically.

        Args:
            y (int): Y-coordinate of the block to check against.

        Returns:
            bool: True if the block collides with another block vertically, False otherwise.
        """
        if y >= ROWS:  # Check if the block is below the field
            return True  # Return True if the block is below the field

        # Check if the block collides with another block vertically
        if y >= 0 and field_data[y][int(
                self.pos.x)]:
            return True  # Return True if the block collides with another block vertically

    def update(self):
        """Update the block's position based on its current position."""
        self.rect.topleft = self.pos * \
            CELL_SIZE  # Update the block's rectangle position based on the current position
