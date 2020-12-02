#[test]
fn it_works() {
    let input: Vec<&str> =
        vec![
            "   +--+",
            "  ++  |",
            "+-++--+",
            "|  |  |",
            "+--+--+"
        ];
    let rectangles = Rectangles::new(input);
    assert_eq!(rectangles.count(), 6);

    let input2: Vec<&str> =
        vec![
            "+---+--+----+",
            "|   +--+----+",
            "+---+--+    |",
            "|   +--+----+",
            "+---+--+--+-+",
            "+---+--+--+-+",
            "+------+  | |",
            "          +-+"
        ];
    let rectangles2 = Rectangles::new(input2);
    assert_eq!(rectangles2.count(), 60);
}

#[derive(Debug)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Line {
    fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        Line { x1, y1, x2, y2 }
    }
}

struct Rectangles {
    characters: Vec<Vec<char>>,
}

impl Rectangles {
    fn new(input: Vec<&'static str>) -> Self {
        Rectangles {
            characters: input.iter().map(|row| row.chars().collect()).collect(),
        }
    }

    fn count(&self) -> usize {
        let mut count = 0;
        let mut i = 0;
        let mut horizontal_lines: Vec<Line> = Vec::new();
        let mut vertical_lines: Vec<Line> = Vec::new();

        for vector in self.characters.iter() {
            let mut j = 0;

            for element in vector {
                match element {
                    '+' => {
                        if j > 0 {
                            self.find_horizontal_lines(i, j, vector, &mut horizontal_lines)
                        }
                        if i > 0 {
                            self.find_vertical_lines(i, j, &mut vertical_lines)
                        }
                    }
                    _ => {}
                }

                j += 1;
            }

            i += 1;
        }

        //check for rectangles
        for top_line in &horizontal_lines {
            for bottom_line in &horizontal_lines {
                if bottom_line.x1 > top_line.x1 && bottom_line.y1 == top_line.y1 && bottom_line.y2 == top_line.y2 {
                    //println!("bottom_line {:?}", line);

                    for left_line in &vertical_lines {
                        if top_line.x1 == left_line.x1 && top_line.y1 == left_line.y1 && bottom_line.x1 == left_line.x2 && bottom_line.y1 == left_line.y2 {
                            //println!("left_line {:?}", left_line);

                            for right_line in &vertical_lines {
                                if top_line.x2 == right_line.x1 && top_line.y2 == right_line.y1 && bottom_line.x2 == right_line.x2 && bottom_line.y2 == right_line.y2 {
                                    /*println!(
                                        "rectangle found top {:?} left {:?} bottom {:?} right {:?}",
                                        top_line, left_line, bottom_line, right_line
                                    );*/

                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        count
    }

    fn find_vertical_lines(&self, i: usize, j: usize, lines: &mut Vec<Line>) {
        for previous_element_index in (0..i).rev() {
            if let Some(vector) = self.characters.iter().nth(previous_element_index) {
                match vector.iter().nth(j) {
                    Some('+') => {
                        //line stops
                        lines.push(Line::new(previous_element_index, j, i, j));

                        //println!("create vertical line ({},{}) to ({},{})", previous_element_index, j, i, j);
                    }
                    Some('-') | Some('|') => {
                        //continue, it's a line
                    }
                    _ => {
                        //white space, no line
                        break;
                    }
                }
            }
        }
    }

    fn find_horizontal_lines(&self, i: usize, j: usize, vector: &Vec<char>, lines: &mut Vec<Line>) {
        for previous_element_index in (0..j).rev() {
            //println!("i {} j {} previous_element_index {}", i, j, previous_element_index);

            match vector.iter().nth(previous_element_index) {
                Some('+') => {
                    //line stops
                    lines.push(Line::new(i, previous_element_index, i, j));

                    //println!("create horizontal line ({},{}) to ({},{})", i, previous_element_index, i, j);
                }
                Some('-') | Some('|') => {
                    //continue, it's a line
                }
                _ => {
                    //white space, no line
                    break;
                }
            }
        }
    }
}

