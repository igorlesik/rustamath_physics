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
#[cfg(test)]
mod test_sine;
#[cfg(test)]
mod test_circle;

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