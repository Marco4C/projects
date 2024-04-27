const PONTOS_MAXIMOS: u32 = 100_000;

fn main() {
    let mut x: i32 = 5;
    println!("O valor de x é: {}", x);
    x = 6;
    println!("O valor de x é: {}", x);

    println!("const PONTOS MÁXIMOS {}", PONTOS_MAXIMOS);

    let espacos: &str = "   ";
    println!("{}", espacos);
    let espacos: usize = espacos.len();
    println!("{}", espacos);
}

//O let é imutavel sem o mut mas pode receber retorno de função em tempo de compilação, constantes não podem receber esses retornos
