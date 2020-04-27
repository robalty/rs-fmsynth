use std::f64::consts::PI;

pub struct Op {
    pub amt: f64,
    pub frequency: f64,
    pub freq_mult: f64,
    pub a: f64,
    pub d: f64,
    pub s: f64,
    pub r: f64,
}

impl Copy for Op { }

impl Clone for Op {
    fn clone(&self) -> Op {
        *self
    }
}


impl Op {
    pub fn new() -> Op {
        Op {
            amt: 0.0,
            frequency: 220.0,
            freq_mult: 1.0,
            a: 1.0,
            d: 1.0,
            s: 0.0,
            r: 0.0,
        }
    }

    pub fn get_sample(&self, input: f64, clock: f64) -> f64 {
        let vol = self.get_adsr(clock);
        ((2.0 * PI * self.frequency * self.freq_mult * clock) + (input * self.amt)).sin() * vol
    }

    pub fn set_amt(&mut self, newamt: f64) {
        self.amt = newamt;
    }

    pub fn get_adsr(&self, clock: f64) -> f64 {
        match clock {
            x if x < self.a => lerp(0.0, 1.0, (clock / self.a)),
            x if x < (self.a + self.d) => lerp(1.0, self.s, (clock - self.a) / self.d),
            _ => self.s,
        }
    }
}

pub struct OpArray {
    pub ops: [Op; 4],
    pub alg: i32,
    pub phase: f64,
    sample_rate: f64,
}


impl Copy for OpArray { }

impl Clone for OpArray {
    fn clone(&self) -> OpArray {
        *self
    }
}

impl OpArray {
    pub fn new() -> OpArray {
        OpArray {
            ops: [Op::new(); 4],
            alg: 1,
            phase: 0.0,
            sample_rate: 48000.0,
        }
    }

    pub fn get_phase(&self) -> f64 {
        self.phase
    }

    pub fn step(&mut self) {
        self.phase += 1.0 / self.sample_rate;
    }

    pub fn get_sample(&mut self) -> f64 {
        let clock = self.phase;
        match self.alg {
            1 => {
                let first = self.ops[0].get_sample(0.0, clock);
                let second = self.ops[1].get_sample(first, clock);
                let third = self.ops[2].get_sample(second, clock);
                self.ops[3].get_sample(third, clock)
            }
            2 => {
                let first = self.ops[0].get_sample(0.0, clock);
                let second = self.ops[1].get_sample(0.0, clock);
                let third = self.ops[2].get_sample(first + second, clock);
                self.ops[3].get_sample(third, clock)
            }
            _ => 0.0
        }
    }
}

//helpers
fn lerp(start:f64, end:f64, pct:f64)->f64{
    return start + ((end - start) * pct);
}
