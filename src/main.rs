fn main() {
    let num01 = 5.2;
    let num02 = 2.1;

    println!("{}", add(num01, num02));
    println!("{}", sub(num01, num02));
    println!("{}", mult(num01, num02));
    println!("{}", div(num01, num02));
}

fn add(num01: f64, num02: f64) -> f64{
    num01 + num02
}

fn sub(num01: f64, num02: f64) -> f64{
    num01 - num02
}

fn mult(num01: f64, num02: f64) -> f64{
    return num01 * num02;
}

fn div(num01: f64, num02: f64) -> f64{
    return num01 / num02;
}

/*
u -> Unsigned, which means, that the number can be '+', or neutral. (Integer)
i -> Signed, which means, the number can be '+', '-' or neutral. (Integer)
*/