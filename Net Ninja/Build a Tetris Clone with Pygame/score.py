from os.path import join
from settings import *


class Score:
    def __init__(self):
        self.surface = pygame.Surface(
            (SIDEBAR_WIDTH,
             GAME_HEIGHT *
             SCORE_HEIGHT_FRACTION -
             PADDING))  # create surface
        self.rect = self.surface.get_rect(
            bottomright=(
                WINDOW_WIDTH - PADDING,
                WINDOW_HEIGHT - PADDING))  # create rect
        self.display_surface = pygame.display.get_surface()     # get display

        # font
        self.font = pygame.font.Font(
            join('assets', 'graphics', 'Russo_One.ttf'), 30)

        # increment
        self.increment_height = self.surface.get_height() / 3  # calculate increment height

        # data
        self.score = 0  # create score
        self.level = 1  # create level
        self.lines = 0  # create lines

    def display_text(self, pos, text):
        text_surface = self.font.render(
            f'{text[0]}: {text[1]}',
            True,
            'white')  # create text surface
        text_rext = text_surface.get_rect(center=pos)  # create text rect
        self.surface.blit(text_surface, text_rext)  # blit text

    def run(self):
        self.surface.fill(GRAY)  # fill surface
        for i, text in enumerate(
                [('Score', self.score), ('Level', self.level), ('Lines', self.lines)]):  # loop through text
            x = self.surface.get_width() / 2  # calculate x
            y = self.increment_height / 2 + i * self.increment_height  # calculate y
            self.display_text((x, y), text)  # display text

        self.display_surface.blit(self.surface, self.rect)  # blit surface
        pygame.draw.rect(
            self.display_surface,
            LINE_COLOR,
            self.rect,
            2,
            2)  # draw border
