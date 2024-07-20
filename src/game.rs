use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::Rng;
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem;
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::{
        canvas::{Canvas, Label, Line, Map, MapResolution, Rectangle},
        Block, BorderType, Borders, Cell, LineGauge, Paragraph, Row, Table, Wrap,
    },
    Frame, Terminal,
};

pub const MARGINX: u16 = 2;
pub const MARGINY: u16 = 1;

pub enum Flip {
    Horizontal,
    Clock,
    CounterClock,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

/// Position on the Grid, the square a tile is currently in
/// {x: 0, y: 0} would be top left square
#[derive(Debug, Clone, Copy, PartialEq, Default, Eq, Hash)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

/// Terminal coordinates, needed for Grid and Tiles to
/// know where to render on the screen
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

impl Coordinates {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

/// Tile is a single square on the grid with it's number - n
/// it has the coordinates which are the coordinates in the terminal
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Tile {
    pub coordinates: Coordinates,
    pub n: u32,
}

impl Tile {
    pub fn new(coordinates: Coordinates, n: u32) -> Self {
        Tile { coordinates, n }
    }

    pub fn mv(&mut self, coordinates: Coordinates) {
        self.coordinates = coordinates
    }
    pub fn update_n(&mut self, n: u32) {
        self.n = n;
    }
}

/// Grid represents the base for the 2048, it holds the tiles with
/// their positions on the Grid. It also holds the tiles that are
/// currently in motion and their desired positions
#[derive(Debug, PartialEq)]
pub struct Grid {
    pub tiles: HashMap<Position, Tile>,
    pub moving_tiles: Vec<(Position, Position)>,
    pub size: u16,
    pub tile_width: u16,
    pub tile_height: u16,
    pub coordinates: Coordinates,
}

impl Grid {
    pub fn new(tile_size: u16, size: u16) -> Self {
        let tile_width = tile_size;
        let tile_height = tile_size / 2;

        let mut new_grid = Self {
            tiles: HashMap::new(),
            moving_tiles: vec![],
            size,
            tile_width,
            tile_height,
            coordinates: Coordinates::new(0, 0),
        };
        new_grid.insert_tile(Position::new(1, 1), 2);
        new_grid
    }

    pub fn mv(&mut self, new_coordinates: Coordinates) {
        self.coordinates = new_coordinates
    }

    pub fn change_tile_size(&mut self, new_size: u16) {
        if new_size == self.tile_width {
            return;
        }
        self.tile_width = new_size;
        self.tile_height = new_size / 2;
        for (pos, tile) in self.tiles.clone().iter() {
            self.tiles
                .insert(*pos, Tile::new(self.get_coordinates_at(*pos), tile.n));
        }
    }

    pub fn change_size(&mut self, new_size: u16) {
        if new_size == self.size {
            return;
        }
        self.size = new_size;
    }

    pub fn width(&self) -> u16 {
        2 + self.tile_width * self.size + MARGINX * self.size
    }

    pub fn height(&self) -> u16 {
        self.width() / 2
    }

    pub fn simulate_size(&self, tile_size: u16) -> (u16, u16) {
        let width = 2 + tile_size * self.size + MARGINX * self.size;
        (width + self.coordinates.x, width / 2 + self.coordinates.y)
    }

    /// try to adjust the size of the game to fit the terminal, if it's not possible return an error
    pub fn adjust_size(&mut self, terminal_width: u16, terminal_height: u16) -> Result<(), String> {
        let tile_sizes: [u16; 2] = [10, 6];
        let mut final_size: u16 = 0;
        for size in tile_sizes {
            let (width, height) = self.simulate_size(size);
            if width <= terminal_width && height <= terminal_height {
                final_size = size;
                break;
            }
        }

        if final_size == 0 {
            return Err("The size of your terminal is too small and can't fit the game! Try to make it larger.".to_string());
        }

        if self.tile_width != final_size {
            self.change_tile_size(final_size);
        }

        return Ok(());
    }

    pub fn check_if_game_can_continue(&mut self) -> Result<(), String> {
        if self.tiles.iter().any(|(_, tile)| tile.n == 2048) {
            return Err("Game Won".to_string());
        }

        if self.tiles.len() == (self.size * self.size) as usize {
            if !vec![Move::Up, Move::Down, Move::Left, Move::Right]
                .iter()
                .any(|mv| self.check(*mv) != self.moving_tiles)
            {
                return Err("Game Lost".to_string());
            }
        }

        return Ok(());
    }

    pub fn get_tile_mut(&mut self, pos: Position) -> Option<&mut Tile> {
        if let Some(_) = self.tiles.get(&pos) {
            Some(self.tiles.get_mut(&pos).unwrap())
        } else {
            None
        }
    }

    pub fn get_tile(&mut self, pos: Position) -> Option<Tile> {
        if let Some(tile) = self.tiles.get(&pos) {
            Some(*tile)
        } else {
            None
        }
    }

    pub fn get_coordinates_at(&self, pos: Position) -> Coordinates {
        Coordinates {
            x: self.coordinates.x + MARGINX + pos.x * MARGINX + pos.x * self.tile_width,
            y: self.coordinates.y + MARGINY + pos.y * MARGINY + pos.y * self.tile_height,
        }
    }

    pub fn insert_tile(&mut self, pos: Position, n: u32) {
        self.tiles
            .insert(pos, Tile::new(self.get_coordinates_at(pos), n));
    }

    pub fn remove_tile(&mut self, pos: Position) {
        self.tiles.remove(&pos);
    }

    pub fn remove_moving_tile(&mut self, pos: Position) {
        let index = self
            .moving_tiles
            .iter()
            .position(|(p, _)| p == &pos)
            .unwrap();
        self.moving_tiles.remove(index);
    }

    pub fn spawn_random_tile(&mut self) {
        let mut available = vec![];
        for x in 0..self.size {
            for y in 0..self.size {
                if !self.tiles.contains_key(&Position::new(x, y)) {
                    available.push((x, y));
                }
            }
        }
        if available.len() < 1 {
            return;
        }

        if let Some((x, y)) = available.choose(&mut rand::thread_rng()) {
            let mut rng = rand::thread_rng();
            let new_n = match rng.gen_range(0..=10) {
                x if x < 9 => 2,
                _ => 4,
            };
            self.insert_tile(Position::new(*x, *y), new_n);
        }
    }

    pub fn flip(&mut self, flip: Flip) {
        let s = self.size - 1;
        self.moving_tiles = self
            .moving_tiles
            .iter()
            .map(|(pos, new_pos)| match flip {
                Flip::Horizontal => (
                    Position::new(s - pos.x, pos.y),
                    Position::new(s - new_pos.x, new_pos.y),
                ),
                Flip::CounterClock => (
                    Position::new(s - pos.y, pos.x),
                    Position::new(s - new_pos.y, new_pos.x),
                ),
                Flip::Clock => (
                    Position::new(pos.y, s - pos.x),
                    Position::new(new_pos.y, s - new_pos.x),
                ),
            })
            .collect();
        self.tiles = self
            .tiles
            .iter()
            .map(|(pos, tile)| match flip {
                Flip::Horizontal => (Position::new(s - pos.x, pos.y), *tile),
                Flip::CounterClock => (Position::new(s - pos.y, pos.x), *tile),
                Flip::Clock => (Position::new(pos.y, s - pos.x), *tile),
            })
            .collect();
    }

    fn get_desired_position(
        &mut self,
        pos: Position,
        n: u32,
        unavailable: &Vec<Position>,
    ) -> (Position, u32) {
        let Position { x, y } = pos;
        if x == 0_u16 {
            return (Position::new(x, y), n);
        }

        let mut new_x = x;
        for checking_x in (0..x).rev() {
            let new_pos = Position::new(checking_x, y);
            if unavailable.contains(&new_pos) {
                break;
            }

            if let Some(checking_tile) = self.get_tile(new_pos) {
                if checking_tile.n == n {
                    return (Position::new(checking_x, y), n * 2);
                } else {
                    break;
                }
            } else {
                new_x = checking_x;
            }
        }
        (Position::new(new_x, y), n)
    }

    /// try to move the tiles in the direction specified by "mv", by first flipping
    /// the board always to the same position, solving for this position and then
    /// flipping it back to its original position
    ///
    /// For example if we want to move the tiles down we can instead rotate the board
    /// clockwise then solve for tiles moving to the left and then rotate the board
    /// back to it's original position (counterclockwise)
    pub fn check(&mut self, mv: Move) -> Vec<(Position, Position)> {
        let mut new_grid = Grid {
            tiles: HashMap::new(),
            moving_tiles: vec![],
            ..*self
        };

        match mv {
            Move::Right => {
                self.flip(Flip::Horizontal);
            }
            Move::Up => {
                self.flip(Flip::Clock);
            }
            Move::Down => {
                self.flip(Flip::CounterClock);
            }
            _ => (),
        };

        // thanks to flipping the grid, now we can move all the tiles to the left and then
        // flip the grid back to it's original position but this time with tiles moved to
        // their desired position
        let mut unavailable = vec![];
        for (pos, tile) in self.tiles.iter().sorted_by_key(|(p, _)| p.x) {
            let (new_pos, n) =
                new_grid.get_desired_position(Position::new(pos.x, pos.y), tile.n, &unavailable);
            if n > tile.n {
                unavailable.push(new_pos);
            }
            new_grid.insert_tile(new_pos, n);
            if pos != &new_pos {
                new_grid.moving_tiles.push((*pos, new_pos));
            }
        }

        match mv {
            Move::Right => {
                self.flip(Flip::Horizontal);
                new_grid.flip(Flip::Horizontal);
            }
            Move::Up => {
                self.flip(Flip::CounterClock);
                new_grid.flip(Flip::CounterClock);
            }
            Move::Down => {
                self.flip(Flip::Clock);
                new_grid.flip(Flip::Clock);
            }
            _ => (),
        };

        new_grid.moving_tiles
    }

    pub fn on_tick(&mut self, mv: Option<Move>) -> Result<(), String> {
        if self.moving_tiles.len() > 0 {
            // if tiles are still moving, move them closer to the desired position
            for (pos, new_pos) in self.moving_tiles.clone().iter() {
                let desired = self.get_coordinates_at(*new_pos);
                let tile = self.get_tile(*pos).unwrap();
                let current = tile.coordinates;

                let mut x = current.x;
                let mut y = current.y;

                match desired {
                    _ if desired.x > current.x => x += 4,
                    _ if desired.x < current.x => x -= 4,
                    _ if desired.y > current.y => y += 2,
                    _ if desired.y < current.y => y -= 2,
                    _ => {}
                }

                if desired == Coordinates::new(x, y) {
                    if let Some(tile) = self.get_tile(*new_pos) {
                        self.insert_tile(*new_pos, tile.n * 2);
                    } else {
                        let n = self.get_tile(*pos).unwrap().n;
                        self.insert_tile(*new_pos, n);
                    }
                    self.remove_tile(*pos);
                    self.remove_moving_tile(*pos);
                } else {
                    let tile = self.get_tile_mut(*pos).unwrap();
                    tile.mv(Coordinates::new(x, y));
                }
            }

            if self.moving_tiles.len() == 0 {
                // if there is no more tiles moving it means that all
                // the tiles achieved their desired position and we can
                // spawn a new tile and check if game can continue
                self.spawn_random_tile();
                self.check_if_game_can_continue()?;
            }

            return Ok(());
        }

        match mv {
            Some(mv) => self.moving_tiles = self.check(mv),
            _ => (),
        }

        return Ok(());
    }
}
