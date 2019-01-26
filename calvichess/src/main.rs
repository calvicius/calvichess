
mod ajedrez;


fn main(){
    
    // primero reseteamos todo el tablero y asignamos la fen de inicio
    let mut board = ajedrez::Tablero::init();
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let fen_valida = ajedrez::set_fen(fen, &mut board);
    let movim = ("e2", "e4", "");
    let mut res = ajedrez::mueve_algebra(&mut board, movim);
    println!("133 --> {} - {} - {} - {} -{}", res.0, res.1, res.2, res.3, res.4);
    let movim1 = ("e5");
    res = ajedrez::mueve_san(&mut board, movim1);
    println!("133 --> {} - {} - {} - {} -{}", res.0, res.1, res.2, res.3, res.4);
    
    // probamos mueve_algebra
    /*
    let mut board = ajedrez::Tablero::init();
    let mut fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    // true o false.
    let fen_valida = ajedrez::set_fen(fen, &mut board);
    println!("131 --> {}",ajedrez::rey_en_jaque(&mut board));
    let movim = ("e2", "e4", "");
    let res = ajedrez::mueve_algebra(&mut board, movim);
    println!("133 --> {} - {} - {}", res.0, res.1, res.2);
    // rey en jaque o jaque mate
    println!("135 --> {}",ajedrez::rey_en_jaque(&mut board));
    println!("136 --> {}",ajedrez::rey_ahogado(&mut board));
    println!("137 --> {}",ajedrez::rey_en_mate(&mut board));
    */
    
    /*
    let mut board = ajedrez::Tablero::init();
    let fen = "2k5/8/3K4/2N2B2/8/8/8/8 b - - 0 1";
    let fen_valida = ajedrez::set_fen(fen, &mut board);
    let in_check = ajedrez::rey_en_jaque(&mut board);
    println!("144 {}", in_check);
    let _ascii = ajedrez::ascii(&mut board);
    println!("146 -->\n{}", _ascii);
    let _grafico = ajedrez::tablero_grafico(&mut board);
    println!("148 -->\n{:?}", _grafico);
    */
    
    /*
    let mut board = ajedrez::Tablero::init();
    let mut fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let fen_valida = ajedrez::set_fen(fen, &mut board);
    println!("154 --> {}", ajedrez::get_fen(&mut board));
    let mut movim = "e4";
    let mut res = ajedrez::mueve_san(&mut board, movim);
    println!("156 --> {} - {} - {}", res.0, res.1, res.2);
    println!("157 --> {}", ajedrez::get_fen(&mut board));
    //ajedrez::mueve_atras(&mut board);
    //println!("160 --> {}", ajedrez::get_fen(&mut board));
    movim = "Nc6";
    res = ajedrez::mueve_san(&mut board, movim);
    println!("156 --> {} - {} - {}", res.0, res.1, res.2);
    println!("157 --> {}", ajedrez::get_fen(&mut board));
    let mut movim_t = ("d2", "d4", "");
    res = ajedrez::mueve_algebra(&mut board, movim_t);
    println!("167 --> {}", ajedrez::get_fen(&mut board));
    let in_check = ajedrez::rey_en_jaque(&mut board);
    println!("169 {}", in_check);
    let mut _ascii = ajedrez::ascii(&mut board);
    println!("171 -->\n{}", _ascii);
    let mut _grafico = ajedrez::tablero_grafico(&mut board);
    println!("173 -->\n{:?}", _grafico);
    let resul = ajedrez::mueve_atras(&mut board);
    println!("175 --> borrada: {}", resul);
    _ascii = ajedrez::ascii(&mut board);
    println!("177 -->\n{}", _ascii);
    */
    
    /*
    // tablero ascii
    let mut board = ajedrez::Tablero::init();
    let mut fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let fen_valida = ajedrez::set_fen(fen, &mut board);
    let mut movim_t = ("e2", "e4", "");
    let mut res = ajedrez::mueve_algebra(&mut board, movim_t);
    movim_t = ("d7", "d5", "");
    res = ajedrez::mueve_algebra(&mut board, movim_t);
    println!("187 --> {} - {} - {}", res.0, res.1, res.2);
    movim_t = ("f1", "b5", "");
    res = ajedrez::mueve_algebra(&mut board, movim_t);
    println!("187 --> {} - {} - {}", res.0, res.1, res.2);
    let mut _ascii = ajedrez::ascii(&mut board);
    println!("192 -->\n{}", _ascii);
    let resul = ajedrez::mueve_atras(&mut board);
    println!("175 --> borrada: {}", resul);
    _ascii = ajedrez::ascii(&mut board);
    println!("192 -->\n{}", _ascii);
    */
    
    /*
    // material insuficiente
    let mut board = ajedrez::Tablero::init();
    let fen = "2k5/8/3K4/8/8/8/8/b1B5 b - - 0 1";
    let fen_valida = ajedrez::set_fen(fen, &mut board);
    let mut _ascii = ajedrez::ascii(&mut board);
    println!("192 -->\n{}", _ascii);
    println!("\n197 --> {}", ajedrez::material_insuficiente(&mut board));
    */
    /*
    // triple repeticion
    let mut board = ajedrez::Tablero::init();
    let mut fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let fen_valida = ajedrez::set_fen(fen, &mut board);
    // primer movimiento completo
    let mut movim = "Nf3";
    let mut resul = ajedrez::mueve_san(&mut board, movim);
    movim = "Nf6";
    resul = ajedrez::mueve_san(&mut board, movim);
    // segundo movimiento completo
    let mut movim = "Ng1";
    let mut resul = ajedrez::mueve_san(&mut board, movim);
    movim = "Ng8";
    resul = ajedrez::mueve_san(&mut board, movim);
    // tercer movimiento completo
    let mut movim = "Nf3";
    let mut resul = ajedrez::mueve_san(&mut board, movim);
    movim = "Nf6";
    resul = ajedrez::mueve_san(&mut board, movim);
    // cuarto movimiento completo
    let mut movim = "Ng1";
    let mut resul = ajedrez::mueve_san(&mut board, movim);
    movim = "Ng8";
    resul = ajedrez::mueve_san(&mut board, movim);
    // quinto movimiento completo
    let mut movim = "Nf3";
    let mut resul = ajedrez::mueve_san(&mut board, movim);
    movim = "Nf6";
    resul = ajedrez::mueve_san(&mut board, movim);
    
    let mut triple = ajedrez::repeticion_triple(&mut board);    // true
    println!("{}", triple);
    */
    /*
    // game over ?
    let mut board = ajedrez::Tablero::init();
    let mut fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut fen_valida = ajedrez::set_fen(fen, &mut board);
    let mut movim = "Nf3";
    let mut resul = ajedrez::mueve_san(&mut board, movim);
    let mut endgame = ajedrez::game_over(&mut board);
    println!("250 --> {}", endgame);        //false
    board = ajedrez::Tablero::init();
    fen = "rnbqkbnr/ppppp2p/8/5ppQ/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 3"; //mate pos
    fen_valida = ajedrez::set_fen(fen, &mut board);
    endgame = ajedrez::game_over(&mut board);
    println!("250 --> {}", endgame);        //true
    
    board = ajedrez::Tablero::init();
    fen = "4k3/4P3/4K3/8/8/8/8/8 b - - 0 78"; //ahogado /stalemate
    fen_valida = ajedrez::set_fen(fen, &mut board);
    endgame = ajedrez::game_over(&mut board);
    println!("250 --> {}", endgame);        //true
    */
    /*
    // obtiene pieza en casilla solicitada
    let mut board = ajedrez::Tablero::init();
    let mut fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut fen_valida = ajedrez::set_fen(fen, &mut board);
    let pieza = ajedrez::get_pieza(&mut board, "c5");
    println!("269 --> {}", pieza);  // "None, "-", "Q"
    */
    /*
    // crea formato pgn
    let mut board = ajedrez::Tablero::init();
    let mut fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut fen_valida = ajedrez::set_fen(fen, &mut board);
    let mut movim = ajedrez::mueve_san(&mut board, "e4");
    movim = ajedrez::mueve_san(&mut board, "e5");
    movim = ajedrez::mueve_san(&mut board, "Nf3");
    movim = ajedrez::mueve_san(&mut board, "Nc6");
    movim = ajedrez::mueve_san(&mut board, "Bb5");
    movim = ajedrez::mueve_san(&mut board, "a6");
    
    let mut _ascii = ajedrez::ascii(&mut board);
    println!("192 -->\n{}", _ascii);
    let pgn_txt = ajedrez::pgn(&mut board);
    */
    /*
    // jugadas legales posibles a partir de la posición actual
    let mut board = ajedrez::Tablero::init();
    let mut fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut fen_valida = ajedrez::set_fen(fen, &mut board);
    let mut jug_posibles = ajedrez::jugadas_posibles(&mut board);
    println!("293 --> {:?}", jug_posibles);
    */
    /*
    let mut new_board = ajedrez::reset();
    let new_fen = ajedrez::get_fen(&mut new_board);
    println!("297 --> FEN: {}", new_fen);
    */
}



