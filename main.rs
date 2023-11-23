use std::io;
use std::ptr;
use std::mem;

// Enum para representar cores
enum Cor {
    Vermelho,
    Verde,
    Azul,
    Amarelo,
    Laranja,
    SemCor,
}

// Função para realizar a divisão
fn realizar_divisao(numero1: i32, numero2: i32) -> Result<i32, &'static str> {
    if numero2 == 0 {
        Err("Divisão por zero não é permitida!")
    } else {
        Ok(numero1 / numero2)
    }
}

const TRUE: i32 = 1;
const PI: f64 = 3.14159265;

// Função de soma
fn soma(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {

  let nome = "Cleiton";
  let idade = 20;
  let sexo = "m";
  let peso = 70.5;
  let ativo = true;

  let mut val1 = 0;
  let mut val2 = 0;
  let mut adicao = 0;
  let mut subtracao = 0;
  let mut multiplicacao = 0;
  let mut divisao = 0;
  let mut modulo = 0;

  let mut idade_ternario = 0;
  let mut idade_if = 0;
  let mut dia = 0;

  let mut a = 0;
  let mut b = 0;
  let mut c = 0;

  // Ponteiro
  let mut ponteiro: *mut i32 = &mut a;

  let mut numero1 = 0;
  let mut numero2 = 0;
  let mut res = 0;

  // Comentários
  // Comentário de uma linha
  /* Comentário
     de várias linhas */

  // ESCREVER NA TELA
  println!("### ESCREVER NA TELA ###");
  println!("Olá Mundo");
  println!();

  // VARIÁVEIS
  println!("### VARIÁVEIS E TIPOS BÁSICOS ###");
  println!("Nome: {}", nome);
  println!("Idade: {}", idade);
  println!("Sexo: {}", sexo);
  println!("Peso: {}", peso);
  println!("Ativo: {}", ativo);
  println!();

  // CONSTANTE
  println!("### CONSTANTE ###");
  println!("PI: {}", PI);
  println!();

  // OPERACOES
  println!("#### OPERACOES ####");
  println!("Informe o valor 1: ");
  let mut input_val1 = String::new();
  std::io::stdin().read_line(&mut input_val1).expect("Falha ao ler a entrada");
  val1 = input_val1.trim().parse().expect("Falha ao converter para inteiro");

  println!("Informe o valor 2: ");
  let mut input_val2 = String::new();
  std::io::stdin().read_line(&mut input_val2).expect("Falha ao ler a entrada");
  val2 = input_val2.trim().parse().expect("Falha ao converter para inteiro");

  adicao = val1 + val2;
  subtracao = val1 - val2;
  multiplicacao = val1 * val2;
  divisao = val1 / val2;
  modulo = val1 % val2;

  println!("Soma: {}", adicao);
  println!("Subtracao: {}", subtracao);
  println!("Multiplicacao: {}", multiplicacao);
  println!("Divisao: {}", divisao);
  println!("Modulo: {}", modulo);
  println!();

  // TERNARIO
  println!("### TERNARIO ###");
  println!("Digite um número: ");
  let mut input_idade_ternario = String::new();
  std::io::stdin()
      .read_line(&mut input_idade_ternario)
      .expect("Falha ao ler a entrada");
  idade_ternario = input_idade_ternario
      .trim()
      .parse()
      .expect("Falha ao converter para inteiro");
  println!(
      "{}",
      if idade_ternario >= 18 {
          "Maior de idade"
      } else {
          "Menor de idade"
      }
  );
  println!();

  // IF ELSE IF ELSE
  println!("### IF ELSE IF ELSE ###");
  println!("Informe a idade: ");
  let mut input_idade_if = String::new();
  std::io::stdin().read_line(&mut input_idade_if).expect("Falha ao ler a entrada");
  idade_if = input_idade_if.trim().parse().expect("Falha ao converter para inteiro");

  if idade_if < 12 {
      println!("CRIANCA");
  } else if idade_if >= 12 && idade_if < 18 {
      println!("ADOLESCENTE");
  } else {
      println!("ADULTO");
  }
  println!();

  // SWITCH CASE
  println!("### SWITCH CASE ###");
  println!("Informe um numero 1 - 7 para semana: ");
  let mut input_dia = String::new();
  std::io::stdin().read_line(&mut input_dia).expect("Falha ao ler a entrada");
  dia = input_dia.trim().parse().expect("Falha ao converter para inteiro");

  match dia {
      1 => println!("Domingo"),
      2 => println!("Segunda"),
      3 => println!("Terça"),
      4 => println!("Quarta"),
      5 => println!("Quinta"),
      6 => println!("Sexta"),
      7 => println!("Sábado"),
      _ => println!("Valor não existente"),
  }
  println!();

  // REPEAT
  println!("### REPEAT ###");
  println!("Não tem REPEAT");
  println!();

  // DO WHILE
  println!("### DO WHILE ###");
  while a < 10 {
      println!("{}", a);
      a += 1;
  }
  println!();

  // WHILE
  println!("### WHILE ###");
  while b < 10 {
      println!("{}", b);
      b += 1;
  }
  println!();

  // FOR
  println!("### FOR ###");
  for c in 0..10 {
      println!("{}", c);
  }
  println!();

  // ARRAY
  println!("### ARRAY ###");
  let numbers = [10, 20, 30, 40];
  for &number in &numbers {
      println!("{}", number);
  }
  println!();

  // MATRIZ
  println!("### MATRIZ ###");
  let mut matriz: [[i32; 3]; 3] = [[0; 3]; 3];

  // Inicialização da matriz
  for i in 0..3 {
      for j in 0..3 {
          matriz[i][j] = i as i32 * 3 + j as i32 + 1;
      }
  }

  // Acesso aos elementos da matriz
  println!("Elementos da matriz:");
  for i in 0..3 {
      for j in 0..3 {
          print!("{} ", matriz[i][j]);
      }
      println!();
  }

  // FUNCAO
  println!("### FUNCAO ###");
  println!("Digite o valor 1: ");

  let mut input_m = String::new();
  io::stdin()
      .read_line(&mut input_m)
      .expect("Falha ao ler a entrada");
  let m: i32 = input_m.trim().parse().expect("Falha ao converter para inteiro");

  println!("Digite o valor 2: ");
  let mut input_n = String::new();
  io::stdin()
      .read_line(&mut input_n)
      .expect("Falha ao ler a entrada");
  let n: i32 = input_n.trim().parse().expect("Falha ao converter para inteiro");

  let resultado = soma(m, n);

  println!("A soma de {} e {} é igual a {}", m, n, resultado);
  println!();

  // PROCEDURE
  println!("### PROCEDURE ###");
  println!("Não tem PROCEDURE");
  println!("");

  // PONTEIRO
  println!("### PONTEIRO ###");

  // Alocação de memória dinamicamente para um inteiro
  let mut ponteiro: *mut i32 = unsafe { mem::transmute(Box::new(0)) };

  if ponteiro.is_null() {
      println!("Erro na alocação de memória.");
      return;
  }

  // Atribui um valor à variável apontada
  unsafe {
      ptr::write(ponteiro, 42);
  }

  // Imprime o valor da variável apontada
  unsafe {
      println!("Valor da variável apontada: {}", *ponteiro);
  }

  // Liberação da memória alocada
  unsafe {
      Box::from_raw(ponteiro);
  }

  println!();


  // TRY CATCH
  println!("### TRY ###");
  println!("Informe o valor 1 para o dividendo: ");

  let mut input_numero1 = String::new();
  io::stdin()
      .read_line(&mut input_numero1)
      .expect("Falha ao ler a entrada");
  let numero1: i32 = input_numero1.trim().parse().expect("Falha ao converter para inteiro");

  println!("Informe o valor 2 para o divisor: ");
  let mut input_numero2 = String::new();
  io::stdin()
      .read_line(&mut input_numero2)
      .expect("Falha ao ler a entrada");
  let numero2: i32 = input_numero2.trim().parse().expect("Falha ao converter para inteiro");

  match realizar_divisao(numero1, numero2) {
      Ok(res) => {
          println!("Resultado da divisão: {}", res);
      }
      Err(mensagem) => {
          println!("Ocorreu uma exceção: {}", mensagem);
      }
  }
  println!();

  // ENUM
  println!("### ENUM ###");
  let minha_cor = Cor::Azul;

  match minha_cor {
      Cor::Vermelho => println!("Minha cor favorita é vermelho."),
      Cor::Verde => println!("Minha cor favorita é verde."),
      Cor::Azul => println!("Minha cor favorita é azul."),
      Cor::Amarelo => println!("Minha cor favorita é amarelo."),
      Cor::Laranja => println!("Minha cor favorita é laranja."),
      Cor::SemCor => println!("Eu não tenho uma cor favorita."),
  }

}
