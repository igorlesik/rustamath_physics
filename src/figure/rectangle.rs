//! Rectangle perimeter and area.
//!
//! (c) 2023 Igor Lesik
//! MIT license
//!
use rustamath_mks::*;
use super::super::{EqParams, Equation, EquationMaker, ParamsUnit};

/// Perimeter of square
pub struct SquarePerimeter {
    /// Perimeter `P = 4*side`.
    pub perimeter: MksVal,
    /// Side
    pub side: MksVal,
}

impl Default for SquarePerimeter {
    fn default() -> Self {
        Self::new()
    }
}

impl SquarePerimeter {
    /// Parameters type
    pub const PARAMS: EqParams<1, 0, 1> = EqParams {
        out: [DISTANCE_UNIT], cns: [], inp: [DISTANCE_UNIT]};

    /// New
    pub fn new() -> SquarePerimeter {
        SquarePerimeter {
            perimeter: MksVal {val: 0.0, unit: DISTANCE_UNIT},
            side: MksVal {val: 0.0, unit: DISTANCE_UNIT},
        }
    }

    /// Calculate perimeter of square.
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_physics::figure::rectangle::SquarePerimeter;
    /// let mut square = SquarePerimeter::new();
    /// square.calc(3.0);
    /// assert_eq!(square.perimeter.val, 4.0 * 3.0);
    /// ```
    pub fn calc(&mut self, side: f64) {
        self.side.val = side;
        self.perimeter.val = 4.0 * side;
    }
}

impl EquationMaker for SquarePerimeter {
    fn params() -> ParamsUnit {
        (&Self::PARAMS.out, &Self::PARAMS.cns, &Self::PARAMS.inp)
    }

    fn make(_cns: &[f64]) -> Box<dyn Equation> {
        Box::new(SquarePerimeter::new())
    }
}

impl Equation for SquarePerimeter {
    fn run(&mut self, inp: &[f64]) -> Vec<f64> {
        self.calc(inp[0]);
        vec![self.perimeter.val]
    }
}

/// Area of square
pub struct SquareArea {
    /// Area `A = side*side`.
    pub area: MksVal,
    /// Side
    pub side: MksVal,
}

impl Default for SquareArea {
    fn default() -> Self {
        Self::new()
    }
}

impl SquareArea {
    /// Parameters type
    pub const PARAMS: EqParams<1, 0, 1> = EqParams {
        out: [AREA_UNIT], cns: [], inp: [DISTANCE_UNIT]};

    /// New
    pub fn new() -> SquareArea {
        SquareArea {
            area: MksVal {val: 0.0, unit: AREA_UNIT},
            side: MksVal {val: 0.0, unit: DISTANCE_UNIT},
        }
    }

    /// Calculate area of square.
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath_physics::figure::rectangle::SquareArea;
    /// let mut square = SquareArea::new();
    /// square.calc(3.0);
    /// assert_eq!(square.area.val, 3.0 * 3.0);
    /// ```
    pub fn calc(&mut self, s: f64) {
        self.side.val = s;
        self.area.val = s * s;
    }
}

impl EquationMaker for SquareArea {
    fn params() -> ParamsUnit {
        (&Self::PARAMS.out, &Self::PARAMS.cns, &Self::PARAMS.inp)
    }

    fn make(_cns: &[f64]) -> Box<dyn Equation> {
        Box::new(SquareArea::new())
    }
}

impl Equation for SquareArea {
    fn run(&mut self, inp: &[f64]) -> Vec<f64> {
        self.calc(inp[0]);
        vec![self.area.val]
    }
}