pub struct BoardSpace {
    pub horz_position: char,
    pub vert_position: i32,
    pub is_active: bool,
    pub available_to_move: bool,
    pub available_to_kill: bool,
    pub is_danger: bool,
}

impl BoardSpace {
    pub fn new(horz: char, vert: u32) -> Result<BoardSpace> {
        Ok(BoardSpace {
            horz_position: horz,
            vert_position: vert,
            is_active: true,
            available_to_move: false,
            available_to_kill: false,
            is_danger: false,
        })
    }

    pub fn reset_status(&mut self) {
        self.available_to_move = false;
        self.available_to_kill = false;
        self.is_danger = false;
    }
}