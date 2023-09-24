from os.path import join
from random import choice
from sys import exit

from game import Game
from preview import Preview
from score import Score
from settings import *


class Main:
    def __init__(self):

        # general
        pygame.init()  # initialize pygame
        self.display_surface = pygame.display.set_mode(
            (WINDOW_WIDTH, WINDOW_HEIGHT))  # create display
        self.clock = pygame.time.Clock()  # create clock
        pygame.display.set_caption('Tetris')  # set caption

        # shapes
        self.next_shapes = [choice(list(TETROMINOES.keys()))
                            for shape in range(3)]  # create next shapes

        # components
        self.game = Game(self.get_next_shape, self.update_score)  # create game
        self.preview = Preview()  # create preview
        self.score = Score()  # create score

        # audio
        self.music = pygame.mixer.Sound(
            join('assets', 'sound', 'music.wav'))  # create music
        self.music.set_volume(0.05)  # set volume
        self.music.play(-1)  # play music

    def update_score(self, lines, score, level):  # update score
        self.score.lines = lines  # update lines
        self.score.score = score  # update score
        self.score.level = level  # update level

    def get_next_shape(self):  # get next shape
        next_shape = self.next_shapes.pop(0)
        self.next_shapes.append(
            choice(list(TETROMINOES.keys())))  # create new shape
        return next_shape

    def run(self):  # main loop
        while True:
            for event in pygame.event.get():  # event loop
                if event.type == pygame.QUIT:
                    pygame.quit()  # quit pygame
                    exit()

            # display
            self.display_surface.fill(GRAY)

            # components
            self.game.run()
            self.score.run()
            self.preview.run(self.next_shapes)

            # updating the game
            pygame.display.update()  # update display
            self.clock.tick()  # tick clock


if __name__ == '__main__':
    main = Main()
    main.run()
