from pygame.time import get_ticks


class Timer:
    def __init__(self, duration, repeated=False, func=None):
        """
        Initialize a Timer instance.

        Args:
            duration (int): Duration of the timer in milliseconds.
            repeated (bool): Indicates whether the timer should repeat after each duration.
            func (callable): Function to be called when the timer's duration is reached.
        """

        self.duration = duration  # Set the duration of the timer
        self.func = func  # Set the function to be called when the timer's duration is reached
        self.repeated = repeated  # Set whether the timer should repeat

        self.active = False  # Initialize the timer as inactive
        self.start_time = 0  # Initialize the start time of the timer

    def activate(self):
        """Activate the timer and set the start time."""
        self.active = True  # Set the timer as active
        self.start_time = get_ticks()  # Set the start time to the current time

    def deactivate(self):
        """Deactivate the timer and reset the start time."""
        self.active = False  # Set the timer as inactive
        self.start_time = 0  # Reset the start time

    def update(self):
        """
        Update the timer's state and call the specified function if the duration is reached.

        If the timer is repeated, activate it again for the next cycle.
        """
        current_time = get_ticks()  # Get the current time

        # Check if the timer's duration is reached and it's active
        if current_time - self.start_time >= self.duration and self.active:
            if self.func and self.start_time != 0:
                self.func()  # Call the specified function if available

            self.deactivate()  # Deactivate the timer

            # If the timer is set to repeat, activate it again for the next
            # cycle
            if self.repeated:
                self.activate()
