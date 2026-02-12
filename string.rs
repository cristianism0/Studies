use std::io;

fn main() {
    // Vimos que a memória static é a responsável por guardar strings literais e valores de texto
    // Como isso ocorre?
    //
    // Vamos supor que temos a string "A" que corresponde ao ASCII 65 que em binário seria 1000001
    // que equilave à F4241 em hexadecimal
    // Esse valor binário vai ser jogado junto com o código de programa dentro do static
    // Exemplo, irei escrever uma string e podemos verificar ela via terminal com o arquivo binário
    // rustc string.rs && hexdump string | grep "0x41"

    let letter = "A";

    // quando fazermos uma string grande
    // o tipo atribuído à variável é &str - string slice / string reference
    let string: &str = "Cristian";

    // quando fizemos o dump do hex vimos que o valor estava completamente desordenado, quando
    // trabalhamos com string, usamos o *pointer* &str para saber os seguintes valores e buscá-los
    // na static: [início, length].
    
    // a variável string morre quando o escopo acaba, mas o valor &str ficará na static até o
    // programa acabar
    
    // Se precisarmos usar uma variável string que seja dinâmica, precisamos que a memória
    // referênciada pela variável também seja dinâmica, principalmente em progamas de IO. Portanto,
    // precisamos alocar dentro da Heap. Chamamos essa string de String dinâmica ou Heap String.
    
    let mut s = String::new();
    s.push('C');
    s.push('r');
    s.push('i');
    s.push('s');
    s.push('t');
    s.push('i');
    s.push('a');
    s.push('n');
    s.push_str(" ");
    s.push_str("dentro da Heap");
 
    println!("{s}");

    let forma1: String = "Formas e Formas".to_string();
    let forma2: String = String::from("Formas e Formas");
    
    // into() tenta converter o valor à tipagem dada.
    let forma4: String = "Bruno".into();

    //lembrando, se caso não seja necessária fazer essa adoção de heap, usamos o &str


    let mut name = String::new();
    println!("Digite o seu nome: ");
    io::stdin()
        .read_line(&mut name)
        .expect("Error reading console.");

    // o len() também captura caracteres especiais como o /r do enter, então, usamos o trim.
    // entretando, len retorna o tamanho em bytes, então, valores como emojis que ocupam 4 bytes na
    // utf8 retorna len = 4 cada.
    println!("Olá {name}\nSeu nome possui {} letras.\n",  name.trim().len());

    //.chars vai ser um len so que retornar os chars em um array, usamos o count para pegar esse
    //valor.
    println!("Olá {name}\nSeu nome possui {} chars.\n",  name.trim().chars().count());
 
    println!("Olá {name}.\nAqui está seu nome em maísculo {}\n",  name.to_uppercase());

    println!("Olá {name}\nAqui está seu nome sem a vogal 'a' : {}\n",  name.trim().replace("a", ""));

    println!("{}", "arara".repeat(5));

    println!("{:-^30}", "REPETIÇÕES");


    let banner = 
        "Este é um texto de múltiplas linhas   
        Está já é a segunda linha deste texto.
        ";
    println!("{banner}")
}
