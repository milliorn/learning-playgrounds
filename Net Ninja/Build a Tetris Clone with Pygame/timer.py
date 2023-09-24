from pygame.time import get_ticks


class Timer:
    def __init__(self, duration, repeated=False, func=None):
        self.repeated = repeated  # repeat timer
        self.func = func  # function to call
        self.duration = duration  # duration of timer

        self.start_time = 0  # start time
        self.active = False

    def activate(self):  # activate timer
        self.active = True
        self.start_time = get_ticks()  # get start time

    def deactivate(self):  # deactivate timer
        self.active = False
        self.start_time = 0  # reset start time

    def update(self):
        current_time = get_ticks()  # get current time
        if current_time - self.start_time >= self.duration and self.active:  # check if timer is done

            # call a function
            if self.func and self.start_time != 0:
                self.func()

            # reset timer
            self.deactivate()

            # repeat the timer
            if self.repeated:
                self.activate()
