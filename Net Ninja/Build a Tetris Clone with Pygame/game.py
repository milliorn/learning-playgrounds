from os.path import join
from random import choice
from sys import exit

from settings import *
from tetromino import Tetromino
from timer import Timer


class Game:
    def __init__(self, get_next_shape, update_score):

        # general
        self.surface = pygame.Surface(
            (GAME_WIDTH, GAME_HEIGHT))  # create surface
        self.display_surface = pygame.display.get_surface()  # get display
        self.rect = self.surface.get_rect(
            topleft=(PADDING, PADDING))  # create rect
        self.sprites = pygame.sprite.Group()  # create sprite group

        # game connection
        self.get_next_shape = get_next_shape
        self.update_score = update_score

        # lines
        self.line_surface = self.surface.copy()  # create line surface
        self.line_surface.fill((0, 255, 0))  # fill with green
        self.line_surface.set_colorkey((0, 255, 0))  # set colorkey
        self.line_surface.set_alpha(120)  # set alpha

        # tetromino
        # create field data
        self.field_data = [[0 for x in range(COLUMNS)] for y in range(ROWS)]
        self.tetromino = Tetromino(
            choice(list(TETROMINOES.keys())),
            self.sprites,
            self.create_new_tetromino,
            self.field_data)  # create tetromino

        # timer
        self.down_speed = UPDATE_START_SPEED  # create down speed
        self.down_speed_faster = self.down_speed * 0.1  # create down speed faster
        self.down_pressed = False  # create down pressed
        self.timers = {
            'vertical move': Timer(self.down_speed, True, self.move_down),
            'horizontal move': Timer(MOVE_WAIT_TIME),
            'rotate': Timer(ROTATE_WAIT_TIME)
        }  # create timers
        self.timers['vertical move'].activate()  # activate vertical move timer

        # score
        self.current_level = 1  # create current level
        self.current_score = 0  # create current score
        self.current_lines = 0  # create current lines

        # sound
        self.landing_sound = pygame.mixer.Sound(
            join('assets', 'sound', 'landing.wav'))
        self.landing_sound.set_volume(0.1)  # set volume

    def calculate_score(self, num_lines):
        self.current_lines += num_lines  # update current lines
        self.current_score += SCORE_DATA[num_lines] * \
            self.current_level  # update current score

        if self.current_lines / 1 > self.current_level:
            self.current_level += 1  # update current level
            self.down_speed *= 0.1  # update down speed
            self.down_speed_faster = self.down_speed * 0.1  # update down speed faster
            # update vertical move timer duration
            self.timers['vertical move'].duration = self.down_speed

        self.update_score(
            self.current_lines,
            self.current_score,
            self.current_level)  # update score

    def check_game_over(self):  # check if game is over
        for block in self.tetromino.blocks:  # check if any block is above the field
            if block.pos.y < 0:  # if so, exit the game
                exit()  # exit the game

    def create_new_tetromino(self):  # create new tetromino
        self.landing_sound.play()  # play landing sound
        self.check_game_over()  # check if game is over
        self.check_finished_rows()  # check if there are any finished rows
        self.tetromino = Tetromino(
            self.get_next_shape(),
            self.sprites,
            self.create_new_tetromino,
            self.field_data)  # create new tetromino

    def timer_update(self):  # update timers
        for timer in self.timers.values():
            timer.update()  # update timer

    def move_down(self):  # move tetromino down
        self.tetromino.move_down()

    def draw_grid(self):  # draw grid
        for col in range(1, COLUMNS):  # draw vertical lines
            x = col * CELL_SIZE  # calculate x
            pygame.draw.line(self.line_surface, LINE_COLOR, (x, 0),
                             (x, self.surface.get_height()), 1)  # draw line

        for row in range(1, ROWS):  # draw horizontal lines
            y = row * CELL_SIZE  # calculate y
            pygame.draw.line(self.line_surface, LINE_COLOR,
                             (0, y), (self.surface.get_width(), y))  # draw line

        self.surface.blit(self.line_surface, (0, 0))  # blit line surface

    def input(self):  # handle input
        keys = pygame.key.get_pressed()  # get pressed keys

        # checking horizontal movement
        # check if horizontal move timer is active
        if not self.timers['horizontal move'].active:
            if keys[pygame.K_LEFT]:  # check if left key is pressed
                self.tetromino.move_horizontal(-1)  # move tetromino left
                # activate horizontal move timer
                self.timers['horizontal move'].activate()
            if keys[pygame.K_RIGHT]:  # check if right key is pressed
                self.tetromino.move_horizontal(1)  # move tetromino right
                # activate horizontal move timer
                self.timers['horizontal move'].activate()

        # check for rotation
        if not self.timers['rotate'].active:  # check if rotate timer is active
            if keys[pygame.K_UP]:  # check if up key is pressed
                self.tetromino.rotate()  # rotate tetromino
                self.timers['rotate'].activate()  # activate rotate timer

        # down speedup
        # check if down key is pressed
        if not self.down_pressed and keys[pygame.K_DOWN]:
            self.down_pressed = True  # set down pressed to True
            # update vertical move timer duration
            self.timers['vertical move'].duration = self.down_speed_faster

        # check if down key is not pressed
        if self.down_pressed and not keys[pygame.K_DOWN]:
            self.down_pressed = False  # set down pressed to False
            # update vertical move timer duration
            self.timers['vertical move'].duration = self.down_speed

    def check_finished_rows(self):  # check if there are any finished rows
        # get the full row indexes
        delete_rows = []  # create delete rows
        for i, row in enumerate(self.field_data):  # loop through rows
            if all(row):    # check if all blocks are filled
                delete_rows.append(i)  # append row index to delete rows

        if delete_rows:  # check if there are any delete rows
            for delete_row in delete_rows:
                # delete full rows
                for block in self.field_data[delete_row]:
                    block.kill()

                # move down blocks
                for row in self.field_data:  # loop through rows
                    for block in row:  # loop through blocks
                        if block and block.pos.y < delete_row:  # check if block is above the deleted row
                            block.pos.y += 1

            # rebuild the field data
            self.field_data = [
                [0 for x in range(COLUMNS)] for y in range(ROWS)]
            for block in self.sprites:
                # add block to field data
                self.field_data[int(block.pos.y)][int(block.pos.x)] = block

            # update score
            self.calculate_score(len(delete_rows))

    def run(self):
        # update
        self.input()
        self.timer_update()
        self.sprites.update()

        # drawing
        self.surface.fill(GRAY)
        self.sprites.draw(self.surface)

        self.draw_grid()
        self.display_surface.blit(
            self.surface, (PADDING, PADDING))  # blit surface
        pygame.draw.rect(
            self.display_surface,
            LINE_COLOR,
            self.rect,
            2,
            2)  # draw rect
