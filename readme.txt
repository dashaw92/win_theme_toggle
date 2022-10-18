I wanted to be able to dynamically set the theme of my windows install based off a schedule.
There is software already out there to do this, but they're insanely bloated for what they do,
causing a noticeable lag whenever a rollover occurs. This runs instantly.

You can force the theme with either "dark" or "light" as arguments to the binary.
Running without arguments will load the config.toml (comes with a preset schedule)
and determine what theme should be picked based off the current time.

This is not a service, and attempting to run it as one will cause the config to
constantly be re-opened and read from disk. I might come back and change this in
the future, but for now, I just set it up as a task in the windows scheduler.