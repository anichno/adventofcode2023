pub trait Grid: Sized {
    type Item;
    fn get_location(&self, x: usize, y: usize) -> Option<Self::Item>;

    fn get_offset_location(
        &self,
        x: usize,
        x_offset: i32,
        y: usize,
        y_offset: i32,
    ) -> Option<Self::Item> {
        let x = checked_offset(x, x_offset);
        let y = checked_offset(y, y_offset);

        if let (Some(x), Some(y)) = (x, y) {
            self.get_location(x, y)
        } else {
            None
        }
    }

    fn adjacents(&self, x: usize, y: usize) -> Adjacents<Self> {
        Adjacents {
            grid: self,
            center_loc: (x, y),
            cur_pos: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Offset<V> {
    pub x: usize,
    pub y: usize,
    pub val: V,
}

pub struct Adjacents<'a, G: Grid> {
    grid: &'a G,
    center_loc: (usize, usize),
    cur_pos: usize,
}

impl<'a, G> Iterator for Adjacents<'a, G>
where
    G: Grid,
{
    type Item = Offset<G::Item>; //(usize, usize, G::Item);

    fn next(&mut self) -> Option<Self::Item> {
        const OFFSETS: [(i32, i32); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        let (x, y) = self.center_loc;

        while self.cur_pos < 8 {
            let (x_offset, y_offset) = OFFSETS[self.cur_pos];
            self.cur_pos += 1;
            if let Some(val) = self.grid.get_offset_location(x, x_offset, y, y_offset) {
                return Some(Offset {
                    x: checked_offset(x, x_offset).unwrap(),
                    y: checked_offset(y, y_offset).unwrap(),
                    val,
                });
            }
        }

        None
    }
}

impl<V> Grid for Vec<Vec<V>>
where
    V: Clone,
{
    type Item = V;

    fn get_location(&self, x: usize, y: usize) -> Option<Self::Item> {
        self.get(y).and_then(|v| v.get(x)).cloned()
    }
}

pub fn checked_offset(val: usize, offset: i32) -> Option<usize> {
    if offset < 0 {
        let offset = -offset as usize;
        val.checked_sub(offset)
    } else {
        val.checked_add(offset as usize)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_adjacents() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let mut adjacents = grid.adjacents(1, 1); //adjacent_iter(1, 1, &grid);
        assert_eq!(adjacents.next(), Some(Offset { x: 0, y: 0, val: 1 }));
        assert_eq!(adjacents.next(), Some(Offset { x: 1, y: 0, val: 2 }));
        assert_eq!(adjacents.next(), Some(Offset { x: 2, y: 0, val: 3 }));
        assert_eq!(adjacents.next(), Some(Offset { x: 0, y: 1, val: 4 }));
        assert_eq!(adjacents.next(), Some(Offset { x: 2, y: 1, val: 6 }));
        assert_eq!(adjacents.next(), Some(Offset { x: 0, y: 2, val: 7 }));
        assert_eq!(adjacents.next(), Some(Offset { x: 1, y: 2, val: 8 }));
        assert_eq!(adjacents.next(), Some(Offset { x: 2, y: 2, val: 9 }));
        assert_eq!(adjacents.next(), None);
        assert_eq!(grid.adjacents(0, 0).count(), 3);
    }
}
