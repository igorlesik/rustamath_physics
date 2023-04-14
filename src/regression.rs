//! Simple Symbolic Regression.
//!
//! (c) 2023 Igor Lesik
//! MIT license
//!
//! Find equation(s) based on input/output unit type and
//! input/output values.
use rustamath_mks::*;
use super::{find_equation_by_units, EQUATIONS};

mod fit;

/// Get list of equations that sutisfy specified input/output unit types
/// and fit to measured input/output values.
///
/// This is simple Symbolic Regression algorithm.
///
/// # Example
///
/// ```text
/// //use rustamath_physics::{find_equation_by_units, Equation, EquationMaker, EQUATIONS};
/// //use rustamath_mks::*;
/// //let ids = find_equation_by_units(&[TIME_UNIT], &[VELOCITY_UNIT]);
/// //let mut equation = (EQUATIONS[ids[0]].new)(&[3.0, 2.0]);
/// //assert_eq!(equation.run(&[10.0])[0], 3.0 + 2.0*10.0);
/// ```
pub fn find_equation(
    unit_inputs: &[MksUnit],
    unit_outputs: &[MksUnit],
    inputs: &[f64],
    outputs: &[f64]
) -> Vec<(usize, f64)>
{
    use std::thread;

    let ids: Vec<usize> = find_equation_by_units(unit_inputs, unit_outputs);

    let mut eqs: Vec<(usize, f64)> = Vec::new();

    thread::scope(|thread_scope| {
        let mut ths = Vec::new();

        for id in ids.iter() {
            let th = thread_scope.spawn(move || {
                (*id, goodness_of_fit(*id, inputs, outputs, &[]))
            });
            ths.push(th);
        }

        for th in ths {
            let id_with_fit = th.join().unwrap();
            eqs.push(id_with_fit);
        }
    });

    eqs.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    eqs
}

/// Return Reduced χ² Chi-squared goodness-of_fit value.
///
/// Return (χ²/degree_freedom), the fit is reasonably good when it is of order 1.0.
///
/// See [Reduced Chi-squared statistics](https://en.wikipedia.org/wiki/Reduced_chi-squared_statistic).
///
/// χ² = ∑ ((Oᵢ - fᵢ)/sigmaᵢ)²
/// where Oᵢ are individual observed values (measurements);
/// fᵢ(params) is predicted value of the model
/// with M parameters which are set to some reasonable trial value.
///
/// Note: when `ssigmas` input array lenght is 0, we **ASSUME** unweighted data when all sigma(i)=1.
///
/// William H. Press - Numerical recipes, the art of scientific computing.
/// Cambridge University Press (2007):
/// In some cases the uncertainties associated with a set of measurements are not
/// known in advance, and considerations related to χ² fitting are used to derive a value for sigma.
/// If we assume that all measurements have the same standard deviation, sigma(i)=sigma,
/// and that the model does fit well, then we can proceed by first assigning an arbitrary
/// constant sigma to all points, next fitting for the model parameters by minimizing χ²
/// and finally recomputing sigma^2=∑(Oᵢ - fᵢ)²/(N-M).
/// !!! Obviously, this approach prohibits an independent assessment of goodness-of-fit. !!!
///
pub fn goodness_of_fit(id: usize, inputs: &[f64], outputs: &[f64], ssigmas: &[f64]) -> f64
{
    let equation_builder = &EQUATIONS[id];
    let (out_params, cns_params, inp_params) = (equation_builder.params)();
    let (nr_out_params, nr_cns_params, nr_inp_params) = (out_params.len(), cns_params.len(), inp_params.len());

    assert!(ssigmas.is_empty() || ssigmas.len() == outputs.len());

    let nr_measurements = inputs.len() / nr_inp_params;
    assert_eq!(outputs.len() / nr_out_params, nr_measurements);

    let mut equation_constants: Vec<f64> = Vec::new();
    equation_constants.resize(nr_cns_params, 1.0);

    if nr_cns_params > 0 && nr_measurements >= nr_cns_params {
        // Find constant parameters of the equation
        fit::fit(equation_builder, inputs, outputs, &mut equation_constants,
            nr_measurements, nr_inp_params);
    }

    let mut equation = (equation_builder.new)(&equation_constants);

    let mut predictions: Vec<f64> = Vec::with_capacity(outputs.len());

    for i in 0..nr_measurements {
        let input_start_index = i * nr_inp_params;
        let input_end_index = input_start_index + nr_inp_params;
        let mut prediction = equation.run(&inputs[input_start_index..input_end_index]);
        predictions.append(&mut prediction);
    }

    assert!(nr_out_params == 1);//FIXME !!! XXX !!!

    let mut chi2: f64 = 0.0_f64;

    for i in 0..nr_measurements {
        let output_start_index = i * nr_out_params;
        //let output_end_index = output_start_index + nr_out_params;
        for j in 0..nr_out_params {
            let diff = outputs[output_start_index + j] - predictions[output_start_index + j];
            let sigma = if ssigmas.is_empty() { 1.0 } else { ssigmas[output_start_index + j] };
            chi2 += (diff * diff) / (sigma * sigma);
        }
    }

    let degrees_of_freedom = if nr_measurements > nr_cns_params { nr_measurements - nr_cns_params } else { 1 };

    // Reduced chi2
    chi2 /= degrees_of_freedom as f64;

    chi2
}

// cargo test --lib test_circle_vs_square -- --nocapture
#[cfg(test)]
#[test]
fn test_circle_vs_square() {
    use crate::*;

    if let Ok(nr_cores) = std::thread::available_parallelism() {
        println!("Number of available CPU cores for parallel execution: {}", nr_cores);
    }

    println!("\nDo 3 -> 18 which is close to circle perimeter 2*3.14*3\n");
    let eqs = find_equation(&[DISTANCE_UNIT], &[DISTANCE_UNIT], &[3.0], &[18.0]);

    for (i, eq) in eqs.iter().enumerate() {
        let equation_info = &EQUATIONS[eq.0];
        println!("#{}: fit = {:8.2} {}", i+1, eq.1, equation_info.desc);
    }

    let eq_index = get_equation_by_typeid(figure::circle::CirclePerimeter::params).unwrap();
    assert_eq!(eq_index, eqs[0].0);

    println!("\nNext do 3 -> 12.1 which is close to square perimeter 3*4\n");
    let eqs = find_equation(&[DISTANCE_UNIT], &[DISTANCE_UNIT], &[3.0], &[12.1]);

    for (i, eq) in eqs.iter().enumerate() {
        let equation_info = &EQUATIONS[eq.0];
        println!("#{}: fit = {:8.2} {}", i+1, eq.1, equation_info.desc);
    }

    let eq_index = get_equation_by_typeid(figure::rectangle::SquarePerimeter::params).unwrap();
    assert_eq!(eq_index, eqs[0].0);
}

// cargo test --lib test_sine_vs_square -- --nocapture
#[cfg(test)]
#[test]
fn test_sine_vs_square() {
    //use crate::physics::*;

    let inputs: [f64; 18] = [0.1, 0.2, 0.3, 0.5, 1.0, 1.1, 1.2, 1.3, 1.4, 1.6, 2.0, 2.4, 2.8, 3.2, 3.6, 4.0, 4.2, 4.4];
    let mut outputs = vec![0.0f64; 18];
    for (i, input) in inputs.iter().enumerate() {
        outputs[i] = 10.5 * (input*2.0f64 + 1.5f64).sin() + 3.3;
    }

    let eqs = find_equation(&[SCALAR_UNIT], &[SCALAR_UNIT], &inputs, &outputs);

    for (i, eq) in eqs.iter().enumerate() {
        let equation_info = &EQUATIONS[eq.0];
        println!("#{}: fit = {:8.2} {}", i+1, eq.1, equation_info.desc);
        //println!("  {}", params);
    }
/*
    let eq_index = get_equation_by_typeid(figure::circle::CirclePerimeter::params).unwrap();
    assert_eq!(eq_index, eqs[0].0);

    println!("\nNext do 3 -> 12.1 which is close to square perimeter 3*4\n");
    let eqs = find_equation(&[DISTANCE_UNIT], &[DISTANCE_UNIT], &[3.0], &[12.1]);

    for (i, eq) in eqs.iter().enumerate() {
        let equation_info = &EQUATIONS[eq.0];
        println!("#{}: fit = {:8.2} {}", i+1, eq.1, equation_info.desc);
    }

    let eq_index = get_equation_by_typeid(figure::rectangle::SquarePerimeter::params).unwrap();
    assert_eq!(eq_index, eqs[0].0);*/
}