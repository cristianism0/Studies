// -- Enums -- //
// Enum nos permite definir um tipo especificando suas possíveis variantes
// Runs tem uma Enum habilitada chamada Option que ou pode ser um valor ou nada.
// Veremos como funciona o pattern match em match

// Enum é usada para comparar se um valor A é um de um conjunto de valores que também tem C e D
// Exemplo: O IP atualmente tem duas variações, o IPV4 e o IPV6.
// CADA ip ou é um V4 ou um V6, não pode ser os dois. Mas os dois são IP.
enum IpAddrKind {
    // Pode atribuir QUALQUER valor
    V4(String),
    V6(String),
}

fn main() {
    // Para fazer o Enum usamos o Namespace::Variante
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    struct IpAddr {
        kind: IpAddrKind,
        addr: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        addr: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        addr: String::from("::1"),
    };

    // Usar um struct para construir esses valores é muito custoso. O enum já oferece uma função para atribuir dados diretamente na criação.
    let casa = IpAddrKind::V4("124.0.0.1");

    //Cada variante pode ter valores diferentes diferentes da struct.
    //Podemos ter IPV6 como String e IPV4 como uma tupla (u8,u8,u8,u8)

}
