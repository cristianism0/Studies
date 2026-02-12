// -- Espaços de Memória -- //
//
// Quando programamos em baixo nível é importante termos noção de espaços de memória.
// O rust utiliza de 3 espaços principais: Static, Stack e Heap.

// -- Static -- // 
// Quando um progama é executado, ele irá ocupar um espaço fixo de memória chamada de 
// memória estática. Aqui o próprio programa será armazenado (o binário), variáveis estáticas
// e strings literais. O compilador sabe o tamanho total durante o tempo de compilação.
// Quando o programa para de ser executado, o espaço de memória é limpo.

// -- Stack -- //
// Os dados escritos com tipagem primitiva (int, float, array, tuplas) são armazenadas na memória
// stack. O tamanho também é calculado pelo compilador, porém, o programa poderá colocar e tirar
// dados desse espaço de memória. Sendo assim, o tamanho do espaço é dinâmico.
// O espaço existe enquando ele está dentro do escopo de uma função, quando a função termina, as
// variáveis são limpas. Cada função executada, irá gerar um stackframe gerando uma pilha (stack).
// Caso o programa calcule errado o tamanho da stack ou o programa usa mais que o calculado, gera
// um stackoverflow.
// Como cada frame é de cada função, isso permite o variable shadowing. Quando uma função termina,
// o stackframe morre também.

// -- Heap -- //
// Vimos que ambas as memórias são calculadas durante o tempo de compilação, embora, em casos onde
// estamos em contato com alguma API ou que dependemos do IO de um usuário, não sabemos como será o
// input. Portanto, precisamos ter uma alocação de memória dinâmica para o tempo de execução.
// Esses valores podem viver mesmo após os escopos que estão definidos acabarem, além de poderem
// ser compartilhados em outras threads. Como podemos alongar as variáveis, o Rust oferece suporte
// para a limpeza manual, por Garbage Collector ou por RAII (escopo).


// static vai informar diretamente onde está localizado essa variável, melhorando o tempo de
// busca. por ser estático, podemos escrever fora do escopo.
static _Y: u32 = 15;

fn main() {
    let x = 5;
    let boolean = true;
    let array = [1,2,3,4];

}
