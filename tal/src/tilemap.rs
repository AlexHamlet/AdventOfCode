use crate::Point;

/// A fixed-size (at runtime) 2D array of `T`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tilemap<T> {
    width: usize,
    height: usize,
    inner: Vec<T>,
}

impl<T> Tilemap<T> {
    pub fn new_empty() -> Tilemap<T> {
        Tilemap {
            width: 0,
            height: 0,
            inner: Vec::new(),
        }
    }
    pub fn new_with_fn(width: usize, height: usize, func: impl Fn(Point) -> T) -> Tilemap<T> {
        let func = &func;
        Tilemap {
            width,
            height,
            inner: (0..height)
                .flat_map(|y| {
                    (0..width).map(move |x| {
                        func(Point {
                            x: x as i64,
                            y: y as i64,
                        })
                    })
                })
                .collect(),
        }
    }
    /// Create and populate a new Tilemap with information read from standard
    /// input. Calls the provided function with the location of each tile and
    /// the character that was found at that tile. The function must return
    /// the tile to put at that point based on that character.
    pub fn new_from_char_based_stdin(mut func: impl FnMut(Point, char) -> T) -> Tilemap<T> {
        let mut ret = Tilemap::new_empty();
        for (y, line) in std::io::stdin().lines().enumerate() {
            let line = line.unwrap();
            ret.add_row(line.chars().enumerate().map(|(x, ch)| {
                func(
                    Point {
                        x: x as i64,
                        y: y as i64,
                    },
                    ch,
                )
            }))
        }
        ret
    }
    /// If we are empty, just add everything this iterator has to a new first
    /// row. If we are non-empty, add everything in the iterator to a new next
    /// row, **which must be the same width as the existing rows**.
    pub fn add_row(&mut self, iter: impl Iterator<Item = T>) {
        self.inner.extend(iter);
        self.height += 1;
        if self.height == 1 {
            self.width = self.inner.len();
        } else {
            if self.width * self.height != self.inner.len() {
                panic!("attempted to add a row of differing width to an existing tilemap");
            }
        }
    }
    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn get_tile(&self, coords: Point) -> Option<&T> {
        let i = self.coords_to_index(coords)?;
        Some(&self.inner[i])
    }
    #[deprecated(note = "use get_tile_mut instead")]
    pub fn get_mut_tile(&mut self, coords: Point) -> Option<&mut T> {
        self.get_tile_mut(coords)
    }
    pub fn get_tile_mut(&mut self, coords: Point) -> Option<&mut T> {
        let i = self.coords_to_index(coords)?;
        Some(&mut self.inner[i])
    }
    /// Replaces the value at the given coordinates with a new one. Returns
    /// the old one (in case you need it for some reason). PANICS if coords
    /// is out of range.
    pub fn set_tile(&mut self, coords: Point, new_value: T) -> T {
        let mut value = new_value;
        std::mem::swap(
            &mut value,
            self.get_tile_mut(coords)
                .expect("set_tile called with out-of-bounds coordinateS"),
        );
        value
    }
    /// If the given coordinates are in-bounds, returns the index of that
    /// tile. Otherwise returns `None`.
    pub fn coords_to_index(&self, coords: Point) -> Option<usize> {
        let x = usize::try_from(coords.x).ok()?;
        let y = usize::try_from(coords.y).ok()?;
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(y * self.width + x)
        }
    }
    /// If the given index is in the range `0..width*height`, returns the X and
    /// Y coordinates that correspond to that index. Otherwise, returns `None`.
    fn index_to_coords(&self, index: usize) -> Option<Point> {
        if self.inner.len() <= index {
            None
        } else {
            Some(Point {
                x: (index % self.width) as i64,
                y: (index / self.width) as i64,
            })
        }
    }
    /// Iterate through every tile in the map, from left to right, top to
    /// bottom.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }
    /// Iterate through every tile in the map, from left to right, top to
    /// bottom. (Mutable edition)
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.inner.iter_mut()
    }
    /// Iterate through **the coordinates of** every tile in the bounds of
    /// the map, from left to right, top to bottom.
    pub fn coordinates(&self) -> impl Iterator<Item = Point> {
        self.inner
            .iter()
            .enumerate()
            .map(|(f, _)| self.index_to_coords(f).unwrap())
    }
}

impl<T: Clone> Tilemap<T> {
    pub fn new_with_clones(width: usize, height: usize, value: T) -> Tilemap<T> {
        Tilemap::new_with_fn(width, height, |_| value.clone())
    }
}

impl<T: Copy> Tilemap<T> {
    pub fn new_with_copies(width: usize, height: usize, value: T) -> Tilemap<T> {
        Tilemap::new_with_fn(width, height, |_| value)
    }
}
