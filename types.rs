// Rust tem dois tipos de tipagem
// Escalares e Compostos


// -- Escalares (Scalar Types) -- //
// representam um único valor dentro de uma escala conhecida
// permitem a comparação com outros escalares

// -- Compostos (Compound Types) -- //
// servem para agregar múltiplos valores


// Inteiros
// Assim como o C, rust tem tipos de inteiros
// | bits | signed | unsigned |
// | ---- | ------ | -------- |
// | 8    | i8     | u8       |
// | 16   | i16    | u16      |
// | 32   | i32    | u32      |
// | 64   | i64    | u64      |
// | 128  | i128   | u128     |
// | arch | isize  | usize    |

// -- signed -- //
// o range de inteiros signed depende dos bits
// eles seguem a fórmula: -(2^[n-1]) até 2^[n-1] - 1
// ex: i8: varia de -128 até 127.

// -- unsigned -- //
// range de 0 até 2^[n] - 1
// ex: u8: 0 até 255
//
// valores unsigned são wraped, previnindo buffer overflows
//
// arch varia de acordo com a arquitetura: 32, 64 bits - em c é o size_t
//
//
// rust considera valores inteiros não tipados como i32.
// rust considera valores flutuantes não tipados como f64.

fn main(){

    let inteiro : u8 = 15;
    // podemos deixar mais legivel usando esses underlines
    let inteiro_grande: u32 = 199_999_999;
    let ponto_flutuante: f64 = 12.5;
    let boolean: bool = true;
    // char tem suporte a todo UTF8
    let charactere : char = 'a';

    let tupla = (5, true, 'a', 15.6);
    let lista = [1,2,3,4,5];

    //podemos printar elementos como:
    println!("{:?}", tupla);
    println!("{:?}", lista);

    // podemos selecionar o valor pelo index usando 0
    println!("{:?}", tupla.0);

    // podemos também 'desenvelopar' uma tupla

    let (a,b,c,d) = tupla;
    println!("Um dos valores da tupla: {}", a);
    
    // ou até alterar os valores dela - respeitando o tamanho e os tipos dos itens
    let mut tupla_mutavel = (0,1,2,3);
    tupla_mutavel.0 = 50;

    println!("Após mudar o valor da tupla : {:?}", tupla_mutavel);


    let array: [i32;3] = [1,2,3];
    //arrays só guardam itens iguais
    
    println!("Primeiro valor do array: {:?}", array[0]);

    //podemos também fazer cortes no array usando uma referência

    println!("Array cortado: {:?}", &array[0..2]);
}
