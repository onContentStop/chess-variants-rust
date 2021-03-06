use std::rc::Rc;

use parking_lot::RwLock;
use sdl2::{event::Event, mouse::MouseButton, rect::Rect, render::WindowCanvas};

use crate::{chess_game::ChessGame, gfx::Widgety};

pub struct EventHandler<'tc, C> {
    chess_game: Rc<RwLock<ChessGame<'tc, C>>>,
    canvas: Rc<RwLock<WindowCanvas>>,
    widgets: Vec<Box<dyn Widgety>>,
    width: u32,
    height: u32,
}

impl<'tc, C> EventHandler<'tc, C> {
    pub fn new(
        chess_game: Rc<RwLock<ChessGame<'tc, C>>>,
        canvas: Rc<RwLock<WindowCanvas>>,
        widgets: Vec<Box<dyn Widgety>>,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            chess_game,
            canvas,
            widgets,
            width,
            height,
        }
    }

    pub fn handle_event(&mut self, event: &Event) -> Result<(), crate::Error> {
        match event {
            Event::RenderTargetsReset { .. } => {
                self.chess_game.write().render_board(
                    self.canvas.clone(),
                    self.width as u32,
                    self.height as u32,
                )?;
            }
            Event::Window { win_event, .. } => {
                if let sdl2::event::WindowEvent::SizeChanged(w, h) = win_event {
                    self.width = *w as u32;
                    self.height = *h as u32;
                    self.chess_game.write().render_board(
                        self.canvas.clone(),
                        self.width as u32,
                        self.height as u32,
                    )?;
                }
            }
            Event::MouseMotion { x, y, .. } => {
                self.chess_game.write().mouse_hover(x, y)?;
            }
            Event::MouseButtonDown { mouse_btn, .. } => {
                if *mouse_btn == MouseButton::Left {
                    self.chess_game.write().mouse_left_click()?;
                }
            }
            Event::MouseButtonUp {
                mouse_btn, x, y, ..
            } => {
                if *mouse_btn == MouseButton::Left {
                    let rect = Rect::new(0, 0, 0, 0);
                    if rect.contains_point((*x, *y)) {}
                }
            }
            _ => {}
        }
        for widget in &mut self.widgets {
            widget.handle_event(event)?;
        }
        Ok(())
    }

    pub fn draw_widgets(&self) -> Result<(), crate::Error> {
        for widget in &self.widgets {
            widget.draw(self.canvas.clone())?;
        }
        Ok(())
    }
}
