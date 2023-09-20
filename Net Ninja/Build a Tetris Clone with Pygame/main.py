from settings import *
from sys import exit


class Main:
    def __init__(self):
        pygame.init()
        pygame.display.set_caption('Tetris')
        self.clock = pygame.time.Clock()
        self.display = pygame.display.set_mode((WINDOW_WIDTH, WINDOW_HEIGHT))

    def run(self):
        while True:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    pygame.quit()
                    exit()

            self.display.fill(GRAY)
            pygame.display.update()
            self.clock.tick(60)


if __name__ == '__main__':
    main = Main()
    main.run()
