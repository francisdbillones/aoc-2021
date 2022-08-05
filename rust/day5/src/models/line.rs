pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct Line {
    pub start: Point,
    pub stop: Point,
}

impl Line {
    pub fn draw(&self, grid: &mut Vec<Vec<u32>>, check_diagonal: bool) {
        if !check_diagonal {
            if self.is_horizontal() {
                let y = self.start.y;
                for x in self.start.x..=self.stop.x {
                    grid[y][x] += 1;
                }
            }

            if self.is_vertical() {
                let x = self.start.x;
                for y in self.start.y..=self.stop.y {
                    grid[y][x] += 1;
                }
            }
            return;
        }

        for point in self.iter() {
            grid[point.y][point.x] += 1;
        }
    }

    pub fn is_diagonal(&self) -> bool {
        return (!self.is_horizontal()) && (!self.is_vertical());
    }

    pub fn is_horizontal(&self) -> bool {
        return self.start.y == self.stop.y;
    }

    pub fn is_vertical(&self) -> bool {
        return self.start.x == self.stop.x;
    }

    pub fn iter(&self) -> LineIterator {
        if self.is_horizontal() {
            return LineIterator {
                step_x: 1,
                step_y: 0,
                cur: Point { ..self.start },
                index: 0,
                iterations: self.stop.x - self.start.x + 1,
            };
        }

        if self.is_vertical() {
            return LineIterator {
                step_x: 0,
                step_y: 1,
                cur: Point { ..self.start },
                index: 0,
                iterations: self.stop.y - self.start.y + 1,
            };
        }

        if self.is_diagonal() {
            let step_x = {
                if self.start.x < self.stop.x {
                    1
                } else {
                    -1
                }
            };

            let step_y = {
                if self.start.y < self.stop.y {
                    1
                } else {
                    -1
                }
            };

            let iterations: usize = (self.stop.x as i64 - self.start.x as i64).abs() as usize + 1;

            return LineIterator {
                step_x,
                step_y,
                cur: Point { ..self.start },
                index: 0,
                iterations,
            };
        }
        panic!("Line is neither vertical, horizontal, or diagonal");
    }
}

pub struct LineIterator {
    step_x: i32,
    step_y: i32,
    index: usize,
    iterations: usize,
    cur: Point,
}

impl Iterator for LineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.iterations {
            return None;
        }
        let cur = Point { ..self.cur };
        self.cur.x = ((self.cur.x as i32) + self.step_x) as usize;
        self.cur.y = ((self.cur.y as i32) + self.step_y) as usize;
        self.index += 1;

        return Some(cur);
    }
}
