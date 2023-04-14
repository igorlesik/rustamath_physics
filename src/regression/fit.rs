//! Parametric fitting equation with input/output data.
//!
//! (c) 2023 Igor Lesik
//! MIT license
//!
//use super::super::{Equation};
use super::super::equations::{BuildTuple};

/// Fitting entry function
pub fn fit(
    builder: &BuildTuple,
    inputs: &[f64],
    outputs: &[f64],
    params: &mut [f64],
    nr_measurements: usize,
    nr_inp_params: usize
)
{
    if params.len() == 1 {
        panic!();
    }
    else {
        fit_multidimensions(builder, inputs, outputs, params, nr_measurements, nr_inp_params);
    }
}

fn fit_multidimensions(
    builder: &BuildTuple,
    inputs: &[f64],
    outputs: &[f64],
    params: &mut [f64],
    nr_measurements: usize,
    nr_inp_params: usize
)
{
    use rustamath_mnmz::amoeba;

    let fun_chi2 = |params_to_fit: &[f64]| {
        let mut chi2: f64 = 0.0_f64;
        #[allow(clippy::needless_range_loop)]
        for i in 0..nr_measurements {
            let input_start_index = i * nr_inp_params;
            let input_end_index = input_start_index + nr_inp_params;

            let mut equation = (builder.new)(params_to_fit);
            let prediction = equation.run(&inputs[input_start_index..input_end_index]);

            let diff = outputs[i] - prediction[0];
            chi2 += diff * diff;
        }
        //dbg!(chi2, params_to_fit);
        chi2
    };

    let (min, _fmin, _nriter) = amoeba(fun_chi2, params, 0.1, 1.0e-3, 150);
    params.copy_from_slice(&min);

    //dbg!((min, _fmin, _nriter));
}