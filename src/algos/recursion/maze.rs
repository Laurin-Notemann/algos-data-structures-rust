#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    x: i32,
    y: i32,
}

// There is one flaw with this solution, it shows the entire path even if part of the path is not
// necessary to traverse for the shortest path
pub fn walk(maze: &Vec<&str>, wall: &str, end: &Point, curr: &Point, seen: &mut Vec<Vec<bool>>) -> bool {
    // is of the map
    if curr.x < 0 || curr.x >= maze[0].as_bytes().len() as i32 || curr.y < 0 || curr.y >= maze.len() as i32 {
        return false;
    } 
    // is curr is a wall
    if maze[curr.y as usize].as_bytes()[curr.x as usize] == wall.as_bytes()[0]{
        return false;
    }
    // is curr at the end
    if end.x == curr.x && end.y == curr.y {
        seen[curr.y as usize][curr.x as usize] = true;
        return true;
    }
    //already been there
    if seen[curr.y as usize][curr.x as usize] {
        return false;
    }
    //edit seen vector
    seen[curr.y as usize][curr.x as usize] = true;
    //top
    if walk(maze, wall, end, &Point { x: curr.x, y: curr.y - 1 }, seen) {
        return true;
    }
    //bottom
    if walk(maze, wall, end, &Point { x: curr.x, y: curr.y + 1 }, seen) {
        return true;
    }
    //right
    if walk(maze, wall, end, &Point { x: curr.x +1 , y: curr.y }, seen) {
        return true;
    }
    if walk(maze, wall, end, &Point { x: curr.x +1 , y: curr.y }, seen) {
        return true;
    }
    return false;
}

pub fn solve(maze: Vec<&str>, wall: &str, start: Point, end: Point, seen: &mut Vec<Vec<bool>>) -> Vec<Point> {

    walk(&maze, wall, &end, &start, seen);
    let mut all_points_vec: Vec<Point> = Vec::new();
    for i in 0..seen.len() {
        for j in 0..seen[i].len() {
            if seen[i][j] {
                let point = Point { x: j as i32, y: i as i32};
                all_points_vec.push(point);
            }
        }
    }
    all_points_vec
}

#[cfg(test)]
mod test {
    use super::{Point, solve};

    #[test]
    fn mave_solver() {
        let maze = vec![
            "XXXXXX XXX",
            "X       XX", 
            "X XXXXXXXX"];
        let wall = "X";
        let start = Point {x: 1, y: 2};
        let end = Point {x: 6, y: 0};
        let mut seen = vec![vec![false, false, false, false,false, false, false, false, false, false],
        vec![false, false, false, false,false, false, false, false, false, false],
        vec![false, false, false, false,false, false, false, false, false, false]];

        let result = solve(maze, wall, start, end, &mut seen);

        let should_be = vec![Point {x: 6, y: 0},Point {x: 1, y: 1},Point {x: 2, y: 1},Point {x: 3, y: 1},Point {x: 4, y: 1},Point {x: 5, y: 1},Point {x: 6, y: 1},Point {x: 1, y: 2},];
        assert_eq!(result, should_be)
    }
}
