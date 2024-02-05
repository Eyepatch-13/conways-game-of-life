use ggez::conf::WindowMode;
use ggez::event::EventHandler;
use ggez::graphics::{Color, Mesh, Rect, DrawParam};
use ggez::input::keyboard::{KeyInput, KeyCode};
use ggez::mint::Point2;
use ggez::{Context, GameResult, event, ContextBuilder, GameError, graphics};
use ggez::glam::*;

const CELL_SIZE:(f32, f32) = (20.0, 20.0);
const GRID_SIZE:(f32, f32) = (40.0, 40.0);
const WINDOW_SIZE: (f32, f32) = (CELL_SIZE.0 * GRID_SIZE.0, CELL_SIZE.1 * GRID_SIZE.1);

struct MainState {
    grid: Vec<Vec<bool>>,
    fps: u32,
    is_running: bool,
}

impl MainState {
    fn new() -> Self {
        MainState {
            grid: vec![vec![false; GRID_SIZE.1 as usize]; GRID_SIZE.0 as usize],
            fps: 1,
            is_running: false,
        }
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        while ctx.time.check_update_time(self.fps) && self.is_running {
            let mut coords: Vec<(usize, usize)> = vec![];

            for i in 0..GRID_SIZE.0 as usize {
                let left = if i > 0 { i - 1 } else { GRID_SIZE.0 as usize - 1 };
                let right = if i < GRID_SIZE.0 as usize - 1 { i + 1 } else { 0 };
                for j in 0..GRID_SIZE.1 as usize {
                    let up = if j > 0  { j - 1 } else { GRID_SIZE.1 as usize - 1};
                    let down = if j < GRID_SIZE.1 as usize - 1 { j + 1 } else { 0 };

                    let neighbors = self.grid[left][j] as u8
                        + self.grid[left][up] as u8 + self.grid[i][up] as u8
                        + self.grid[right][up] as u8 + self.grid[right][j] as u8
                        + self.grid[right][down] as u8 + self.grid[i][down] as u8
                        + self.grid[left][down] as u8;

                    if self.grid[i][j] && (neighbors < 2 || neighbors > 3) {
                        coords.push((i, j));
                    } else if !self.grid[i][j] && neighbors == 3 {
                        coords.push((i, j));
                    }
                }
            }

            for coord in coords {
                self.grid[coord.0][coord.1] ^= true;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        for i in 0..GRID_SIZE.0 as usize {
            for j in 0..GRID_SIZE.1 as usize {
                if self.grid[i][j] {
                    let rect = Mesh::new_rectangle(ctx,
                                               graphics::DrawMode::fill(),
                                               Rect::new(
                                                   i as f32 * CELL_SIZE.0
                                                   , j as f32 * CELL_SIZE.1
                                                   , CELL_SIZE.0
                                                   , CELL_SIZE.1),
                                               Color::BLACK)?;
                    canvas.draw(&rect, DrawParam::default());
                }
                if j == 0 {
                    continue;
                }
                let line = Mesh::new_line(ctx,
                                          &vec![Point2 { x: 0.0, y: j as f32 * CELL_SIZE.1},
                                                Point2 { x: WINDOW_SIZE.0, y: j as f32 * CELL_SIZE.1}],
                                          2.0,
                                          Color::BLACK)?;
                canvas.draw(&line, DrawParam::default());

                if i == 0 {
                    continue;
                }
                let line = Mesh::new_line(ctx,
                                          &vec![Point2 {x: i as f32 * CELL_SIZE.0, y: 0.0},
                                                Point2 {x: i as f32 * CELL_SIZE.0, y: WINDOW_SIZE.1}],
                                          2.0,
                                          Color::BLACK)?;
                canvas.draw(&line, DrawParam::default());
            }
        }
        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
            &mut self,
            _ctx: &mut Context,
            _button: event::MouseButton,
            x: f32,
            y: f32,
        ) -> Result<(), GameError> {
        self.grid[(x / CELL_SIZE.0).floor() as usize][(y / CELL_SIZE.1).floor() as usize] ^= true;
        Ok(())
    }

    fn key_down_event(
            &mut self,
            ctx: &mut Context,
            input: KeyInput,
            repeated: bool,
        ) -> Result<(), GameError> {
        if input.keycode == Some(KeyCode::Space) && !repeated {
            self.is_running ^= true;
        }
        if input.keycode == Some(KeyCode::Up) {
            self.fps += 1;
        }
        if input.keycode == Some(KeyCode::Down) {
            self.fps -= 1;
        }
        if input.keycode == Some(KeyCode::Delete) {
            self.grid = vec![vec![false; GRID_SIZE.1 as usize]; GRID_SIZE.0 as usize];
        }
        if input.keycode == Some(KeyCode::Q) {
            ctx.request_quit();
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let state = MainState::new();
    let (ctx, event_loop) = ContextBuilder::new("Conway's Game of Life", "Eyepatch")
        .window_mode(WindowMode::default().dimensions(WINDOW_SIZE.0, WINDOW_SIZE.1))
        .build()?;

    event::run(ctx, event_loop, state);
}
