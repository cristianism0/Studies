// Variáveis em rust são definidas e mortas dentro de escopos
// Tais escopos são definidos dentro das chaves {}

// Exceto valores const
// eles são trocados pelo valor atual harcoded dentro do binário
const SCREAMING_SNAKE: u32 = 1;

fn main() {

    // inicialização de variáveis é por let
    let name = "cristian";

    println!("My name is: {} ", name);

    // variáveis também pode ser definidas por shadowing

    let name = "cristiane";
    println!("My name is: {} ", name);

    //normalmente as variáveis são imutáveis a menos que atribuimos mut nelas

    let mut mutavel = "mutabilidade";
    
    mutavel = "mudou";
    
    println!("{}", mutavel);

    // normalmente é melhor manter tudo estrito 
    
    // escopos podem ser criados usados chaves
    {
        let criada_nesse_escopo = "criada";
    }


}

