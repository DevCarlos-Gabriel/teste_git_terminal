#[derive(Debug)]
struct CadPessoa
{
    nome: String,
    idade: u32,
    cidade: String,
    email: String,
}

fn main() 
{
    let pessoa_1 = CadPessoa
    {
        nome: String::from("Lur"),
        idade: 25,
        cidade: String::from("Pensilvania"),
        email: String::from("lurckzs@gmail.com"),
    };
    
    println!("Todas as informções que temos em pessoa_1 é {:?}", pessoa_1);
}
