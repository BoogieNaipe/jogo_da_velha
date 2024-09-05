mod game;
mod interface;

use game::vez_jogada;
use interface::{imprimir_grid, mostrar_jogada, interpretar_args};

fn run_prompt(game_grid: &mut Vec<String>, jogador_1: &str, jogador_2: &str) {
    let mut jogador_da_vez = jogador_1;
    let mut fim_de_jogo = false;

    while !fim_de_jogo {
        imprimir_grid(game_grid);
        mostrar_jogada(jogador_da_vez);

        let arg_interp_resultado = interpretar_args();

        match arg_interp_resultado {
            Ok(posicao) => {
                if game_grid[posicao - 1] == "X" || game_grid[posicao - 1] == "O" {
                    println!("Posição já escolhida, escolha outra...");
                    continue;
                }
                fim_de_jogo = vez_jogada(posicao, game_grid, jogador_da_vez);
                jogador_da_vez = if jogador_da_vez == jogador_1 { jogador_2 } else { jogador_1 };
            }
            Err(e) => {
                println!("Erro no Jogo: {}", e);
            }
        }
    }
}

fn main() {
    let jogador_1 = "X";
    let jogador_2 = "O";

    let mut game_grid = game::criar_grid();
    run_prompt(&mut game_grid, jogador_1, jogador_2);
}
