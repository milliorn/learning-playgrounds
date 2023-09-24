import pygame
from settings import BLOCK_OFFSET, CELL_SIZE, COLUMNS, ROWS


class Block(pygame.sprite.Sprite):
    def __init__(self, group, pos, color):

        # general
        super().__init__(group)  # add to group
        self.image = pygame.Surface((CELL_SIZE, CELL_SIZE))  # create image
        self.image.fill(color)  # fill with color

        # position
        self.pos = pygame.Vector2(pos) + BLOCK_OFFSET  # add offset
        self.rect = self.image.get_rect(
            topleft=self.pos * CELL_SIZE)  # create rect

    def rotate(self, pivot_pos):  # rotate around pivot point
        # rotate 90 degrees
        return pivot_pos + (self.pos - pivot_pos).rotate(90)

    def horizontal_collide(self, x, field_data):  # check horizontal collision
        if not 0 <= x < COLUMNS:  # check if in bounds
            return True

        if field_data[int(self.pos.y)][x]:  # check if block is there
            return True

    def vertical_collide(self, y, field_data):  # check vertical collision
        if y >= ROWS:  # check if in bounds
            return True

        if y >= 0 and field_data[y][int(
                self.pos.x)]:  # check if block is there
            return True

    def update(self):  # update position
        self.rect.topleft = self.pos * CELL_SIZE  # update rect
