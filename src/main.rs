use std::{env, io::BufRead, process::Command, thread, time::Duration};

type Maze = Vec<Vec<char>>;

fn clear_terminal_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        Command::new("clear")
            .spawn()
            .expect("clear command failed to start")
            .wait()
            .expect("failed to wait");
    };
}

fn is_valid(maze: &Maze, row: usize, col: usize) -> bool {
    let rows = maze.len() as usize;
    let cols = maze[0].len() as usize;
    row as i32 >= 0 && row < rows && col as i32 >= 0 && col < cols
}

fn is_space(maze: &Maze, row: usize, col: usize) -> bool {
    let m = maze[row][col];
    m == ' ' || m == 'O' || m == 'X'
}

fn display(maze: &Maze) {
    for i in maze.iter() {
        for j in i.iter() {
            print!("{} ", j);
        }
        println!();
    }
}

fn solve_maze(maze: &mut Maze, source: (usize, usize)) {
    let mut stack: Vec<(usize, usize)> = Vec::new();
    stack.push(source);
    while let Some((row, col)) = stack.pop() {
        if is_valid(maze, row, col) && is_space(maze, row, col) {
            if maze[row][col] == 'X' {
                println!("TARGET FOUND AT ({}, {})", row + 1, col + 1);
                break;
            }
            maze[row][col] = '.';

            clear_terminal_screen();
            display(&maze);

            thread::sleep(Duration::from_millis(250));

            let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

            directions.iter().for_each(|(x, y)| {
                let nr = row as i32 + x;
                let nc = col as i32 + y;
                if is_valid(maze, nr as usize, nc as usize)
                    && is_space(maze, nr as usize, nc as usize)
                {
                    stack.push((nr as usize, nc as usize))
                }
            })
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut maze: Maze = Vec::new();

    let mut source: (usize, usize) = (0, 0);

    let args = env::args().collect::<Vec<String>>();
    let fs = std::fs::OpenOptions::new()
        .read(true)
        .open(args.get(1).map_or("./data.txt", |arg| &*arg))?;
    let r = std::io::BufReader::new(fs);
    r.lines().enumerate().for_each(|(idx, res)| match res {
        Ok(value) => {
            let cs = value
                .chars()
                .enumerate()
                .filter_map(|(i, x)| if i % 2 == 0 { Some(x) } else { None })
                .collect::<Vec<char>>();
            if let Some(i) = cs.iter().position(|&x| x == 'O') {
                source = (idx, i)
            }
            maze.push(cs);
        }
        Err(e) => println!("ERROR {:?}", e),
    });
    solve_maze(&mut maze, source);
    Ok(())
}
