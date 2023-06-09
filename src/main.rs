use complex::{Fraction, Complex};

fn main()
{
    {
        let fraction_string = "-4 / 5";

        let a = Fraction::unchecked_from(10, 3);
        let b = fraction_string.parse::<Fraction>().unwrap();

        println!("\"{fraction_string}\" = {b}");
        println!("{a} + {b} = {}", a + b);

        println!();

        let float_value = 144.2;

        let c = Fraction::from_f64(float_value, 0.000000001);

        println!("{float_value} = {c}");
        println!("{c} = {}", c.to_f64());

        println!();

        let d = Fraction::unchecked_from(1, 4);
        let e = Fraction::unchecked_from(-4, 9);

        println!("sqrt({d}) = {}", d.sqrt());
        println!("sqrt({e}) = {}", e.sqrt());

        println!();
    }

    {
        let a = Complex::from_i32_pair(10, -4);
        let b = Complex::from_i32_pair(-1, 9);

        println!("({a}) - ({b}) = {}", a - b);

        let c = Complex::from_i32_pair(20, -4);
        let d = Complex::from_i32_pair(3, 2);

        println!("({c}) / ({d}) = {}", c / d);

        println!();

        let e = Complex::from_i32_pair(-3, 4);

        println!("|{e}| = {}", e.abs());

        let f = Complex::from_i32(5);
        let g = Complex::from_i32_pair(0, -3);

        println!("{f} * {g} = {}", f * g);
    }
}
