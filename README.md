# calvichess
A Rust chess library for chess move generation/validation, piece placement/movement, and check/checkmate/draw detection

Instrucciones rsAjedrez


1.- Crea un tablero vacio - Creates an empty board 

> let mut board = ajedrez::Tablero::init();



2.- Inicializa una posición en el tablero - Initializes a position on the board 

a)  Posición inicial de partida – Initial starting position
> let fen_valida = ajedrez::setup_inicio(&mut board);
> retorna true o false. Si es true configura el tablero con esta posición - returns true or false. If true, configure the board with this position 

b) Una posición arbitraria - An arbitrary position 
> let fen =  "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
> let fen_valida = ajedrez::set_fen(fen, &mut board);
> retorna true o false. Si es true configura el tablero con esta posición
c) Comprueba si una FEN es válida - Check if a FEN is valid 
> let fen =  "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
> let fen_valida = valida_fen(fen);
> retorna true o false. Si es true la fen es válida; false en caso contrario



3.- Obtiene la fen actual – Get the current FEN

> let fen_actual = ajedrez::get_fen(&mut board);



4.- Efectúa un movimiento con una SAN – Executes a move with SAN notation

ajedrez::mueve_san(&mut board, movim);

> let movim = "Kb2";
> let result = ajedrez::mueve_san(&mut board, movim);
> result es una tupla con cinco elementos:
1. SAN  -> por ejemplo : result.0 = "Nf3"
2. UCI  -> por ejemplo: result.1 = "g1f3"
3. El bando que ha movido   -> por ejemplo: result,2 = "w"
4. La pieza movida. Si es mayusucula es una pieza blanca, en caso contrario es negra.
5. El tipo de jugada. Los "flags" son los siguientes :
  ORDINARY_MOVE   => "O",
  SHORT_CASTLE    => "S",
        LONG_CASTLE     => "L",
        EN_PASSANT      => "E",
        PROMOTION_QUEEN => "Q",
        PROMOTION_ROOK  => "R",
        PROMOTION_BISHOP => "B",
        PROMOTION_KNIGHT => "N",
> si el movimiento es erronero la tupla retornada será ("None", "None", "None", "None", "None")


5.- Efectúa un movim con coordenadas algebraicas. El formato es (origen, destino, promocion). Si no hay promoción no se tiene en cuenta.

Perform a movement with algebraic coordinates. The format is (origin, destination, promotion). If there is no promotion, it is not taken into account. 

ajedrez::mueve_algebra(&mut board, movim);

> let movim = ("e2", "e4", "Q");		// ("e2", "e4", "");
> let result = ajedrez::mueve_algebra(&mut board, movim);
> el resultado es lo mismo que con la SAN



6.- Deshacer movimiento. Retorna true si se ha deshecho correctamente el ultimo movimiento; retorna false en caso contrario.

Undo movement. Returns true if the last movement has been undone correctly; returns false otherwise. 

ajedrez::mueve_atras(&mut board);

> let resul = ajedrez::mueve_atras(&mut board);		// true o false



7.- Averiguar si está en jaque; retorna true o false - Find out if it is in check; returns true or false 

ajedrez::rey_en_jaque(&mut board);

> let mut board = ajedrez::Tablero::init();
> let fen = "2k5/8/3K4/2N2B2/8/8/8/8 b - - 0 1";
> let fen_valida = ajedrez::set_fen(fen, &mut board);
> let in_check = ajedrez::rey_en_jaque(&mut board);		//true



8.- Averiguar si el rey está ahogado. Retorna true o false - Find out if the king is stalemated. Returns true or false 

ajedrez::rey_ahogado(&mut board);

> let mut board = ajedrez::Tablero::init();
> let fen = "2k5/8/2NK4/2N5/8/8/8/8 b - - 0 1";
> let fen_valida = ajedrez::set_fen(fen, &mut board);
> let stalemated = ajedrez::rey_ahogado(&mut board);		// true



9.- Averiguar si el rey sufre jaque mate. Retorna true o false - Find out if the king is checkmate. Returns true or false 

ajedrez::rey_en_mate(&mut board);

> let mut board = ajedrez::Tablero::init();
> let fen = "2k5/8/2NK4/2N2B2/8/8/8/8 b - - 0 1";
> let fen_valida = ajedrez::set_fen(fen, &mut board);
> let mate_check = ajedrez::rey_en_mate(&mut board);		// true


10.- Mostrar el tablero formato texto. Retorna una cadena de texto - Show the text format board. Returns a text string 

> let _ascii = ajedrez::ascii(&mut board);		// to_string()
> println!("\n{}", _ascii);

          +-----------------+
//      8 | r   n  b   q  k   b   n   r  |
//      7 | p  p  p   p  -   p   p   p |
//      6 | -  -  -   -  -  -   -   -. |
//      5 | -  -  -   -  p  -   -   -  |
//      4 | -  -  -   -  P  P  -   -  |
//      3 | -  -  -   -  -   -  -   -  |
//      2 | P  P  P  P  -   -  P  P  |
//      1 | R  N  B  Q  K  B  N  R |
//        +-----------------+
             a   b   c   d   e   f   g  h



11.- Array del tablero . Util para usar en representacion grafica. Retorna un vector de string

Array of the board. Useful to use in graphic representation. Returns a string vector 

ajedrez::tablero_grafico(&mut board);

> let _grafico = ajedrez::tablero_grafico(&mut board);		//Vec<String>
> println!("148 -->\n{:?}", _grafico);

["r", "n", ...., "K"]



12.- Material insuficiente. Retorna true o false - Insufficient material Returns true or false 

ajedrez::material_insuficiente(&mut board);	

>   let mut board = ajedrez::Tablero::init();
>    let fen = "2k5/8/3K4/8/8/8/8/b1B5 b - - 0 1";
>    let fen_valida = ajedrez::set_fen(fen, &mut board);
>    println!("\n197 --> {}", ajedrez::material_insuficiente(&mut board));		//true




13.- Triple repetición de posición. Retorna true o false. Solo comprueba la ultima posición jugada.
Triple (threefold) repetition of position. Returns true or false. Just check the last position played. 

ajedrez::repeticion_triple(&mut board);

>    let mut board = ajedrez::Tablero::init();
>    let mut fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
>    let fen_valida = ajedrez::set_fen(fen, &mut board);
>    // primer movimiento completo
>    let mut movim = "Nf3";
>    let mut resul = ajedrez::mueve_san(&mut board, movim);
>    movim = "Nf6";
>    resul = ajedrez::mueve_san(&mut board, movim);
>    // segundo movimiento completo
>    let mut movim = "Ng1";
>    let mut resul = ajedrez::mueve_san(&mut board, movim);
>    movim = "Ng8";
>    resul = ajedrez::mueve_san(&mut board, movim);
>    // tercer movimiento completo
>    let mut movim = "Nf3";
>    let mut resul = ajedrez::mueve_san(&mut board, movim);
>    movim = "Nf6";
>    resul = ajedrez::mueve_san(&mut board, movim);
>    // cuarto movimiento completo
>    let mut movim = "Ng1";
>    let mut resul = ajedrez::mueve_san(&mut board, movim);
>    movim = "Ng8";
>    resul = ajedrez::mueve_san(&mut board, movim);
>    // quinto movimiento completo
>    let mut movim = "Nf3";
>    let mut resul = ajedrez::mueve_san(&mut board, movim);
>    movim = "Nf6";
>    resul = ajedrez::mueve_san(&mut board, movim);
>    
>    let mut triple = ajedrez::repeticion_triple(&mut board);    // true
>    println!("{}", triple);



14. ¿Fin de partida? - Retorna true o false
	Game over? - Returns true or false

ajedrez::game_over(&mut board);

>    let mut board = ajedrez::Tablero::init();
>    let mut fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
>    let mut fen_valida = ajedrez::set_fen(fen, &mut board);
>    let mut movim = "Nf3";
>    let mut resul = ajedrez::mueve_san(&mut board, movim);
>    let mut endgame = ajedrez::game_over(&mut board);
>    println!("250 --> {}", endgame);        //false

>    board = ajedrez::Tablero::init();
>    fen = "rnbqkbnr/ppppp2p/8/5ppQ/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 3"; //mate pos
>    fen_valida = ajedrez::set_fen(fen, &mut board);
>    endgame = ajedrez::game_over(&mut board);
>    println!("250 --> {}", endgame);        //true

>    board = ajedrez::Tablero::init();
>    fen = "4k3/4P3/4K3/8/8/8/8/8 b - - 0 78"; //ahogado /stalemate
>    fen_valida = ajedrez::set_fen(fen, &mut board);
>    endgame = ajedrez::game_over(&mut board);
>    println!("250 --> {}", endgame);        //true



15.- Obtiene pieza en la casilla solicitada. Retorna "None" si la casilla no existe, "-" si la casilla no tiene una pieza; y, en caso de tenerla, retorna la pieza en mayúsculas o minúsculas (dependiendo si es blanca o negra) --> "R" o "r".

Obtains piece in the requested square. Returns "None" if the square does not exist, "-" if the square does not have a piece; and, in case of having it, returns the piece in upper or lowercase (depending if it is white or black) -> "R" or "r". 

ajedrez::get_pieza(&mut board, "c5");

>   let mut board = ajedrez::Tablero::init();
>    let mut fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
>    let mut fen_valida = ajedrez::set_fen(fen, &mut board);
>    let pieza = ajedrez::get_pieza(&mut board, "c5");
>    println!("269 --> {}", pieza);  // "None, "-", "Q"



16.- Reproduce la partida en formato PGN. Retorna una String. Con el pgn o una cadena "None" caso de error.

ajedrez::pgn(&mut board);

>    let mut board = ajedrez::Tablero::init();
>    let mut fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
>    let mut fen_valida = ajedrez::set_fen(fen, &mut board);
>    let mut movim = ajedrez::mueve_san(&mut board, "e4");
>    movim = ajedrez::mueve_san(&mut board, "e5");
>    movim = ajedrez::mueve_san(&mut board, "Nf3");
>    movim = ajedrez::mueve_san(&mut board, "Nc6");
>    movim = ajedrez::mueve_san(&mut board, "Bb5");
>    movim = ajedrez::mueve_san(&mut board, "a6");
    
>    let pgn_txt = ajedrez::pgn(&mut board);

[FEN "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"]

1. e4 e5 2. Nf3 Nc6 3. Bb5 a6




17.- Jugadas posibles. Retorna un array con las jugadas legales posibles en formato UCI

Returns a vector of legal moves from the current position in UCI format.

ajedrez::jugadas_posibles(&mut board);

>  let mut board = ajedrez::Tablero::init();
>    let mut fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
>    let mut fen_valida = ajedrez::set_fen(fen, &mut board);
>    let mut jug_posibles = ajedrez::jugadas_posibles(&mut board);
>    println!("293 --> {:?}", jug_posibles);

["e2e4", ..... , "h2h4"]



18 .- Resetea el tablero y lo inicializa con la posición de inicio

Reset the board to the initial starting position.

ajedrez::reset();

>    let mut new_board = ajedrez::reset();
>    let new_fen = ajedrez::get_fen(&mut new_board);
>    println!("297 --> FEN: {}", new_fen);






