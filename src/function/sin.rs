//! Unit-less wave and trigonometry functions.
//!
//! (c) 2023 Igor Lesik
//! MIT license
//!
use rustamath_mks::*;
use super::super::{EqParams, Equation, EquationMaker, ParamsUnit};

/// Sine
pub struct Sine {
    /// `v = A*sin(speed*t + phase) + shift`.
    pub output: f64,
    /// Amplitude
    pub amplitude: f64,
    /// Speed
    pub speed: f64,
    /// Phase
    pub phase: f64,
    /// Shift
    pub shift: f64,
    /// Angle
    pub angle: f64,
}

impl Sine {
    /// Parameters type
    pub const PARAMS: EqParams<1, 4, 1> = EqParams {
        out: [SCALAR_UNIT], cns: [SCALAR_UNIT; 4], inp: [SCALAR_UNIT]};

    /// New
    pub fn new(amplitude: f64, speed: f64, phase: f64, shift: f64) -> Sine {
        Sine {
            output: 0.0,
            amplitude, speed, phase, shift,
            angle: 0.0,
        }
    }

    /// Calculate sine.
    pub fn calc(&mut self, angle: f64) {
        self.angle = angle;
        self.output = (angle*self.speed + self.phase).sin()*self.amplitude + self.shift;
    }
}

impl Equation for Sine {
    fn run(&mut self, inp: &[f64]) -> Vec<f64> {
        self.calc(inp[0]);
        vec![self.output]
    }
}

impl EquationMaker for Sine {
    fn params() -> ParamsUnit {
        (&Self::PARAMS.out, &Self::PARAMS.cns, &Self::PARAMS.inp)
    }

    fn make(cns: &[f64]) -> Box<dyn Equation> {
        Box::new(Sine::new(cns[0], cns[1], cns[2], cns[3]))
    }
}