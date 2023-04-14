#[test]
fn test_circle_vs_square() {
    use crate::*;
    use rustamath_mks::*;

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