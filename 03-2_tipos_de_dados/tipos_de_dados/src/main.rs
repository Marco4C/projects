fn main() {
    let guess: u32 = "42".parse().expect("Não é um número!");

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // adição
    let soma = 5 + 10;

    // subtração
    let diferenca = 95.5 - 4.3;

    // multiplicação
    let produto = 4 * 30;

    // divisão
    let quociente = 56.7 / 32.2;

    // resto
    let resto = 43 % 5;

    let t = true;

    let f: bool = false; // com tipo explícito

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //println!("{}", tup)  `(i32, f64, u8)` cannot be formatted with the default formatter

    //Esse primeito programa cria uma tupla e vincula ela à variável tup.
    let tup = (500, 6.4, 1);
    //Em seguida, ele usa um padrão com let para tirar tup e tranformá-lo em três variáveis separadas, x, y e z. Isso é chamado de desestruturação
    let (x, y, z) = tup;

    println!("O valor do x é: {}", x);
    //O valor do x é: 500
    println!("O valor do y é: {}", y);
    // O valor do y é: 6.4
    println!("O valor do z é: {}", z);
    // O valor do z é: 1

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let quinhentos = x.0;
    println!("{}", quinhentos);
    let seis_ponto_quatro = x.1;
    println!("{}", seis_ponto_quatro);
    let um = x.2;
    println!("{}", um);

    let a = [1, 2, 3, 4, 5];

    let primeiro = a[0];
    let segundo = a[1];

    let meses = [
        "Janeiro",
        "Fevereiro",
        "Março",
        "Abril",
        "Maio",
        "Junho",
        "Julho",
        "Agosto",
        "Setembro",
        "Outubro",
        "Novembro",
        "Dezembro",
    ];

    let a = [1, 2, 3, 4, 5];
    let indice = 10;

    // let elemento = a[indice]; index out of bounds: the length is 5 but the index is 10

    //https://rust-br.github.io/rust-book-pt-br/ch03-02-data-types.html#acesso-inv%C3%A1lido-a-elemento-da-matriz
    //println!("O valor do elemento é: {}", elemento);
}
