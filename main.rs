#[derive(Debug)]
struct CadPessoa
{
    nome: String,
    idade: u32,
    cidade: String,
<<<<<<< HEAD
=======
    email: String,
    solteiro: boll,
>>>>>>> b1bc6de (git teste de nova feature)
}

fn main() 
{
    let pessoa_1 = CadPessoa
    {
        nome: String::from("Lur"),
        idade: 25,
        cidade: String::from("teste"),
<<<<<<< HEAD
=======
        email: String::from("lurckzs@gmail.com23"),
        solteiro: true,
>>>>>>> b1bc6de (git teste de nova feature)
    };
    
    println!("Todas as informções que temos em pessoa_1 é {:?}", pessoa_1);
    println!("teste 0001 para Marge");
    println!("Agora testando o Rebase, Vamo lá");
}
