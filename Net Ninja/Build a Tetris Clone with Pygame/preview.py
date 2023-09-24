from os import path
from pygame.image import load
from settings import *


class Preview:
    def __init__(self):

        # general
        self.display_surface = pygame.display.get_surface()  # get display
        self.surface = pygame.Surface(
            (SIDEBAR_WIDTH, GAME_HEIGHT * PREVIEW_HEIGHT_FRACTION))  # create surface
        self.rect = self.surface.get_rect(
            topright=(WINDOW_WIDTH - PADDING, PADDING))  # create rect

        # shapes
        self.shape_surfaces = {
            shape: load(
                path.join(
                    'assets',
                    'graphics',
                    f'{shape}.png')).convert_alpha() for shape in TETROMINOES.keys()}  # create shape surfaces

        # image position data
        self.increment_height = self.surface.get_height() / 3  # calculate increment height

    def display_pieces(self, shapes):
        for i, shape in enumerate(shapes):  # loop through shapes
            shape_surface = self.shape_surfaces[shape]  # get shape surface
            x = self.surface.get_width() / 2  # calculate x
            y = self.increment_height / 2 + i * self.increment_height  # calculate y
            rect = shape_surface.get_rect(center=(x, y))  # create rect
            self.surface.blit(shape_surface, rect)  # blit shape

    def run(self, next_shapes):
        self.surface.fill(GRAY)  # fill surface
        self.display_pieces(next_shapes)  # display pieces
        self.display_surface.blit(self.surface, self.rect)  # blit surface
        pygame.draw.rect(
            self.display_surface,
            LINE_COLOR,
            self.rect,
            2,
            2)  # draw border
