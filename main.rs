enum Cor {
    Vermelho,
    Verde,
    Azul,
    Amarelo,
    Laranja,
}

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

      // TRY CATCH
  println!("### TRY CATCH ###");
  /*
  if numero2 == 0.0 {
      println!("Divisão por zero não é permitida!");
  } else {
      let resultado = numero1 / numero2;
      println!("Resultado da divisão: {:.2}", resultado);
  }*/
  println!();


  // ENUM
  println!("### ENUM ###");
  let minha_cor = Cor::Azul;

  // Verificar o valor da variável enum e imprimir uma mensagem correspondente
  match minha_cor {
      Cor::Vermelho => println!("Minha cor favorita é vermelho."),
      Cor::Verde => println!("Minha cor favorita é verde."),
      Cor::Azul => println!("Minha cor favorita é azul."),
      Cor::Amarelo => println!("Minha cor favorita é amarelo."),
      Cor::Laranja => println!("Minha cor favorita é laranja."),
  }
}
