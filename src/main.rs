use std::env;
use std::io::{Read, BufReader, BufRead};
use std::env::args;
use std::fs::File;


fn main() {
    let mut q = 10;
    let argument = env::args().skip(1).collect::<Vec<String>>();

    if argument[0].contains("-") {
        let mut z = 2usize;
        q = argument[1].clone().parse().unwrap();
        for file in argument.clone() {
            let fopen = File::open(argument[z].clone()).unwrap();
            let mut reader = BufReader::new(fopen);
            for line in 0..q {
                println!("{:?}", reader.by_ref().lines().next())
            }
            z = z+1;
        }
    }
    let mut z = 0;
    for file in argument.clone() {
        let fopen = File::open(argument[z].clone()).unwrap();
        let mut reader = BufReader::new(fopen);
        for line in 0..q {
            println!("{:?}", reader.by_ref().lines().next())
        }
        z = z +1;
    }

}
