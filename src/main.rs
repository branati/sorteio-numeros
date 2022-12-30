use rand::{thread_rng, Rng};
use clap::{App, Arg};
fn main() {
    let mut rng = thread_rng();

    let mut list_numbers: Vec<Vec<u32>> = Vec::new();
    let matches = App::new("Sorteio de dezenas. By BRaNaTi and ChatGPT. v1.0")
    .arg(
        Arg::with_name("vetor-elementos")
            .short("v")
            .long("vetor-elementos")
            .value_name("NÚMERO")
            .help("A quantidade de elementos do vetor")
            .takes_value(true),
    )
    .arg(
        Arg::with_name("quantidade-numeros")
            .short("q")
            .long("quantidade-numeros")
            .value_name("NÚMERO")
            .help("A quantidade de números sorteados")
            .takes_value(true),
    )
    .arg(
        Arg::with_name("range")
            .short("r")
            .long("range")
            .value_name("NÚMERO")
            .help("O intervalo de números que serão sorteados")
            .takes_value(true),
    )
    .get_matches();

    let vetor_elementos: u32 = matches
        .value_of("vetor-elementos")
        .unwrap_or("6")
        .parse()
        .unwrap();

    let quantidade_numeros: u32 = matches
        .value_of("quantidade-numeros")
        .unwrap_or("10")
        .parse()
        .unwrap();

    let range: u32 = matches
    .value_of("range")
    .unwrap_or("60")
    .parse()
    .unwrap();

    for _ in 0..quantidade_numeros {
        let mut numbers: Vec<u32> = Vec::new();
        let mut drawn_numbers: Vec<u32> = Vec::new();

        // continua sorteando números até que o vetor tenha 6 elementos
        while numbers.len() < vetor_elementos.try_into().unwrap() {
            let number: u32 = rng.gen_range(1, range+1); // gera um número aleatório de 1 a 60

            // verifica se o número já foi sorteado
            if !drawn_numbers.contains(&number) {
                drawn_numbers.push(number);
                numbers.push(number);
            }
        }

        numbers.sort();

        list_numbers.push(numbers);
    }

    println!("Os conjuntos de números sorteados são: {:?}", list_numbers);
}
