// -- Match -- //
// 
// O match é um controlador de fluxo de comparação
// ele vai comparar um valor com uma sequencia, praticamente uma cadeia if/elif/else
// mas uma diferença do match para o if é que o match aceita tipos diferentes de valores booleanos
// ele pode usar valores definidos em enum ou structs por exemplo
// 
// 
// podemos usar valores option para evitar problema de null pointers
// 
// -- If Let -- //
// If let é usado para não precisar escrever muito código de match, por exemplo:
// se precisarmos pegar um valor e ignorar o resto, ao invés de criar um match, usamos if let.

let config_max = (3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}

enum Coin {
    Centavos,
    Reais,
}

fn sum_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn match_coins(coin: Coin) -> u8 {
    match coin {
        Coin::Centavos => {println!("Sorte|"); 1},
        Coin::Reais => 100,
    }
}
// match tem uma palavra para considerar todas as outras possibilidades chamada other

let dice = 9;

match dice {
    1 => nada(),
    2 => outronada(),
    // se não quisermos usar o valor other, usamos um _
    other => oresto(other)
}

fn nada() {}
fn outronada(){}
fn oresto(num: u8) {}


fn main() {
    match_coins(Coin::Centavos);
    
    println!("{:?}", sum_one(Some(5)));
    
}