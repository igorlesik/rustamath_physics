#[test]
fn test_sine() {
    use crate::*;
    use rustamath_mks::*;

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

    let sine_eq_index = get_equation_by_typeid(function::wave::Sine::params).unwrap();
    assert_eq!(sine_eq_index, eqs[0].0);
}