//! Equations of physics.
//!
//! (c) 2023 Igor Lesik
//! MIT license
//!
//! References:
//!
//! - <https://en.wikipedia.org/wiki/Lists_of_physics_equations>
//! - [Deep symbolic regression for physics guided by units constraints](https://arxiv.org/pdf/2303.03192.pdf)
//!
use rustamath_mks::MksUnit;

pub mod figure;
pub mod function;
pub mod mechanics;

mod equations;
pub use self::equations::{EQUATIONS};

mod regression;
pub use self::regression::find_equation;

/// Equation parameters
pub struct EqParams<const NR_OUT: usize, const NR_CONST: usize, const NR_IN: usize> {
    /// Output params
    pub out: [MksUnit; NR_OUT],
    /// Constant params
    pub cns: [MksUnit; NR_CONST],
    /// Input params
    pub inp: [MksUnit; NR_IN]
}

/// Parameters unit
pub type ParamsUnit = (&'static [MksUnit], &'static [MksUnit], &'static [MksUnit]);

/// Equation creation interface
pub trait EquationMaker {
    /// Return tuple with paramerts type
    fn params() -> ParamsUnit;

    /// Create new equation with provided constant parameters.
    fn make(cns: &[f64]) -> Box<dyn Equation>;
}

/// Equation interface
pub trait Equation {
    /// Run equation with provided input parameters.
    fn run(&mut self, inp: &[f64]) -> Vec<f64>;
}

/// Get list of equations that have specified input/output unit types.
///
/// # Example
///
/// ```
/// use rustamath_physics::{find_equation_by_units, Equation, EquationMaker, EQUATIONS};
/// use rustamath_mks::*;
/// let ids = find_equation_by_units(&[TIME_UNIT], &[VELOCITY_UNIT]);
/// let mut equation = (EQUATIONS[ids[0]].new)(&[3.0, 2.0]);
/// assert_eq!(equation.run(&[10.0])[0], 3.0 + 2.0*10.0);
/// ```
pub fn find_equation_by_units(inputs: &[MksUnit], outputs: &[MksUnit]) -> Vec<usize> {
    let mut eqs: Vec<usize> = Vec::new();

    for (index, eq) in EQUATIONS.iter().enumerate() {
        let (out, _cns, inp) = (eq.params)();
        if out == outputs && inp == inputs {
            eqs.push(index);
        }
    }
    eqs
}

/// Get index of equation in the list by (almost) any::TypeId.
///
/// TypeId::of is not stable const, for now we find index by function params() address.
///
/// # Example
///
/// ```
/// use rustamath_physics::*;
/// if let Some(eq_index) = get_equation_by_typeid(figure::circle::CirclePerimeter::params) {
///    // assert_eq!(eq_index, 0);
///    let mut equation = (EQUATIONS[eq_index].new)(&[]);
///    assert_eq!(equation.run(&[10.0])[0], 2.0 * std::f64::consts::PI * 10.0);
/// }
/// ```
#[allow(clippy::fn_address_comparisons)]
pub fn get_equation_by_typeid(typeid: fn () -> ParamsUnit) -> Option<usize> {
    for (index, eq) in EQUATIONS.iter().enumerate() {
        if typeid == eq.params {
            return Some(index);
        }
    }
    None
}