use std::io;

fn verificar_senha(senha: &str) -> bool {
    // Critério 1: pelo menos 8 caracteres
    if senha.len() < 8 {
        return false;
    }

    // Critérios 2 e 3: conter número e letra maiúscula
    let mut tem_numero = false;
    let mut tem_maiuscula = false;

    for c in senha.chars() {
        if c.is_ascii_digit() {
            tem_numero = true;
        }
        if c.is_ascii_uppercase() {
            tem_maiuscula = true;
        }
    }

    tem_numero && tem_maiuscula
}

fn main() {
    loop {
        println!("Digite sua senha: ");

        let mut senha = String::new();
        io::stdin()
            .read_line(&mut senha)
            .expect("Falha ao ler a entrada");

        // Remove o \n no final da string
        let senha = senha.trim();

        if verificar_senha(senha) {
            println!("Senha válida! Acesso concedido.");
            break;
        } else {
            println!("Senha inválida! Tente novamente.");
        }
    }
}
