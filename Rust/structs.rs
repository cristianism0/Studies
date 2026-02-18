// -- Structs -- //
// Uma struct ou structure é um tipo dado que me permite reunir pacote e dados relacionados e criar um grupo comum
// Usando a linguagem de OOP, struct são os atributos de um objeto.
//
// Vamos verificar como podemos criar structs de uma forma mais refinada de organizar os dados.
//
// Structs tem relações com Enum para modificar dados e tipos de dados

// Struct são parecidas com Tuplas e ambas podem segurar valores relacioandos
// Para criar uma struct seguimos o seguinte padrão:
struct User {
    active: bool,
    username: String,
    email: String,
    sigin_in_count: u64,
}

// os valores dessa struct são chamadas de campo e definem como o User vai seguir o padrão.
// para usar uma structure precisamos criar uma nova instância de objeto
//assim como em OOP, struct possuem métodos que podem ser implementados usando impl Objeto{fn metodo(args){}}

// podemos construir uma função que tem como objetivo retornar um User, assim como um construtor em Java

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        email: email,
        username: username,
        sigin_in_count: 1,
    }
// não precisamos do return, essa é a última expressão da funçaõ e vai ser retornada
}

// Rust possui suporte para tuplas de struct
 struct Color(i32, i32, i32);
 struct Point(i32, i32, i32);

//Podemos criar Unit-Like structs = lembrando, unit são () o null do Rust
struct AlwaysEqual;

// Exemplo
#[derive(Debug)]
struct Retangulo {
    altura: u32,
    largura: u32,
}

//dentro de impl podemos criar funções associadas que retornar um Self e não usam &self
impl Retangulo{
    //assim como em String::new, podemos usar Retangulo::new(1,1) para criar uma nova instancia.
    fn new(altura, largura) -> Self {
        altura: altura,
        largura: largura,
    }

    //self é um alias para o struct que impl está se dirigindo
    // como não queremos pegar o ownership da instancia, faremos com &self
    fn area(&self) -> u32 {
        self.altura * self.largura
    }

    fn can_hold(&self, ret: &Retangulo) -> bool{
        //dentro DESTE, cabe OUTRO
        if self.altura > ret.altura && self.largura > ret.largura {
            return true;
        }
        false
    }
}


fn area(rectangle: &Retangulo) -> u32 {
    rectangle.altura * rectangle.largura
}


fn main() {
    // precisamos do igual e assinalamos os valores usando o :
    let mut user1 = User {
        active: true,
        username: String::from("Cristian"),
        email: String::from("email@email.com"),
        sigin_in_count: 1,
    };

    // para pegar o valor da estrutura usamos uma dot notation
    println!("Esse é o email: {}", user1.email);

    //se a instancia for mutavel (não o valor, a instancia completa), podemos fazer assim:
    user1.active = false;

    // usando o construtor
    let mut user2 = build_user(String::from("Novo"), String::from("Novo@email.com"));
    println!("Novo usuário feito com o build tem nome: {}", user2.username);

    // podemos nos apropriar de outros valores usando a sintaze `..`

    let mut user3 = User{
        active: false,
        ..user1
    };

    //se quisermos pegar valores especificos, precisamos deixar claro e usar a dot notation

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    // embora ambos os objetos tenham os mesmos valores
    //podemos desconstruir o struct da mesma forma que fazemos com as tuplas
    // Precisamos especificar o tipo de objeto que queremos descontruir

    let Point(x,y,z) = origin;
    //variávies x, y e z agora correspondem aos valores de origin

    // instancias dessa forma -> unitlike são usadas especialmente com traits que veremos mais tarde
    let subject = AlwaysEqual;

    let ret1 = Retangulo {
        altura: 30,
        largura: 20,
    };

    let area_ret1 = area(&ret1);

    let ret2 = Retangulo {
        altura: 50,
        largura: 10,
    };

    let ret3 = Retangulo {
        altura: 60,
        largura: 80,
    };

    // usar o ret1, uma instancia dentro da macro print invoca um metodo de display que não está diponível e gera um erro
    // para evitar, usamos a macro #[derive(Debug)] antes da struct
    println!(
        "A área do {ret1:?} é exatamete {} quadrados",
        area_ret1
    );

    println!(
        "Usando métodos, temos que a área é: {}",
        ret1.area()
    );

    println!(
        "O retangulo {ret1:?} cabe no {ret2:?}? {}",
        ret2.can_hold(&ret1)
    );

    println!(
        "O retangulo {ret3:?} cabe no {ret2:?}? {}",
        ret2.can_hold(&ret3)
    );

    println!(
        "O retangulo {ret2:?} cabe no {ret3:?}? {}",
        ret3.can_hold(&ret2)
    );


}
