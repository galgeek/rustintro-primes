#![feature(step_by)]
fn main() {
    primes(1000);
}

fn primes(n: usize) {
    let mut composite : Vec<_> = (0..n).map(|x| x % 2 == 0).collect();
    composite[2] = false;

    for prime in 3..n {
        if !composite[prime] {
            for m in (2*prime..n).step_by(prime) {
                composite[m] = true;
            }
        }
    }

    for i in 2..n {
        if !composite[i] {
            println!("{:?}", i);
        }
    }
}
