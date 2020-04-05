use std::fmt;
use std::iter::StepBy;
use std::ops::Index;
use std::ops::IndexMut;
use std::slice::Iter;
use std::slice::IterMut;

/// Stores elements of a certain type in a 2D grid structure.
pub struct Grid<T> {
    data: Vec<T>,
    cols: usize,
    rows: usize,
}

#[doc(hidden)]
#[macro_export]
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + $crate::count!($($xs)*));
}

/// Init a grid with values.
/// ```
/// use grid::grid;
/// let grid = grid![[1, 2, 3]
/// [4, 5, 6]
/// [7, 8, 9]];
/// assert_eq!(grid.size(), (3, 3))
/// ```
#[macro_export]
macro_rules! grid {
    () => {
        $crate::Grid::from_vec(vec![], 0)
    };
    ( [$( $x:expr ),* ]) => { {
        let vec = vec![$($x),*];
        let len  = vec.len();
        $crate::Grid::from_vec(vec, len)
    } };
    ( [$( $x0:expr ),*] $([$( $x:expr ),*])* ) => {
        {
            let mut _assert_width0 = [(); $crate::count!($($x0)*)];
            let cols = $crate::count!($($x0)*);
            let rows = 1usize;

            $(
                let _assert_width = [(); $crate::count!($($x)*)];
                _assert_width0 = _assert_width;
                let rows = rows + 1usize;
            )*

            let mut vec = Vec::with_capacity(rows * cols);

            $( vec.push($x0); )*
            $( $( vec.push($x); )* )*

            $crate::Grid::from_vec(vec, cols)
        }
    };
}

impl<T: Clone> Grid<T> {
    /// Init a grid of size rows x columns with default values of the given type.
    /// For example this will generate a 2x3 grid of zeros:
    /// ```
    /// use grid::Grid;
    /// let grid : Grid<u8> = Grid::new(2,2);
    /// assert_eq!(grid[0][0], 0);
    /// ```
    pub fn new(rows: usize, cols: usize) -> Grid<T>
    where
        T: Default,
    {
        if rows < 1 || cols < 1 {
            panic!("Grid size of rows and columns must be greater than zero.");
        }
        Grid {
            data: vec![T::default(); rows * cols],
            cols: cols,
            rows: rows,
        }
    }

    /// Init a grid of size rows x columns with the given data element.
    pub fn init(rows: usize, cols: usize, data: T) -> Grid<T> {
        if rows < 1 || cols < 1 {
            panic!("Grid size of rows and columns must be greater than zero.");
        }
        Grid {
            data: vec![data; rows * cols],
            cols: cols,
            rows: rows,
        }
    }

    /// Returns a grid from a vector with a given column length.
    /// The length of `vec` must be a multiple of `cols`.
    ///
    /// For example:
    ///
    /// ```
    /// use grid::Grid;
    /// let grid = Grid::from_vec(vec![1,2,3,4,5,6], 3);
    /// assert_eq!(grid.size(), (2, 3));
    /// ```
    ///
    /// will create a grid with the following layout:
    /// [1,2,3]
    /// [4,5,6]
    ///
    /// This example will fail, because `vec.len()` is not a multiple of `cols`:
    ///
    /// ``` should_panic
    /// use grid::Grid;
    /// Grid::from_vec(vec![1,2,3,4,5], 3);
    /// ```
    pub fn from_vec(vec: Vec<T>, cols: usize) -> Grid<T> {
        let rows = vec.len();
        if rows == 0 {
            if cols == 0 {
                return Grid {
                    data: vec![],
                    rows: 0,
                    cols: 0,
                };
            } else {
                panic!("Vector length is zero, but cols is {:?}", cols);
            }
        } else if rows % cols != 0 {
            panic!("Vector length must be a multiple of cols.");
        } else {
            Grid {
                data: vec,
                rows: rows / cols,
                cols: cols,
            }
        }
    }

    /// Returns a reference to an element, without performing bound checks.
    /// Generally not recommended, use with caution!
    /// Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used.
    pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> &T {
        self.data.get_unchecked(row * self.cols() + col)
    }

    /// Returns a mutable reference to an element, without performing bound checks.
    /// Generally not recommended, use with caution!
    /// Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used.
    pub unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut T {
        let cols = self.cols;
        self.data.get_unchecked_mut(row * cols + col)
    }

    /// Access a certain element in the grid.
    /// Returns None if an element beyond the grid bounds is tried to be accessed.
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            unsafe { Some(self.get_unchecked(row, col)) }
        } else {
            None
        }
    }

    /// Mutable access to a certain element in the grid.
    /// Returns None if an element beyond the grid bounds is tried to be accessed.
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row < self.rows && col < self.cols {
            unsafe { Some(self.get_unchecked_mut(row, col)) }
        } else {
            None
        }
    }

    /// Returns the size of the gird as a two element tuple.
    /// First element are the number of rows and the second the columns.
    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    /// Returns the number of rows of the grid.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns of the grid.
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Returns true if the grid contains no elements.
    /// For example:
    /// ```
    /// use grid::*;
    /// let grid : Grid<u8> = grid![];
    /// assert!(grid.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.cols == 0 && self.rows == 0
    }

    /// Clears the grid.
    pub fn clear(&mut self) {
        self.rows = 0;
        self.cols = 0;
        self.data.clear();
    }

    /// Returns an iterator over the whole grid, starting from the first row and column.
    /// ```
    /// use grid::*;
    /// let grid: Grid<u8> = grid![[1,2][3,4]];
    /// let mut iter = grid.iter();
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), Some(&4));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    /// Returns an mutable iterator over the whole grid that allows modifying each value.
    /// ```
    /// use grid::*;
    /// let mut grid: Grid<u8> = grid![[1,2][3,4]];
    /// let mut iter = grid.iter_mut();
    /// let next = iter.next();
    /// assert_eq!(next, Some(&mut 1));
    /// *next.unwrap() = 10;
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }

    /// Returns an iterator over a column.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::*;
    /// let grid: Grid<u8> = grid![[1, 2, 3][3, 4, 5]];
    /// let mut col_iter = grid.iter_col(1);
    /// assert_eq!(col_iter.next(), Some(&2));
    /// assert_eq!(col_iter.next(), Some(&4));
    /// assert_eq!(col_iter.next(), None);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the col index is out of bounds.
    pub fn iter_col(&self, col: usize) -> StepBy<Iter<T>> {
        if col < self.cols {
            return self.data[col..].iter().step_by(self.cols);
        } else {
            panic!(
                "out of bounds. Column must be less than {:?}, but is {:?}.",
                self.cols, col
            )
        }
    }

    /// Returns a mutable iterator over a column.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::*;
    /// let mut grid: Grid<u8> = grid![[1, 2, 3][3, 4, 5]];
    /// let mut col_iter = grid.iter_col_mut(1);
    /// let next = col_iter.next();
    /// assert_eq!(next, Some(&mut 2));
    /// *next.unwrap() = 10;
    /// assert_eq!(grid[0][1], 10);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the col index is out of bounds.
    pub fn iter_col_mut(&mut self, col: usize) -> StepBy<IterMut<T>> {
        let cols = self.cols;
        if col < cols {
            return self.data[col..].iter_mut().step_by(cols);
        } else {
            panic!(
                "out of bounds. Column must be less than {:?}, but is {:?}.",
                self.cols, col
            )
        }
    }

    /// Returns an iterator over a row.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::*;
    /// let grid: Grid<u8> = grid![[1, 2, 3][3, 4, 5]];
    /// let mut col_iter = grid.iter_row(1);
    /// assert_eq!(col_iter.next(), Some(&3));
    /// assert_eq!(col_iter.next(), Some(&4));
    /// assert_eq!(col_iter.next(), Some(&5));
    /// assert_eq!(col_iter.next(), None);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the row index is out of bounds.
    pub fn iter_row(&self, row: usize) -> Iter<T> {
        if row < self.rows {
            let start = row * self.cols;
            return self.data[start..(start + self.cols)].iter();
        } else {
            panic!(
                "out of bounds. Row must be less than {:?}, but is {:?}.",
                self.rows, row
            )
        }
    }

    /// Returns a mutable iterator over a row.
    ///
    /// # Examples
    ///
    /// ```
    /// use grid::*;
    /// let mut grid: Grid<u8> = grid![[1, 2, 3][3, 4, 5]];
    /// let mut col_iter = grid.iter_row_mut(1);
    /// let next = col_iter.next();
    /// *next.unwrap() = 10;
    /// assert_eq!(grid[1][0], 10);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the row index is out of bounds.
    pub fn iter_row_mut(&mut self, row: usize) -> IterMut<T> {
        if row < self.rows {
            let cols = self.cols;
            let start = row * cols;
            return self.data[start..(start + cols)].iter_mut();
        } else {
            panic!(
                "out of bounds. Row must be less than {:?}, but is {:?}.",
                self.rows, row
            )
        }
    }
}

impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Grid {
            rows: self.rows,
            cols: self.cols,
            data: self.data.clone(),
        }
    }
}

impl<T: Clone> Index<usize> for Grid<T> {
    type Output = [T];

    fn index(&self, idx: usize) -> &Self::Output {
        if idx < self.rows {
            &self.data[(idx * &self.cols)..]
        } else {
            panic!(
                "index {:?} out of bounds. Grid has {:?} rows.",
                self.rows, idx
            );
        }
    }
}

impl<T: Clone> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.data[(idx * &self.cols)..]
    }
}

impl<T: fmt::Debug> fmt::Debug for Grid<T> {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[");
        if self.cols > 0 {
            for (i, _) in self.data.iter().enumerate().step_by(self.cols) {
                write!(f, "{:?}", &self.data[i..(i + self.cols)]);
            }
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iter_row() {
        let grid: Grid<u8> = grid![[1,2,3][1,2,3]];
        let mut iter = grid.iter_row(0);
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    #[should_panic]
    fn iter_row_empty() {
        let grid: Grid<u8> = grid![];
        let _ = grid.iter_row(0);
    }

    #[test]
    #[should_panic]
    fn iter_row_out_of_bound() {
        let grid: Grid<u8> = grid![[1,2,3][1,2,3]];
        let _ = grid.iter_row(2);
    }

    #[test]
    #[should_panic]
    fn iter_col_out_of_bound() {
        let grid: Grid<u8> = grid![[1,2,3][1,2,3]];
        let _ = grid.iter_col(3);
    }

    #[test]
    #[should_panic]
    fn iter_col_zero() {
        let grid: Grid<u8> = grid![];
        let _ = grid.iter_col(0);
    }

    #[test]
    fn iter() {
        let grid: Grid<u8> = grid![[1,2][3,4]];
        let mut iter = grid.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn clear() {
        let mut grid: Grid<u8> = grid![[1, 2, 3]];
        grid.clear();
        assert!(grid.is_empty());
    }

    #[test]
    fn is_empty_false() {
        let grid: Grid<u8> = grid![[1, 2, 3]];
        assert!(!grid.is_empty());
    }

    #[test]
    fn is_empty_true() {
        let grid: Grid<u8> = grid![];
        assert!(grid.is_empty());
    }

    #[test]
    fn fmt_empty() {
        let grid: Grid<u8> = grid![];
        assert_eq!(format!("{:?}", grid), "[]");
    }

    #[test]
    fn fmt_row() {
        let grid: Grid<u8> = grid![[1, 2, 3]];
        assert_eq!(format!("{:?}", grid), "[[1, 2, 3]]");
    }

    #[test]
    fn fmt_grid() {
        let grid: Grid<u8> = grid![[1,2,3][4,5,6][7,8,9]];
        assert_eq!(format!("{:?}", grid), "[[1, 2, 3][4, 5, 6][7, 8, 9]]");
    }

    #[test]
    fn clone() {
        let grid = grid![[1, 2, 3][4, 5, 6]];
        let mut clone = grid.clone();
        clone[0][2] = 10;
        assert_eq!(grid[0][2], 3);
        assert_eq!(clone[0][2], 10);
    }

    #[test]
    fn macro_init() {
        let grid = grid![[1, 2, 3][4, 5, 6]];
        assert_eq!(grid[0][0], 1);
        assert_eq!(grid[0][1], 2);
        assert_eq!(grid[0][2], 3);
        assert_eq!(grid[1][0], 4);
        assert_eq!(grid[1][1], 5);
        assert_eq!(grid[1][2], 6);
    }

    #[test]
    fn macro_init_2() {
        let grid = grid![[1, 2, 3]
                         [4, 5, 6]
                         [7, 8, 9]];
        assert_eq!(grid.size(), (3, 3))
    }

    #[test]
    fn macro_init_char() {
        let grid = grid![['a', 'b', 'c']
                         ['a', 'b', 'c']
                         ['a', 'b', 'c']];
        assert_eq!(grid.size(), (3, 3));
        assert_eq!(grid[1][1], 'b');
    }

    #[test]
    fn macro_one_row() {
        let grid: Grid<usize> = grid![[1, 2, 3, 4]];
        assert_eq!(grid.size(), (1, 4));
        assert_eq!(grid[0][0], 1);
        assert_eq!(grid[0][1], 2);
        assert_eq!(grid[0][2], 3);
    }

    #[test]
    fn macro_init_empty() {
        let grid: Grid<usize> = grid![];
        assert_eq!(grid.size(), (0, 0));
    }

    #[test]
    fn from_vec_zero() {
        let grid: Grid<u8> = Grid::from_vec(vec![], 0);
        assert_eq!(grid.size(), (0, 0));
    }

    #[test]
    #[should_panic]
    fn from_vec_panics_1() {
        let _: Grid<u8> = Grid::from_vec(vec![1, 2, 3], 0);
    }

    #[test]
    #[should_panic]
    fn from_vec_panics_2() {
        let _: Grid<u8> = Grid::from_vec(vec![1, 2, 3], 2);
    }

    #[test]
    #[should_panic]
    fn from_vec_panics_3() {
        let _: Grid<u8> = Grid::from_vec(vec![], 1);
    }

    #[test]
    fn init() {
        Grid::init(1, 2, 3);
        Grid::init(1, 2, 1.2);
        Grid::init(1, 2, 'a');
    }

    #[test]
    fn new() {
        let grid: Grid<u8> = Grid::new(1, 2);
        assert_eq!(grid[0][0], 0);
    }

    #[test]
    #[should_panic]
    fn init_panics() {
        Grid::init(0, 2, 3);
    }

    #[test]
    #[should_panic]
    fn ctr_panics_2() {
        Grid::init(1, 0, 3);
    }

    #[test]
    fn get() {
        let grid = Grid::init(1, 2, 3);
        assert_eq!(grid.get(0, 0), Some(&3));
    }
    #[test]
    fn get_none() {
        let grid = Grid::init(1, 2, 3);
        assert_eq!(grid.get(1, 0), None);
    }

    #[test]
    fn get_mut() {
        let mut grid = Grid::init(1, 2, 3);
        let mut_ref = grid.get_mut(0, 0).unwrap();
        *mut_ref = 5;
        assert_eq!(grid[0][0], 5);
    }

    #[test]
    fn get_mut_none() {
        let mut grid = Grid::init(1, 2, 3);
        let mut_ref = grid.get_mut(1, 4);
        assert_eq!(mut_ref, None);
    }

    #[test]
    fn idx() {
        let grid = Grid::init(1, 2, 3);
        assert_eq!(grid[0][0], 3);
    }

    #[test]
    #[should_panic]
    fn idx_panic_1() {
        let grid = Grid::init(1, 2, 3);
        grid[20][0];
    }

    #[test]
    #[should_panic]
    fn idx_panic_2() {
        let grid = Grid::init(1, 2, 3);
        grid[0][20];
    }

    #[test]
    fn idx_set() {
        let mut grid = Grid::init(1, 2, 3);
        grid[0][0] = 4;
        assert_eq!(grid[0][0], 4);
    }

    #[test]
    fn size() {
        let grid = Grid::init(1, 2, 3);
        assert_eq!(grid.size(), (1, 2));
    }
}
