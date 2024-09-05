pub fn criar_grid() -> Vec<String> {
    let mut game_grid = Vec::new();

    for i in 0..9 {
        game_grid.push((i + 1).to_string());
    }
    game_grid
}

pub fn vez_jogada(posicao: usize, game_grid: &mut Vec<String>, jogador_da_vez: &str) -> bool {
    game_grid[posicao - 1] = jogador_da_vez.to_string();

    if jogador_ganhou(game_grid, jogador_da_vez) {
        println!("Fim de jogo, {} ganhou!", jogador_da_vez);
        true
    } else if deu_velha(game_grid) {
        println!("Deu velha!");
        true
    } else {
        false
    }
} 

fn jogador_ganhou(game_grid: &Vec<String>, jogador_da_vez: &str) -> bool {
    checagem_coluna(game_grid, jogador_da_vez) ||
    checagem_linha(game_grid, jogador_da_vez) ||
    checagem_diagonal(game_grid, jogador_da_vez)
}

fn checagem_coluna(game_grid: &Vec<String>, selecao: &str) -> bool {
    (game_grid[0] == selecao && game_grid[3] == selecao && game_grid[6] == selecao) ||
    (game_grid[1] == selecao && game_grid[4] == selecao && game_grid[7] == selecao) ||
    (game_grid[2] == selecao && game_grid[5] == selecao && game_grid[8] == selecao)
}

fn checagem_linha(game_grid: &Vec<String>, selecao: &str) -> bool {
    (game_grid[0] == selecao && game_grid[1] == selecao && game_grid[2] == selecao) ||
    (game_grid[3] == selecao && game_grid[4] == selecao && game_grid[5] == selecao) ||
    (game_grid[6] == selecao && game_grid[7] == selecao && game_grid[8] == selecao)
}

fn checagem_diagonal(game_grid: &Vec<String>, selecao: &str) -> bool {
    (game_grid[0] == selecao && game_grid[4] == selecao && game_grid[8] == selecao) ||
    (game_grid[2] == selecao && game_grid[4] == selecao && game_grid[6] == selecao)
}

fn deu_velha(game_grid: &Vec<String>) -> bool {
    !game_grid.iter().any(|elemento| elemento.trim().parse::<i32>().is_ok())
}
