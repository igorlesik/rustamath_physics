//! Tests for linear motion with constant acceleration equations.
//!
//! (c) 2023 Igor Lesik
//! MIT license
//!
use crate::{find_equation_by_units, /*Equation, EquationMaker,*/ EQUATIONS};
use rustamath_mks::*;

#[test]
fn find_velocity_equation() {
    let ids = find_equation_by_units(&[TIME_UNIT], &[VELOCITY_UNIT]);
    assert_eq!(ids.len(), 1);

    let mut equation = (EQUATIONS[ids[0]].new)(&[/*v0*/3.0, /*a*/2.0]);

    let res = equation.run(&[10.0]);
    assert_eq!(res.len(), 1);

    // `v = v0 + a*t`
    assert_eq!(res[0], 3.0 + 2.0*10.0);
}