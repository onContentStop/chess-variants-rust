use crate::sdl_error::ToSdl;

use super::piece::Piece;
use lazy_static::lazy_static;
use regex::Regex;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::{image::LoadSurface, surface::Surface};
use std::{io::BufRead, num::ParseIntError, sync::Arc};

lazy_static! {
    static ref TXT_FILE_REGEX: Regex = Regex::new(r"\.txt$").unwrap();
}

pub struct PieceFactory<'tc> {
    pub piece_name: String,
    pub piece_movement: Vec<Vec<i32>>,
    // FIXME not an option
    pub texture: Arc<Texture<'tc>>,
}

enum State {
    Start,
    Name,
    LFMove,
    Move,
}

impl<'tc> PieceFactory<'tc> {
    pub fn new(
        file: &std::fs::DirEntry,
        texture_creator: &'tc TextureCreator<WindowContext>,
    ) -> Result<PieceFactory<'tc>, crate::Error> {
        let mut piece_name = String::new();
        let mut piece_movement: Vec<Vec<i32>> = vec![];
        let mut state = State::Start;
        for line in file_to_lines_iter(file.path())? {
            let orig_line = line?;
            let line = orig_line.trim();
            use State::*;
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            match state {
                Start => {
                    if line == "start_name" {
                        state = Name;
                    }
                }
                Name => {
                    piece_name = line.to_string();
                    state = LFMove;
                }
                LFMove => {
                    if line == "start_moves" {
                        state = Move;
                    }
                }
                Move => {
                    if line == "end_moves" {
                        break;
                    }
                    let parts: Result<Vec<i32>, ParseIntError> =
                        line.split_whitespace().take(3).map(|n| n.parse()).collect();
                    let parts = parts?;
                    piece_movement.push(parts);
                }
            }
        }
        let image_surface = Surface::from_file(
            TXT_FILE_REGEX
                .replacen(
                    std::env::current_dir()
                        .unwrap()
                        .join("chess_pieces")
                        .join(&file.file_name())
                        .to_str()
                        .unwrap(),
                    1,
                    ".png",
                )
                .as_ref(),
        )
        .sdl_error()?;
        let texture = texture_creator
            .create_texture_from_surface(image_surface)
            .sdl_error()?;

        Ok(PieceFactory {
            piece_name,
            piece_movement,
            texture: Arc::new(texture),
        })
    }
    pub fn build_piece(&self, team: u32, pos_horz: u32, pos_vert: u32) -> Piece<'tc> {
        let new_piece: Piece = Piece::new(team, pos_horz, pos_vert, self.texture.clone());
        new_piece
    }
}

fn file_to_lines_iter<P: AsRef<std::path::Path>>(
    file_name: P,
) -> std::io::Result<impl Iterator<Item = std::io::Result<String>>> {
    let f = std::fs::File::open(file_name)?;
    let reader = std::io::BufReader::new(f);
    Ok(reader.lines())
}