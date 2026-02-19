// -- Paths -- //
// Os caminhos em rust avisam ao compilador onde achar a função ou o código que estamos chamando
// de um determinado módulo. Temos algumas alternativas para esse cenário.
// Usar o caminho absoluto indo de crate::....::função ou utilizar palavras como self, super ou um identificador
// Todos eles precisam ser concatenados usados o separador ::


mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
    pub mod public_hosting {
        pub fn add_to_wailist() {}
    }
}

// dentro da função se precisarmos chaamar uma função de algum módulo, precisamos adicionar ao path
pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    
    //podemos usar o relativo, de acordo a origem da execução, embora seja preferivel o absolute por previnir alguns erros
}

// esse código não irá compilar porque o housing é privado e não podemos pegar funções de modulos privados.
// se executarmos essa função chamando pela hosting publica ainda vai gerar erros.
// deixar um modulo public não faz de seus componentes publicos também. poderemos acessar apenas o módulo.
// 
// para usar os componentes, eles devem ser publicos também.

// -- Relatividade com o Super -- //
// super é uma espécia de alias para .., ou seja, ele irá olhar para o pai.

mod back_of_house {
    pub struct Breakfast {
        pub toast; String,
        seasonal_fruit: String,
    }
    
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::fromm("peaches"),
            }
        }
    }
}
pub fn eat_at_restaurant() {
    
    // essas duas funções são praticamente equivalentes
    let mut janta = back_of_house::Breakfast::summer("Pão Francês");
    meal.toast = String::from("Pão doce");
    
    println!("I'd like {} toast please", meal.toast);
}