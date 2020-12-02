#[test]
fn it_works() {
    let mut pt = PascalTriangle::new(5);
    pt.compute();

    assert_eq!(pt.data.len(), 5);
    assert_eq!(pt.data.iter().nth(0), Some(&vec![1]));
    assert_eq!(pt.data.iter().nth(1), Some(&vec![1, 1]));
    assert_eq!(pt.data.iter().nth(2), Some(&vec![1, 2, 1]));
    assert_eq!(pt.data.iter().nth(3), Some(&vec![1, 3, 3, 1]));
    assert_eq!(pt.data.iter().nth(4), Some(&vec![1, 4, 6, 4, 1]));
}

struct PascalTriangle {
    data: Vec<Vec<usize>>,
    size: usize,
}

impl PascalTriangle {
    fn new(size: usize) -> Self {
        PascalTriangle { size, data: Vec::with_capacity(size) }
    }

    fn compute(&mut self) {
        for i in 0..self.size {
            if i == 0 {
                self.data.push(vec![1]);
            } else {
                if let Some(previous_row) = self.data.iter().nth(i - 1) {
                    let row_size = previous_row.len() + 1;
                    let mut row: Vec<usize> = Vec::with_capacity(row_size);

                    for j in 0..row_size {
                        if j == 0 || j == row_size - 1 {
                            row.push(1);
                        } else {
                            if let Some(previous_left) = previous_row.iter().nth(j - 1) {
                                if let Some(previous_right) = previous_row.iter().nth(j) {
                                    row.push(previous_left + previous_right);
                                }
                            }
                        }
                    }

                    self.data.push(row);
                }
            }
        }
    }
}

