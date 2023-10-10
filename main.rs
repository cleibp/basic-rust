fn main() {
    // ARRAY
    println!("### ARRAY ###");
    let numbers = [10, 20, 30, 40];

    for number in &numbers {
        println!("{}", number);
    }
    println!();

    // MATRIZ
    println!("### MATRIZ ###");
    let mut matriz = vec![vec![0; 3]; 3];  // Declaração de uma matriz 3x3 de inteiros

    // Inicialização da matriz
    for i in 0..3 {
        for j in 0..3 {
            matriz[i][j] = i * 3 + j + 1;
        }
    }

    // Acesso aos elementos da matriz
    println!("Elementos da matriz:");
    for row in &matriz {
        println!("{}", row.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
    println!();
}
