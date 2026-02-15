// -- Borrowing -- //
// O processo de borrowing é analisando através do borrow-checker.
// Esse sistema vai procurar por esses erros:
// 1. Null Pointer Exception
// 2. Segmentation Fault 
// 3. Memory Leak
// 4. Dangling Pointers
// 5. Double Free
// 6. Use After Free
// 7. Data Races

// Borrow checker funciona em nível de função e esse é o ponto mais importante da gestão de memória
// do Rust

fn say_hello(text: &str) {
    println!("Hello {text}");
}

fn say_goodbye(text: &str) {
    println!("Goodbye {text}");
}

fn say_hello_heap(text: String) {
    println!("Hello {text}");
}

fn say_goodbye_heap(text: String) {
    println!("Goodbye {text}");
}


fn main() {
    let a: i32 = 1; // a variável a estara na memoria stack e podemos chamar o seu valor de Copy
    // Copy pq os valores dentro da stack quando são atribuídos a outras variáveis como no caso
    // abaixo, o Rust irá gerar uma cópia. Ao invés do ponteiro apontar para o a que está apontando
    // para o 1, o rust irá apontar para um valor 1 em outra alocação. Então a é independente de b.
    let b = a;

    println!("Valor de A é {}", a);
    println!("Valor de B é {}", b);

    //para evitar essa cópia eu preciso falar que B tem que apontar para o valor que A está
    //apontando, ou seja, B tem que receber a referências de A.
    
    let b = &a;
    // normalmente não precisamos deferenciar o ponteiro, o println já faz isso
    println!("O valor de B copiando A é {}", *b);

    // para introduzir o conceito de ownership vamos precisar usar a memória heap, aquela memória
    // dinamica que é alterada em tempo de execução

    let heap = String::from("Cristian");
    // nesse caso, a variável heap é DONA da string Cristian, ou seja, Cristian está em POSSE da
    // heap
    
    // Valores não primitivos (i32, f64, char, bool) NÃO são copy, ou seja, Rust não irá gerar
    // outro valor igual se fizermos uma atribuição assim

    //let emprestimo_borrow = heap;

    // B não pode apontar para o valor da string Cristian pois teriamos DUAS variáveis
    // apontando para o mesmo local e o compilador não saberia quem é responsável pela limpeza
    // Se ocorresse a limpeza, B apontaria para o Null ou seria um Dangling Pointer.
    
    // Se o Rust não copia nem Referencia, o que ele faz? Ele MOVE a propriedade para a outra
    // variável.
    // Isto é chamado de Move Semantics

    //println!("Valor de Heap é {}", heap);
    // se executarmos esse valor, heap não terá mais propriedade
    // emprestimo agora é dona da string cristian

    //println!("Valor de Emprestimo é {}", emprestimo_borrow);

    // para evitar esse valor, precisamos de um emprestimo, então, emprestimo precisa pegar uma
    // referencia de A. Valor agora é
    let emprestimo = &heap;

    println!("Valor de Heap é {}", heap);
    println!("Valor de emprestimo é {}", *emprestimo);

    // o emprestimo agora tem uma propriedade de borrowing de heap e consegue ler o valor da posse
    // de heap
    

    // -- Maximas do Rust -- //
    
    // 1. CADA valor possui UM dono, ou você faz emprestimo ou move -> Ownership
    // 2. Borrowing serve como uma referência para o DONO do valor
    // 3. Podemos ter UMA UNICA referencia se a variável for mutavel (&mut String - exemplo)
    // 4. Podemos ter várias se ela for imutavel (sem mut)
    
    let name = "Cristian";

    let name_heap = "Cristian".to_string(); // alocada na heap, sem copy

    say_hello(name);
    say_goodbye(name);

    //name é uma string literal e está na memória static, então ela será copiada para dentro da
    //função e executada. depois da execução, a copia sai do escopo e é descartada
   
    // say_hello(name_heap);
    // say_goodbye(name_heap);
    
    // isso irá gerar um erro pois name_heap é uma String e está na heap
    // como String não possui Copy, ela será movida para dentro da função em text
    // text agora é proprietário desse valor, portanto, quando sair do escopo o valor é limpo
    
    // então name ficaria apontando para nada
    
    // para evitar isso, a string possui uma propriedade clone
    
    say_hello_heap(name_heap.clone());
    // podemos deixar esse sem o clone pq ele vai morrer no escopo. Depois so não podemos chamar o
    // name_heap de novo
    // deixarei o clone para manter a variável
    say_goodbye_heap(name_heap.clone());

    //se não quisermos o clone, usamos a referencia (melhor) 
    // mas o parametro da função também precisa ser referencia &String
    // say_hello_heap(&name_heap);
    // say_goodbye_heap(&name_heap);

}
