use std::io;

fn eh_par(numero: i32) -> bool {
    numero % 2 == 0
}

fn main() {
    // Jogador 1 escolhe "par" ou "ímpar"
    println!("Jogador 1, escolha 'par' ou 'ímpar': ");
    let mut escolha = String::new();
    io::stdin()
        .read_line(&mut escolha)
        .expect("Falha ao ler a entrada");
    let escolha = escolha.trim().to_lowercase();

    // Jogador 1 digita um número
    println!("Jogador 1, digite seu número: ");
    let mut entrada1 = String::new();
    io::stdin()
        .read_line(&mut entrada1)
        .expect("Falha ao ler a entrada");
    let num1: i32 = entrada1.trim().parse().expect("Digite um número válido");

    // Jogador 2 digita um número
    println!("Jogador 2, digite seu número: ");
    let mut entrada2 = String::new();
    io::stdin()
        .read_line(&mut entrada2)
        .expect("Falha ao ler a entrada");
    let num2: i32 = entrada2.trim().parse().expect("Digite um número válido");

    // Soma
    let soma = num1 + num2;
    println!("A soma dos números é: {}", soma);

    // Verificação par/ímpar
    let resultado_par = eh_par(soma);

    // Determinação do vencedor
    if (resultado_par && escolha == "par") || (!resultado_par && escolha == "ímpar") {
        println!("Jogador 1 venceu!");
    } else {
        println!("Jogador 2 venceu!");
    }
}
