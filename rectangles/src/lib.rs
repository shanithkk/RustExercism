use std::collections::{HashMap, HashSet};

struct RectangleCounter {
    x: usize,
    y: usize,
    count: u32,
    row_connecting: Vec<usize>,
    vert_connections: HashMap<usize, HashSet<usize>>,
}

impl RectangleCounter {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            count: 0,
            row_connecting: vec![],
            vert_connections: HashMap::new(),
        }
    }

    fn end_horizontal_segment(&mut self) {
        self.row_connecting = vec![];
    }

    fn end_vertical_segment(&mut self) {
        self.vert_connections.insert(self.x, HashSet::new());
    }

    fn step(&mut self, c: char) {
        match c {
            '+' => {
                let vert_connection = self
                    .vert_connections
                    .entry(self.x)
                    .or_default();
                vert_connection.insert(self.y);

                // It's a shame I need to clone this
                let current_vert = vert_connection.clone();

                for x in self.row_connecting.iter() {
                    if let Some(ys) = self.vert_connections.get(x) {
                        for y in ys.iter() {
                            if *y != self.y && current_vert.contains(y) {
                                self.count += 1;
                            }
                        }
                    }
                }

                self.row_connecting.push(self.x);
            }
            '|' => {
                self.end_horizontal_segment();
            }
            '-' => {
                self.end_vertical_segment();
            }
            _ => {
                self.end_horizontal_segment();
                self.end_vertical_segment();
            }
        }

        self.x += 1;
    }

    fn new_line(&mut self) {
        self.end_horizontal_segment();
        self.x = 0;
        self.y += 1;
    }
}

pub fn count(lines: &[&str]) -> u32 {
    let mut rectangle_counter = RectangleCounter::new();

    for r in lines.iter() {
        for c in r.chars() {
            rectangle_counter.step(c);
        }
        rectangle_counter.new_line();
    }

    rectangle_counter.count
}