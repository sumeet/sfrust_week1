use std::fs::read_to_string;

fn main() {
    let file = read_to_string("./day1_input").unwrap();
    let mut sum = 0;
    for mass in file.lines().map(|line| line.parse::<usize>().unwrap()) {
        // Specifically, to find the fuel required for a module, take its mass, divide by three,
        // round down, and subtract 2.
        let fuel = (mass / 3) - 2;
        sum += fuel;
    }
    dbg!(sum);
}
// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
// use sdl2::rect::Rect;
// use std::time::Duration;
//
// const WIDTH: usize = 200;
// const HEIGHT: usize = 150;
//
// fn randomize(grid: &mut [[bool; WIDTH]; HEIGHT]) {
//     for i in 0..HEIGHT {
//         for j in 0..WIDTH {
//             grid[i][j] = rand::random();
//         }
//     }
// }
//
// fn next_generation(grid: &mut [[bool; WIDTH]; HEIGHT]) {
//     let prev_grid = grid.clone();
//     for y in 0..HEIGHT {
//         for x in 0..WIDTH {
//             grid[y][x] = should_be_on_or_off(x, y, prev_grid);
//         }
//     }
// }
//
// // Game Of Life Rules
// // Any live cell with fewer than two live neighbours dies, as if by underpopulation.
// // Any live cell with two or three live neighbours lives on to the next generation.
// // Any live cell with more than three live neighbours dies, as if by overpopulation.
// // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
// fn should_be_on_or_off(x: usize, y: usize, prev_grid: [[bool; WIDTH]; HEIGHT]) -> bool {
//     let is_currently_on = prev_grid[y][x];
//     let dxdys = [
//         (-1isize, -1isize),
//         (-1, 0),
//         (-1, 1),
//         (0, -1),
//         (0, 1),
//         (1, -1),
//         (1, 0),
//         (1, 1),
//     ];
//     let num_live_neighbors = dxdys
//         .iter()
//         .filter(|(dx, dy)| {
//             let nx = x as isize + dx;
//             let ny = y as isize + dy;
//             if (nx >= 0 && nx < WIDTH as _) && (ny >= 0 && ny < HEIGHT as _) {
//                 prev_grid[ny as usize][nx as usize]
//             } else {
//                 false
//             }
//         })
//         .count();
//     match (is_currently_on, num_live_neighbors) {
//         (true, 2 | 3) => true,
//         (false, 3) => true,
//         _ => false,
//     }
// }
//
// pub fn main() {
//     let mut grid = [[false; WIDTH]; HEIGHT];
//     randomize(&mut grid);
//
//     let sdl_context = sdl2::init().expect("This should never happen");
//     let video_subsystem = sdl_context.video().unwrap();
//
//     let window_width = (WIDTH * 16) as _;
//     let window_height = (HEIGHT * 16) as _;
//     let window = video_subsystem
//         .window("Game of Life", window_width, window_height)
//         .position_centered()
//         .build()
//         .unwrap();
//
//     let mut canvas = window.into_canvas().build().unwrap();
//
//     let mut event_pump = sdl_context.event_pump().unwrap();
//     'running: loop {
//         for event in event_pump.poll_iter() {
//             match event {
//                 Event::Quit { .. }
//                 | Event::KeyDown {
//                     keycode: Some(Keycode::Escape),
//                     ..
//                 } => break 'running,
//                 _ => {}
//             }
//         }
//         next_generation(&mut grid);
//
//         canvas.set_draw_color(Color::BLACK);
//         canvas.clear();
//
//         for y in 0..HEIGHT {
//             for x in 0..WIDTH {
//                 if grid[y][x] {
//                     canvas.set_draw_color(Color::WHITE);
//                     //canvas.draw_point((y as i32, x as i32)).unwrap();
//
//                     canvas
//                         .draw_rect(Rect::new((x * 8) as i32, (y * 8) as i32, 8, 8))
//                         .unwrap();
//                 }
//             }
//         }
//         canvas.present();
//
//         ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
//     }
// }

// calculator
// use num_bigint::BigInt;
// use std::io::stdin;

// fn main() {
//     for line in stdin().lines() {
//         let line = line.unwrap();
//         let line = line.trim();
//         let mut words = line.split(' ');
//         let lhs = words.next().unwrap().parse::<BigInt>().unwrap();
//         let operator = words.next().unwrap();
//         let rhs = words.next().unwrap().parse::<BigInt>().unwrap();
//
//         let result = match operator {
//             "*" => lhs * rhs,
//             "/" => lhs / rhs,
//             "+" => lhs + rhs,
//             "-" => lhs - rhs,
//             _ => panic!("Unknown operator: {}", operator),
//         };
//         println!("result: {}", result);
//     }
// }

// O 12 * 23
// X (12 * 23) + (12 * 23)
