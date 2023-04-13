//! Linear motion with constant acceleration.
//!
//! (c) 2023 Igor Lesik
//! MIT license
//!
//! References:
//!
//! - <https://en.wikipedia.org/wiki/List_of_equations_in_classical_mechanics>
//!
use rustamath_mks::*;
use super::super::super::{EqParams, Equation, EquationMaker, ParamsUnit};

/// Velocity formula parameters type
pub const VELOCITY_EQ_PARAMS: EqParams<1, 2, 1> = EqParams {
    out: [VELOCITY_UNIT], cns: [VELOCITY_UNIT, ACCEL_UNIT], inp: [TIME_UNIT]};

/// Velocity equation
pub struct VelocityEquation {
    /// Velocity `v = v0 + at`.
    pub velocity: MksVal,
    /// Initial velocity
    pub initial_velocity: MksVal,
    /// Constant acceleration
    pub acceleration: MksVal,
    /// Time
    pub time: MksVal,
}

impl VelocityEquation {
    /// Parameters type
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_physics::mechanics::linear_motion::const_accel::VelocityEquation;
    /// use rustamath_mks::*;
    /// assert!(VelocityEquation::PARAMS.out == [VELOCITY_UNIT]);
    /// assert!(VelocityEquation::PARAMS.cns == [VELOCITY_UNIT, ACCEL_UNIT]);
    /// ```
    pub const PARAMS: EqParams<1, 2, 1> = VELOCITY_EQ_PARAMS;

    /// Initialize constants
    pub fn new(v0: f64, a: f64) -> VelocityEquation {
        VelocityEquation {
            velocity: MksVal {val: 0.0, unit: VELOCITY_UNIT},
            initial_velocity: MksVal {val: v0, unit: VELOCITY_UNIT},
            acceleration: MksVal {val: a, unit: ACCEL_UNIT},
            time: MksVal {val: 0.0, unit: TIME_UNIT},
        }
    }

    /// Calculate velocity by time with constant acceleration.
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_physics::mechanics::linear_motion::const_accel::VelocityEquation;
    /// let mut eq = VelocityEquation::new(2.0, 3.0);
    /// eq.calc(10.0);
    /// assert_eq!(eq.velocity.val, 32.0);
    /// ```
    pub fn calc(&mut self, t: f64) {
        self.time.val = t;
        self.velocity = self.initial_velocity +
            self.acceleration * self.time;
    }
}

impl EquationMaker for VelocityEquation {
    /// Get parameters type.
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_physics::mechanics::linear_motion::const_accel::VelocityEquation;
    /// use rustamath_physics::{Equation, EquationMaker};
    /// use rustamath_mks::*;
    /// let params = VelocityEquation::params();
    /// assert!(params.0 == &[VELOCITY_UNIT]);
    /// assert!(params.1 == &[VELOCITY_UNIT, ACCEL_UNIT]);
    /// ```
    fn params() -> ParamsUnit {
        (&Self::PARAMS.out, &Self::PARAMS.cns, &Self::PARAMS.inp)
    }

    /// Create new equation with constant parameters provided.
    fn make(cns: &[f64]) -> Box<dyn Equation> {
        Box::new(VelocityEquation::new(cns[0], cns[1]))
    }
}

impl Equation for VelocityEquation {
    /// Run equation with inputs provided.
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_physics::mechanics::linear_motion::const_accel::VelocityEquation;
    /// use rustamath_physics::{Equation, EquationMaker};
    /// let mut eq = VelocityEquation::make(&[2.0, 3.0]);
    /// let res = eq.run(&[10.0]);
    /// assert_eq!(res[0], 32.0);
    /// ```
    fn run(&mut self, inp: &[f64]) -> Vec<f64> {
        self.calc(inp[0]);
        vec![self.velocity.val]
    }
}

/// Velocity-by-distance formula parameters type
pub const VELOCITY_BY_DIST_EQ_PARAMS: EqParams<1, 2, 1> = EqParams {
    out: [VELOCITY_UNIT], cns: [VELOCITY_UNIT, ACCEL_UNIT], inp: [DISTANCE_UNIT]};

/// Velocity-by-distance equation
pub struct VelocityByDistEquation {
    /// Velocity `v = sqrt(v0^2 + 2*a*s)`.
    pub velocity: MksVal,
    /// Initial velocity
    pub initial_velocity: MksVal,
    /// Constant acceleration
    pub acceleration: MksVal,
    /// Distance
    pub distance: MksVal,
}

impl VelocityByDistEquation {
    /// Parameters type
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_physics::mechanics::linear_motion::const_accel::VelocityByDistEquation;
    /// use rustamath_mks::*;
    /// assert!(VelocityByDistEquation::PARAMS.out == [VELOCITY_UNIT]);
    /// assert!(VelocityByDistEquation::PARAMS.inp == [DISTANCE_UNIT]);
    /// ```
    pub const PARAMS: EqParams<1, 2, 1> = VELOCITY_BY_DIST_EQ_PARAMS;

    /// Initialize constants
    pub fn new(v0: f64, a: f64) -> VelocityByDistEquation {
        VelocityByDistEquation {
            velocity: MksVal {val: 0.0, unit: VELOCITY_UNIT},
            initial_velocity: MksVal {val: v0, unit: VELOCITY_UNIT},
            acceleration: MksVal {val: a, unit: ACCEL_UNIT},
            distance: MksVal {val: 0.0, unit: DISTANCE_UNIT},
        }
    }

    /// Calculate velocity by distance with constant acceleration.
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_physics::mechanics::linear_motion::const_accel::VelocityByDistEquation;
    /// let mut eq = VelocityByDistEquation::new(3.0, 4.0);
    /// eq.calc(2.0);
    /// assert_eq!(eq.velocity.val, (3.0*3.0f64 + 2.0*4.0*2.0f64).sqrt());
    /// ```
    pub fn calc(&mut self, d: f64) {
        self.distance.val = d;
        self.velocity = ((self.initial_velocity * self.initial_velocity) +
            (MksVal::new_scalar(2.0) * self.acceleration * self.distance)).sqrt();
    }
}

impl EquationMaker for VelocityByDistEquation {
    /// Get parameters type.
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_physics::mechanics::linear_motion::const_accel::VelocityByDistEquation;
    /// use rustamath_physics::EquationMaker;
    /// use rustamath_mks::*;
    /// let params = VelocityByDistEquation::params();
    /// assert!(params.0 == &[VELOCITY_UNIT]);
    /// assert!(params.2 == &[DISTANCE_UNIT]);
    /// ```
    fn params() -> ParamsUnit {
        (&Self::PARAMS.out, &Self::PARAMS.cns, &Self::PARAMS.inp)
    }

    /// Create new equation with constant parameters provided.
    fn make(cns: &[f64]) -> Box<dyn Equation> {
        Box::new(VelocityByDistEquation::new(cns[0], cns[1]))
    }
}

impl Equation for VelocityByDistEquation {
    /// Run equation with inputs provided.
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_physics::mechanics::linear_motion::const_accel::VelocityByDistEquation;
    /// use rustamath_physics::{Equation, EquationMaker};
    /// let mut eq = VelocityByDistEquation::make(&[3.0, 4.0]);
    /// let res = eq.run(&[2.0]);
    /// assert_eq!(res[0], (3.0*3.0f64 + 2.0*4.0*2.0f64).sqrt());
    /// ```
    fn run(&mut self, inp: &[f64]) -> Vec<f64> {
        self.calc(inp[0]);
        vec![self.velocity.val]
    }
}

/// Distance formula parameters type
pub const DISTANCE_EQ_PARAMS: EqParams<1, 2, 1> = EqParams {
    out: [DISTANCE_UNIT], cns: [VELOCITY_UNIT, ACCEL_UNIT], inp: [TIME_UNIT]};

/// Distance equation
pub struct DistanceEquation {
    /// Distance `s = v0*t + (a*t^2)/2`.
    pub distance: MksVal,
    /// Initial velocity
    pub initial_velocity: MksVal,
    /// Constant acceleration
    pub acceleration: MksVal,
    /// Time
    pub time: MksVal,
}

impl DistanceEquation {
    /// Parameters type
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_physics::mechanics::linear_motion::const_accel::DistanceEquation;
    /// use rustamath_mks::*;
    /// assert!(DistanceEquation::PARAMS.out == [DISTANCE_UNIT]);
    /// assert!(DistanceEquation::PARAMS.cns == [VELOCITY_UNIT, ACCEL_UNIT]);
    /// ```
    pub const PARAMS: EqParams<1, 2, 1> = DISTANCE_EQ_PARAMS;

    /// Initialize constants
    pub fn new(v0: f64, a: f64) -> DistanceEquation {
        DistanceEquation {
            distance: MksVal {val: 0.0, unit: DISTANCE_UNIT},
            initial_velocity: MksVal {val: v0, unit: VELOCITY_UNIT},
            acceleration: MksVal {val: a, unit: ACCEL_UNIT},
            time: MksVal {val: 0.0, unit: TIME_UNIT},
        }
    }

    /// Calculate distance by time with constant acceleration.
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_physics::mechanics::linear_motion::const_accel::DistanceEquation;
    /// let mut eq = DistanceEquation::new(2.0, 3.0);
    /// eq.calc(10.0);
    /// assert_eq!(eq.distance.val, (2.0 * 10.0) + (3.0 * 100.0)/2.0);
    /// ```
    pub fn calc(&mut self, t: f64) {
        self.time.val = t;
        self.distance =
            self.initial_velocity * self.time +
            (self.acceleration * self.time * self.time) / MksVal::new_scalar(2.0);
    }
}

impl EquationMaker for DistanceEquation {
    /// Get parameters type.
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_physics::mechanics::linear_motion::const_accel::DistanceEquation;
    /// use rustamath_physics::{Equation, EquationMaker};
    /// use rustamath_mks::*;
    /// let params = DistanceEquation::params();
    /// assert!(params.0 == &[DISTANCE_UNIT]);
    /// assert!(params.1 == &[VELOCITY_UNIT, ACCEL_UNIT]);
    /// ```
    fn params() -> ParamsUnit {
        (&Self::PARAMS.out, &Self::PARAMS.cns, &Self::PARAMS.inp)
    }

    /// Create new equation with constant parameters provided.
    fn make(cns: &[f64]) -> Box<dyn Equation> {
        Box::new(DistanceEquation::new(cns[0], cns[1]))
    }
}

impl Equation for DistanceEquation {
    /// Run equation with inputs provided.
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_physics::mechanics::linear_motion::const_accel::DistanceEquation;
    /// use rustamath_physics::{Equation, EquationMaker};
    /// let mut eq = DistanceEquation::make(&[2.0, 3.0]);
    /// let res = eq.run(&[10.0]);
    /// assert_eq!(res[0], (2.0 * 10.0) + (3.0 * 100.0)/2.0);
    /// ```
    fn run(&mut self, inp: &[f64]) -> Vec<f64> {
        self.calc(inp[0]);
        vec![self.distance.val]
    }
}

/// Distance-by-velocity formula parameters type
pub const DISTANCE_BY_VEL_EQ_PARAMS: EqParams<1, 2, 1> = EqParams {
    out: [DISTANCE_UNIT], cns: [VELOCITY_UNIT, VELOCITY_UNIT], inp: [TIME_UNIT]};

/// Distance-by-velocity equation
pub struct DistanceByVelEquation {
    /// Distance `s = t*(v0 + v)/2`.
    pub distance: MksVal,
    /// Initial velocity
    pub initial_velocity: MksVal,
    /// Final velocity
    pub final_velocity: MksVal,
    /// Time
    pub time: MksVal,
}

impl DistanceByVelEquation {
    /// Parameters type
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_physics::mechanics::linear_motion::const_accel::DistanceByVelEquation;
    /// use rustamath_mks::*;
    /// assert!(DistanceByVelEquation::PARAMS.out == [DISTANCE_UNIT]);
    /// assert!(DistanceByVelEquation::PARAMS.cns == [VELOCITY_UNIT, VELOCITY_UNIT]);
    /// ```
    pub const PARAMS: EqParams<1, 2, 1> = DISTANCE_BY_VEL_EQ_PARAMS;

    /// Initialize constants
    pub fn new(v0: f64, v: f64) -> DistanceByVelEquation {
        DistanceByVelEquation {
            distance: MksVal {val: 0.0, unit: DISTANCE_UNIT},
            initial_velocity: MksVal {val: v0, unit: VELOCITY_UNIT},
            final_velocity: MksVal {val: v, unit: VELOCITY_UNIT},
            time: MksVal {val: 0.0, unit: TIME_UNIT},
        }
    }

    /// Calculate distance by time with constant acceleration.
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_physics::mechanics::linear_motion::const_accel::DistanceByVelEquation;
    /// let mut eq = DistanceByVelEquation::new(2.0, 3.0);
    /// eq.calc(10.0);
    /// assert_eq!(eq.distance.val, (50.0)/2.0);
    /// ```
    pub fn calc(&mut self, t: f64) {
        self.time.val = t;
        self.distance =
            (self.initial_velocity + self.final_velocity) *
            self.time * MksVal::new_scalar(0.5);
    }
}

impl EquationMaker for DistanceByVelEquation {
    /// Get parameters type.
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_physics::mechanics::linear_motion::const_accel::DistanceByVelEquation;
    /// use rustamath_physics::{Equation, EquationMaker};
    /// use rustamath_mks::*;
    /// let params = DistanceByVelEquation::params();
    /// assert!(params.0 == &[DISTANCE_UNIT]);
    /// assert!(params.1 == &[VELOCITY_UNIT, VELOCITY_UNIT]);
    /// ```
    fn params() -> ParamsUnit {
        (&Self::PARAMS.out, &Self::PARAMS.cns, &Self::PARAMS.inp)
    }

    /// Create new equation with constant parameters provided.
    fn make(cns: &[f64]) -> Box<dyn Equation> {
        Box::new(DistanceByVelEquation::new(cns[0], cns[1]))
    }
}

impl Equation for DistanceByVelEquation {
    /// Run equation with inputs provided.
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_physics::mechanics::linear_motion::const_accel::DistanceByVelEquation;
    /// use rustamath_physics::{Equation, EquationMaker};
    /// let mut eq = DistanceByVelEquation::make(&[2.0, 3.0]);
    /// let res = eq.run(&[10.0]);
    /// assert_eq!(res[0], (50.0)/2.0);
    /// ```
    fn run(&mut self, inp: &[f64]) -> Vec<f64> {
        self.calc(inp[0]);
        vec![self.distance.val]
    }
}