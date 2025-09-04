use std::io;

// Função que imprime a tabuada
fn imprimir_tabuada(numero: i32, limite_inferior: i32, limite_superior: i32) {
    for i in limite_inferior..=limite_superior {
        println!("{} x {} = {}", numero, i, numero * i);
    }
}

fn main() {
    // Número da tabuada
    println!("Digite o número da tabuada: ");
    let mut entrada_num = String::new();
    io::stdin().read_line(&mut entrada_num).expect("Erro ao ler entrada");
    let numero: i32 = entrada_num.trim().parse().expect("Digite um número válido");

    // Limite inferior
    println!("Digite o limite inferior: ");
    let mut entrada_inf = String::new();
    io::stdin().read_line(&mut entrada_inf).expect("Erro ao ler entrada");
    let limite_inferior: i32 = entrada_inf.trim().parse().expect("Digite um número válido");

    // Limite superior
    println!("Digite o limite superior: ");
    let mut entrada_sup = String::new();
    io::stdin().read_line(&mut entrada_sup).expect("Erro ao ler entrada");
    let limite_superior: i32 = entrada_sup.trim().parse().expect("Digite um número válido");

    // Chama a função
    println!("\nTabuada do {} de {} até {}:", numero, limite_inferior, limite_superior);
    imprimir_tabuada(numero, limite_inferior, limite_superior);
}
