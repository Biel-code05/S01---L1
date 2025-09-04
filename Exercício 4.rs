// Função para calcular a média ponderada
fn calcular_media(nota1: f64, nota2: f64, nota3: f64) -> f64 {
    let peso1 = 2.0;
    let peso2 = 3.0;
    let peso3 = 5.0;

    let soma_pesos = peso1 + peso2 + peso3;
    let media = (nota1 * peso1 + nota2 * peso2 + nota3 * peso3) / soma_pesos;
    
    media
}

fn main() {
    // Notas declaradas
    let nota1 = 7.5;
    let nota2 = 8.0;
    let nota3 = 9.2;

    // Calcula a média
    let media = calcular_media(nota1, nota2, nota3);

    // Exibe a média
    println!("A média ponderada é: {:.2}", media);

    // Verifica aprovação
    if media >= 7.0 {
        println!("Aluno Aprovado!");
    } else {
        println!("Aluno Reprovado!");
    }
}
