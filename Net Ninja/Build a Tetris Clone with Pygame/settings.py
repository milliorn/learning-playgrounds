import pygame  # Importing the pygame library

# Game size
COLUMNS = 10  # Number of columns in the game grid
ROWS = 20  # Number of rows in the game grid
CELL_SIZE = 40  # Size of each cell in the game grid
GAME_WIDTH, GAME_HEIGHT = COLUMNS * CELL_SIZE, ROWS * \
    CELL_SIZE  # Total game dimensions

# Sidebar size
SIDEBAR_WIDTH = 208  # Width of the sidebar
PREVIEW_HEIGHT_FRACTION = 0.7  # Height fraction for the preview area
# Height fraction for the score area
SCORE_HEIGHT_FRACTION = 1 - PREVIEW_HEIGHT_FRACTION

# Window
PADDING = 16  # Padding around the game area
WINDOW_HEIGHT = GAME_HEIGHT + PADDING * 2  # Total window height
WINDOW_WIDTH = GAME_WIDTH + SIDEBAR_WIDTH + PADDING * 3  # Total window width

# Game behavior
BLOCK_OFFSET = pygame.Vector2(COLUMNS // 2, -1)  # Initial block offset
MOVE_WAIT_TIME = 200  # Wait time for block movement
ROTATE_WAIT_TIME = 200  # Wait time for block rotation
UPDATE_START_SPEED = 600  # Initial update speed for the game default 600

# Colors
BLUE = '#204b9b'
CYAN = '#6cc6d9'
GRAY = '#1c1c1c'
GREEN = '#65b32e'
LINE_COLOR = '#ffffff'
ORANGE = '#f07e13'
PURPLE = '#7b217f'
RED = '#e51b20'
YELLOW = '#ffff00'


# Tetris shapes
TETROMINOES = {
    'I': {'shape': [(0, 0), (0, -1), (0, -2), (0, 1)], 'color': CYAN},
    'J': {'shape': [(0, 0), (0, -1), (0, 1), (-1, 1)], 'color': BLUE},
    'L': {'shape': [(0, 0), (0, -1), (0, 1), (1, 1)], 'color': ORANGE},
    'O': {'shape': [(0, 0), (0, -1), (1, 0), (1, -1)], 'color': YELLOW},
    'S': {'shape': [(0, 0), (-1, 0), (0, -1), (1, -1)], 'color': GREEN},
    'T': {'shape': [(0, 0), (-1, 0), (1, 0), (0, -1)], 'color': PURPLE},
    'Z': {'shape': [(0, 0), (1, 0), (0, -1), (-1, -1)], 'color': RED}
}

# Score data for line clearing
SCORE_DATA = {1: 100, 2: 300, 3: 500, 4: 800}
