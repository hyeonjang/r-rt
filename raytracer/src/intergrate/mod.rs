mod tests {

    use rxmath::vector::*;
    use rxmath::random::*;
    use image::*;

    use crate::sample::*;

    fn pdf(x:vec3) -> f32 {
        return 1.0/(4.0*3.14);
    }

    fn integrate_squared_x() {
        let N = 1000000;
        let mut sum = 0.0;
        for _ in 0..N {
            let d = random_unit_sphere();
            let cos_squared = d.z*d.z;
            sum += cos_squared / pdf(d);
        }
        println!("I = {}", sum/N as f32);
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