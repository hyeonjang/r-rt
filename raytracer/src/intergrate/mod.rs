mod tests {

    use rxmath::vector::*;
    use rxmath::random::*;
    use image::*;

    fn pdf(x:f64) -> f64 {
        return 0.5*x;
    }

    fn integrate_squared_x() {
        let N = 1000000;
        let mut sum = 0.0;
        for _ in 0..N {
            let x = (random_range_f64(0.0, 4.0)).sqrt();
            sum += x*x / pdf(x);
        }
        println!("I = {}", sum/N as f64);
    }

    fn simple_mc() {
        let N = 1000000;
        let mut inside_circle = 0;
        for _ in 0..N {
            let x = random_range_f64(-1.0, 1.0);
            let y = random_range_f64(-1.0, 1.0);
            if x*x + y*y<1.0 {
                inside_circle += 1;
            } 
        }
        println!("{}", (4.0*inside_circle as f64)/N as f64);
    }

    #[test]
    fn name() {

        integrate_squared_x();
        //simple_mc();

        unimplemented!();
    }
}