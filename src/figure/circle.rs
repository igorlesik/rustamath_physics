//! Circle perimeter and area.
//!
//! (c) 2023 Igor Lesik
//! MIT license
//!
use rustamath_mks::*;
use super::super::{EqParams, Equation, EquationMaker, ParamsUnit};

/// Perimeter of circle
pub struct CirclePerimeter {
    /// Perimeter `s = 2*Pi*r`.
    pub perimeter: MksVal,
    /// Radius
    pub radius: MksVal,
}

impl Default for CirclePerimeter {
    fn default() -> Self {
        Self::new()
    }
}

impl CirclePerimeter {
    /// Parameters type
    pub const PARAMS: EqParams<1, 0, 1> = EqParams {
        out: [DISTANCE_UNIT], cns: [], inp: [DISTANCE_UNIT]};

    /// New
    pub fn new() -> CirclePerimeter {
        CirclePerimeter {
            perimeter: MksVal {val: 0.0, unit: DISTANCE_UNIT},
            radius: MksVal {val: 0.0, unit: DISTANCE_UNIT},
        }
    }

    /// Calculate circle perimeter.
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_physics::figure::circle::CirclePerimeter;
    /// let mut circle = CirclePerimeter::new();
    /// circle.calc(3.0);
    /// assert_eq!(circle.perimeter.val, 2.0 * std::f64::consts::PI * 3.0);
    /// ```
    pub fn calc(&mut self, r: f64) {
        self.radius.val = r;
        self.perimeter.val = 2.0 * std::f64::consts::PI * r;
    }
}

impl EquationMaker for CirclePerimeter {
    fn params() -> ParamsUnit {
        (&Self::PARAMS.out, &Self::PARAMS.cns, &Self::PARAMS.inp)
    }

    fn make(_cns: &[f64]) -> Box<dyn Equation> {
        Box::new(CirclePerimeter::new())
    }
}

impl Equation for CirclePerimeter {
    fn run(&mut self, inp: &[f64]) -> Vec<f64> {
        self.calc(inp[0]);
        vec![self.perimeter.val]
    }
}

/// Area of circle
pub struct CircleArea {
    /// Area `s = Pi*r^2`.
    pub area: MksVal,
    /// Radius
    pub radius: MksVal,
}

impl Default for CircleArea {
    fn default() -> Self {
        Self::new()
    }
}

impl CircleArea {
    /// Parameters type
    pub const PARAMS: EqParams<1, 0, 1> = EqParams {
        out: [AREA_UNIT], cns: [], inp: [DISTANCE_UNIT]};

    /// New
    pub fn new() -> CircleArea {
        CircleArea {
            area: MksVal {val: 0.0, unit: AREA_UNIT},
            radius: MksVal {val: 0.0, unit: DISTANCE_UNIT},
        }
    }

    /// Calculate circle area.
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_physics::figure::circle::CircleArea;
    /// let mut circle = CircleArea::new();
    /// circle.calc(3.0);
    /// assert_eq!(circle.area.val, std::f64::consts::PI * 3.0 * 3.0);
    /// ```
    pub fn calc(&mut self, r: f64) {
        self.radius.val = r;
        self.area.val = std::f64::consts::PI * r * r;
    }
}

impl EquationMaker for CircleArea {
    fn params() -> ParamsUnit {
        (&Self::PARAMS.out, &Self::PARAMS.cns, &Self::PARAMS.inp)
    }

    fn make(_cns: &[f64]) -> Box<dyn Equation> {
        Box::new(CircleArea::new())
    }
}

impl Equation for CircleArea {
    fn run(&mut self, inp: &[f64]) -> Vec<f64> {
        self.calc(inp[0]);
        vec![self.area.val]
    }
}