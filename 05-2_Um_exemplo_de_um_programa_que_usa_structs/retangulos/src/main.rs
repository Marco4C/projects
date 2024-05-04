#[derive(Debug)]
struct Retangulo {
    comprimento: u32,
    largura: u32,
}

fn main() {
    let rect1 = Retangulo {
        comprimento: 50,
        largura: 30,
    };

    println!("A área do retângulo é {} pixels", area(&rect1));
    println!("rect1 é {:#?}", rect1);
}

fn area(retangulo: &Retangulo) -> u32 {
    retangulo.comprimento * retangulo.largura
}
