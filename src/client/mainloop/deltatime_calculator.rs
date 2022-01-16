use sdl2::TimerSubsystem;

pub struct DeltaTimeCalculator {
    cur: u32,
    prev: u32,
    timer_subsystem: TimerSubsystem,
}

impl DeltaTimeCalculator {
    pub fn new(timer_subsystem: TimerSubsystem) -> Self {
        return DeltaTimeCalculator {
            cur: timer_subsystem.ticks(),
            prev: 0,
            timer_subsystem,
        };
    }
}

impl DeltaTimeCalculator {
    pub fn tick(&mut self) {
        self.prev = self.cur;
        self.cur = self.timer_subsystem.ticks();
    }

    pub fn millis(&self) -> u32 {
        return self.cur - self.prev;
    }

    pub fn delta_time(&self) -> f32 {
        return (self.cur - self.prev) as f32 / 1000.0;
    }
}
