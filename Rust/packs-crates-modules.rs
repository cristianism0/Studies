// -- Crates -- //
// Crates é a menor quantidade de código que o rust considera por vez.
// Se usar o rustc ou código em um determinado arquivo, aquele arquivo pode ser chamado de caixa.
// As crates podem vir em sua forma binária ou em forma de biblioteca. 
// As binarias são as que podemos rodar usando o compilador ou cargo run e que geralmente executa a função main do arquivo.
// Bibliotecas não possuem função main e eles não são executaveis. Geralmente usamos crates quando queremos falar de biliotecas.
// 
// O crate root é o arquivo raiz que o rust compila: se usarmos cargo run ele geralmente executa o main.rs
// Um pacote é um conjunto de uma ou mais crates que provem um conjunto de funcionalidades. Um pacote contem um arquivo Cargo.toml que
// informa as dependencias para fazer as crates funcionarem. 
// 
// O cargo também é uma crate.
// 
// podemos criar um novo novo pacote com: cargo new novo-projeto && ls novo-projeto
// O arquivo cargo tem as linhas de execução do diretorio src. Por padrão, ele define o src/main.rs como root de execução.

// -- Use e Mod -- //
// Podemos declarar modulos na root crate usando: mod modulo. O compilador irá buscar dentro de src pelo nome modulo ou dentro de src/modulo/mod.rs.
// Se esses modulos estiverem em submodulos, ele deverão estar dentro de pastas de mesmo nome que o modulo pai.
// através da função de path :: podemos pegar códigos e funções de um modulo usando: crates::modulo::função
// 
// -- Private vs Publico -- //
// O código dentro de um modulo é privado pelos modulos do arquivo pai por padrão.
// Para deixar publico, usamos: pub mod modulo ao invés de apenas mod.
// Podemos usar a função use para definir caminhos de modulos ou de código:
// use crate::modulo:submodullo:função joga função dentro do meu path e agora eu consigo usar apenas função.
// 
// -- Agrupando codigos relacionados -- //
// Modulos nos deixam o código dentro das crates que ajudam a legibilidade a reuso. Além disso, nos permitem 
// controlar a privacidade de itens porque o codigo dentro de um módulo é privado (temos que usar o use com o modulo publico)
// 
// Exemplo: Restaurante
// 1. Criamos uma library usando cargo new restaurante --lib
// 2. dentro de src/lib.rs definimos o seguinte código:

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        
        fn seat_at_table() {}
    }
    
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// a tree seria a seguinte:
// crate
// └── front_of_house           -> modulo pai
//     ├── hosting              -> modulo filho
//     │   ├── add_to_waitlist
//     │   └── seat_at_table
//     └── serving              -> modulo filho
//         ├── take_order
//         ├── serve_order
//         └── take_payment