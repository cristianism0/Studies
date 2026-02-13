// -- Funções -- //
//
// funções são definidas usando fn nome
// os nomes são em snake case, nomes em minusculo separados por underlines
// os parametros PRECISAM ter a tipagem dos dados
// não existem parametros opicionais [[maybe_unused]], parametros dados precisam de argumentos
// mesmo assinalando valores, por exemplo:
// fn name (name: &str = "cristian") {} isto não vai funcionar
//
// para valores de fallback/default, precisariamos definir macros ou criar objetos/struct

//alem disso, os argumentos não podem ser nomeados, eles precisam estar nas posições corretas. se
//os parametros forem nome e cores, os argumentos precisam estar nessa ordem

// subescopos dentro de funções são dadas como expressões e também podem ser armazenadas em
// variáveis

fn say_hello(name: &str){

    println!("Hello {name}");

    // a diferença entre statements e funções é que statements não realizam retornos
    let x = 6;
    // literais possuem valores de retorno e podem ser armazenados em variáveis.

    let nome = "Cristian";
    //string literal de tipo &str
}

fn add_num(x: i32, y: i32) -> i32 {
    // não é necessário usar o return, como x + y é uma expressão, e é a última, este será o valor
    // retornado
     x + y
    //usamos um return se quisermos fazer um retorno mais cedo
}

fn escopes() {
    say_hello("Cristian");

    let subescopo = {
        let nome = "Cristian";
        let var_shadowing = true;
        99
        // essa expressão retorna 99 pois é a última expressão. Não precisa de ;
    };
    // o valor de subescopo é dados apena pelo retorno do bloco de código

    println!("{:?}", subescopo);  // -> ()
    
    // o valor () retornado pelo print é o que chamamos de unit time, basicamente uma tupla vazia 
    // então, sempre que tivermos uma função ou expressão, devemos dar o valor de return, como em
    // add_num(). se isso não acontecer, o unit time é um None ou Null
    
    let res = add_num(8,9);
    println!("{res}", );

}

fn convert_to_number(s: &str) -> i32 {
    // parse pega uma string e converte para o tipo que colocamos na função
    // unwrap serve como um exception, ele vai receber dois valores, um bom e um ruim
    // o bom será a conversão e o ruim um erro
    // aqui também poderiamos usar o closure 
    // (|s| s.parse::<i32>().unwrap())
    s.parse().unwrap()
}

fn double(n: i32) -> i32 {
    // poderiamos omitir essas funções e usar uma closure direto na aplicação
    // |n| n * 2
    n * 2
}

fn main(){

    let input = "10 12 15 48 9 9 556 2 3 333 48";

    // Vec é um array dinamico, aloca na Heap
    let values: Vec<i32> = input
        .split(' ')
        .map(convert_to_number)
        .map(double)
        //.map(|n| n * 2)
        .collect();
    //collect converte um iterador (output do map) em uma coleção (nova array)
    println!("{:?}", values)
}
