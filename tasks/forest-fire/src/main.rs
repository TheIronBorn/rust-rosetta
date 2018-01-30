#![feature(range_contains)]

extern crate ansi_term;
extern crate rng_stuff;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Tile {
    Empty,
    Tree,
    Burning,
    Heating,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match *self {
            Empty => Black.paint(" "),
            Tree => Green.paint("T"),
            Burning => Red.bold().paint("B"),
            Heating => Yellow.bold().paint("T"),
        };
        write!(f, "{}", output)
    }
}

const NEW_TREE_PROB: u64 = 100; // 1/100 = 0.01
const INITIAL_TREE_PROB: u64 = 2; // 1/2 = 0.5
const FIRE_PROB: u64 = 1000; // 1/1000 = 0.001

const FOREST_WIDTH: usize = 60;
const FOREST_HEIGHT: usize = 30;

const SLEEP_MILLIS: u64 = 100;

use std::fmt;
use std::io::{self, BufWriter, StdoutLock};
use std::io::prelude::*;
use std::process::Command;
use std::time::Duration;
use std::thread;

use rng_stuff::Rng;
use ansi_term::Colour::*;

use Tile::{Burning, Empty, Heating, Tree};

// #[derive(Debug)]
struct Forest<R: Rng> {
    tiles: [[Tile; FOREST_WIDTH]; FOREST_HEIGHT],
    rng: R,
    generation: usize,
}

impl<R: Rng> fmt::Display for Forest<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Generation: {}", self.generation + 1)?;
        for row in self.tiles.iter() {
            for tree in row.iter() {
                write!(f, "{}", tree)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<R: Rng> Forest<R> {
    fn new(rng: R) -> Self {
        let mut forest = Self {
            tiles: [[Tile::Empty; FOREST_WIDTH]; FOREST_HEIGHT],
            rng,
            generation: 0,
        };
        forest.prepopulate();
        forest
    }

    fn prepopulate(&mut self) {
        for row in self.tiles.iter_mut() {
            for tile in row.iter_mut() {
                // *tile = if self.rng.gen_weighted_bool(INITIAL_TREE_PROB) {
                *tile = if self.rng.next_u64() % INITIAL_TREE_PROB == 0 {
                    Tree
                } else {
                    Empty
                };
            }
        }
    }

    fn update(&mut self) {
        self.generation += 1;

        /*for row in self.tiles.iter_mut() {
            for tile in row.iter_mut() {
                *tile = match *tile {
                    Empty => {
                        // if self.rng.gen_weighted_bool(NEW_TREE_PROB) {
                        if self.rng.next_u64() % NEW_TREE_PROB == 0 {
                            Tree
                        } else {
                            Empty
                        }
                    }
                    Tree => {
                        // if self.rng.gen_weighted_bool(FIRE_PROB) {
                        if self.rng.next_u64() % FIRE_PROB == 0 {
                            Burning
                        } else {
                            Tree
                        }
                    }
                    Burning => Empty,
                    Heating => Burning,
                }
            }
        }

        for y in 0..FOREST_HEIGHT {
            for x in 0..FOREST_WIDTH {
                if self.tiles[y][x] == Burning {
                    self.heat_neighbors(y, x);
                }
            }
        }*/

        for y in 0..FOREST_HEIGHT {
            for x in 0..FOREST_WIDTH {
                /*let tile = unsafe {
                    self.tiles.get_unchecked_mut(y).get_unchecked_mut(x)
                };*/
                *unsafe {
                    self.tiles.get_unchecked_mut(y).get_unchecked_mut(x)
                } = match *unsafe {
                    self.tiles.get_unchecked_mut(y).get_unchecked_mut(x)
                } {
                    Empty => {
                        // if self.rng.gen_weighted_bool(NEW_TREE_PROB) {
                        if self.rng.next_u64() % NEW_TREE_PROB == 0 {
                            Tree
                        } else {
                            Empty
                        }
                    }
                    Tree => {
                        // if self.rng.gen_weighted_bool(FIRE_PROB) {
                        if self.rng.next_u64() % FIRE_PROB == 0 {
                            Burning
                        } else {
                            Tree
                        }
                    }
                    Burning => Empty,
                    Heating => {
                        self.heat_neighbors(y, x);
                        Burning
                    },
                }
            }
        }
    }

    fn heat_neighbors(&mut self, y: usize, x: usize) {
        let neighbors = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for &(x_off, y_off) in &neighbors {
            let nx = x as i8 + x_off;
            let ny = y as i8 + y_off;
            if (0..FOREST_WIDTH as i8).contains(nx) && (0..FOREST_HEIGHT as i8).contains(ny) {
                let tile = unsafe {
                    self.tiles.get_unchecked_mut(y).get_unchecked_mut(x)
                };
                if *tile == Tree {
                    *tile = Heating;
                }
                // self.tiles[ny as usize][nx as usize] = Heating
            }
        }
    }

    fn print(&mut self) {
        let stdout = io::stdout();
        let mut writer = BufWriter::new(stdout.lock());
        clear_screen(&mut writer);
        writeln!(&mut writer, "{}", self).unwrap();
    }
}

fn clear_screen(writer: &mut BufWriter<StdoutLock>) {
    let output = Command::new("clear").output().unwrap();
    let string = unsafe {
        String::from_utf8_unchecked(output.stdout)
    };
    write!(writer, "{}", string).unwrap();
    // write!(writer, "{}", String::from_utf8_lossy(&output.stdout)).unwrap();
}

fn main() {
    let sleep_duration = Duration::from_millis(SLEEP_MILLIS);
    let rng = rng_stuff::xoro_62_rng();

    let mut forest = Forest::new(rng);
    forest.print();

    thread::sleep(sleep_duration);

    loop {
        forest.update();

        /*for y in 0..FOREST_HEIGHT {
            for x in 0..FOREST_WIDTH {
                if forest.tiles[y][x] == Burning {
                    forest.heat_neighbors(y, x);
                }
            }
        }*/

        forest.print();

        thread::sleep(sleep_duration);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update() {
        let mut rng = rand::xoro_62_rng();
        let mut tiles = [[Tile::Empty; FOREST_WIDTH]; FOREST_HEIGHT];
        let mut expected = [[Tile::Empty; FOREST_WIDTH]; FOREST_HEIGHT];

        let indices = {
            let mut gen_indices = || {
                (
                    (
                        rng.gen_range(0, FOREST_HEIGHT),
                        rng.gen_range(0, FOREST_WIDTH),
                    ),
                    (
                        rng.gen_range(0, FOREST_HEIGHT),
                        rng.gen_range(0, FOREST_WIDTH),
                    ),
                )
            };

            [
                gen_indices(),
                gen_indices(),
                gen_indices(),
                gen_indices(),
                gen_indices(),
            ]
        };
        for &((i, j), (i2, j2)) in &indices {
            tiles[i][j] = Burning;
            expected[i2][j2] = Empty;

            tiles[i2][j2] = Heating;
            expected[i2][j2] = Burning;
        }
        let mut forest = Forest {
            tiles,
            ..Forest::new(rng)
        };

        forest.update();

        assert_eq!(forest.generation, 1);
        for &((i, j), (i2, j2)) in &indices {
            assert_eq!(forest.tiles[i][j], expected[i][j]);
            assert_eq!(forest.tiles[i2][j2], expected[i2][j2]);
        }
    }
}
