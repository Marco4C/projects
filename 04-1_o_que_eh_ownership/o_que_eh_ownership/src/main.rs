fn main() {
    let s1 = entrega_valor(); // entrega_valor move o valor retornado
                              // para s1.

    println!("{}", s1);
    let s2 = String::from("texto"); // s2 entra em escopo.
    println!("{}", s2);

    let s3 = pega_e_entrega_valor(s2); // s2 é movido para dentro da função
                                       // println!("{}", s2);
    println!("{}", s2);
    println!("{}", s3);
    // pega_e_entrega_valor, que também
    // move o valor retornado para s3.
} // Aqui, s3 sai de escopo e é destruída. s2 sai de escopo, mas já foi movida,
  // então nada demais acontece. s1 sai de escopo e é destruída.

fn entrega_valor() -> String {
    // entrega_valor move o valor
    // retornado para dentro da função
    // que a chamou.

    let uma_string = String::from("olá"); // uma_string entra em escopo.

    uma_string // uma_string é retornada e movida
               // para a função que chamou
               // entrega_valor.
}

// pega_e_entrega_valor vai pegar uma String e retorná-la.
fn pega_e_entrega_valor(uma_string: String) -> String {
    // uma_string entra em
    // escopo.

    uma_string // uma_string é retornada e movida para a função que chamou
               // pega_e_entrega_valor.
}
