use std::io;
use std::io::Write;

pub fn imprimir_grid(valores_grid: &Vec<String>) {
    if valores_grid.len() != 9 {
        println!("Não há valores suficientes para formar o grid, {}", valores_grid.len());
        return;
    }

    let mut saida_grid = String::new();
    
    for (i, valor) in valores_grid.iter().enumerate() {
        saida_grid.push_str(&format!("|{:^3}", valor));

        if i % 3 == 2 {
            saida_grid.push_str("|\n-------------\n");
        }
    }
    println!("{}", saida_grid);
}

pub fn mostrar_jogada(simbolo_jogador: &str) {
    let mut stdout = io::stdout();
    print!("É a vez do jogador {}, escolha um lugar: ", simbolo_jogador);
    stdout.flush().expect("Não foi possível exibir a saída.");
}

pub fn ler_entrada() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())  // Remove trailing newline
}

pub fn interpretar_args() -> Result<usize, &'static str> {
    let entrada_jog = ler_entrada();

    match entrada_jog {
        Ok(arg_str) => {
            let args: Vec<&str> = arg_str.split_whitespace().collect();
            if args.len() != 1 {
                Err("Por favor, digite um valor entre 1 e 9")
            } else {
                let posicao = args[0].parse::<usize>();
                
                match posicao {
                    Ok(posicao) => {
                        if posicao < 1 || posicao > 9 {
                            Err("Valor não disponível... Digite um número entre 1 e 9")
                        } else {
                            Ok(posicao)
                        }
                    }
                    Err(_) => Err("Você não digitou um número...")
                }
            }
        }
        Err(_) => Err("Não foi possível ler a entrada, tente novamente...")
    }
}
