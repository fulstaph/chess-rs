pub struct Game {}

pub struct GameState {
    active_color: Color,
    castling_rights: CastlingRights,
    en_passant_target: Unit,
    half_move_clock: usize,
    full_move_number: number,
    status: Gamestate,
}

pub enum GameState {}

pub struct CastlingRights {
    white_king_side: bool,
    white_queen_side: bool,
    black_king_side: bool,
    black_queen_side: bool,
}
