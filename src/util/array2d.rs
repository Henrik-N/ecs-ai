use std::fmt::{Debug, Display, Formatter};

type Coord = (usize, usize);

const NEW_LINE_CHAR: char = '\n';
const NEW_LINE_CHAR_LEN: usize = 1;

#[derive(Debug)]
pub struct Array2D<T: Clone> {
    data: Vec<T>,
    pub height: usize,
    pub width: usize,
}

impl<T: Clone + PartialEq> PartialEq for Array2D<T> {
    fn eq(&self, other: &Self) -> bool {
        // Rust's Vec<T>s may allocate a bit more than we tell it to. Compare only the part of the allocation we use,
        //  as the garbage values we filled in at allocation may not be the same ones.
        let equal_dimensions = self.height == other.height && self.width == other.width;
        let len = self.height * self.width;
        let equal_data = self.data[..len] == other.data[..len];

        equal_dimensions && equal_data
    }
}
impl<T: Clone + PartialEq> Eq for Array2D<T> {}

impl<T: Clone> Array2D<T> {
    pub fn iter_data(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }
}

impl<T: Clone> Array2D<T> {
    pub fn new(width: usize, height: usize, init_value: T) -> Self {
        let size = width * height;

        Self {
            data: vec![init_value; size],
            height,
            width,
        }
    }

    fn get_index(&self, (x, y): Coord) -> usize {
        let index = (y * self.width) + x;
        debug_assert!(index < self.data.len());
        index
    }

    pub fn get(&self, (x, y): Coord) -> &T {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);

        let index = self.get_index((x, y));

        &self.data[index]
    }

    pub fn set(&mut self, (x, y): Coord, data: T) {
        let index = self.get_index((x, y));
        self.data[index] = data;
    }
}

pub use to_string::*;
mod to_string {
    use super::*;

    pub trait CharRepresentation {
        /// Character/Symbol to represent the item in the array when translating it to a string
        fn char_representation(&self) -> char;
    }
    impl CharRepresentation for char {
        // obviously a char will be represented by itself
        fn char_representation(&self) -> char {
            *self
        }
    }

    impl<T: Clone + CharRepresentation> ToString for Array2D<T> {
        fn to_string(&self) -> String {
            debug_assert!(self.data.len() > 0);

            // each item will have 1 length, as they are each represented by a single char
            let total_string_len = self.data.len();

            let mut output = String::with_capacity(total_string_len);

            for y in 0..self.height {
                for x in 0..self.width {
                    let data: &T = self.get((x, y));
                    output.push(data.char_representation());
                    //output += &format!("{}", data);
                }
                output.push(NEW_LINE_CHAR);
            }
            output
        }
    }
}

pub use from_string::*;
mod from_string {
    use super::*;

    fn find_width(arr_str: &str) -> usize {
        let (first_line, _) = arr_str
            .split_once(NEW_LINE_CHAR)
            .expect("couldn't split string");
        first_line.chars().count()
    }

    fn find_height(arr_str: &str) -> usize {
        arr_str
            .split(NEW_LINE_CHAR)
            .filter(|&line| line.is_empty() == false) // ignore any empty lines
            .count()
    }

    impl From<String> for Array2D<char> {
        fn from(str: String) -> Self {
            let (width, height) = (find_width(&str), find_height(&str));

            let mut arr = Array2D::new(width, height, char::default());

            for (y, line) in str
                .lines()
                .into_iter()
                .filter(|&line| line.is_empty() == false)
                .enumerate()
            {
                for (x, symbol) in line.chars().enumerate() {
                    arr.set((x, y), symbol);
                }
            }

            arr
        }
    }
}

#[test]
fn test_2d_arr() {
    let mut arr = Array2D::new(10, 4, '|');

    let to_test = [
        ((0, 0), 'a'),
        ((1, 0), 'b'),
        ((0, 1), 'c'),
        ((1, 1), 'd'),
        //
        ((8, 2), 'e'),
        ((9, 2), 'f'),
        ((8, 3), 'g'),
        ((9, 3), 'h'),
    ];

    for (coord, v) in to_test {
        arr.set(coord, v);
        assert_eq!(arr.get(coord), &v);
    }

    assert_eq!(
        arr.to_string(),
        "\
        ab||||||||\n\
        cd||||||||\n\
        ||||||||ef\n\
        ||||||||gh\n"
    );

    let arr2: Array2D<char> = Array2D::from(arr.to_string());

    println!("arr2: {}", arr2.to_string());

    assert_eq!(arr.to_string(), arr2.to_string());

    assert_eq!(arr, arr2);
}
