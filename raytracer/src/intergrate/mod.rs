mod tests {

    use rxmath::random::*;
    use image::*;

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

        simple_mc();

        unimplemented!();
    }
}