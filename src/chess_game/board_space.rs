use sdl2::pixels::Color;

use super::game_piece::GamePiece;

pub struct BoardSpace {
    pub horz_position: u32,
    pub vert_position: u32,
    pub game_pieces: Vec<GamePiece>,
    pub hovered: bool,
    pub is_active: bool,
    pub available_to_move: bool,
    pub available_to_kill: bool,
    pub is_danger: bool,
    pub color: Color,
}

impl BoardSpace {
    pub fn new(horz: u32, vert: u32, color: Color) -> Result<BoardSpace, crate::Error> {
        Ok(BoardSpace {
            horz_position: horz,
            vert_position: vert,
            game_pieces: vec![],
            hovered: false,
            is_active: true,
            available_to_move: false,
            available_to_kill: false,
            is_danger: false,
            color,
        })
    }

    #[allow(dead_code)]
    pub fn reset_status(&mut self) {
        self.available_to_move = false;
        self.available_to_kill = false;
        self.is_danger = false;
    }
}
