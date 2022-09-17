use raylib::prelude::*;
use rand::prelude::*;
 
const BLOCK_SIZE: i32 = 20;

#[derive(Debug, Clone, PartialEq)]
enum Type {
    Sand,
    Wall
}
 
#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
    block_type: Type
}

#[derive(Debug)]
struct Mouse {
    x: i32,
    y: i32,
    left_button_pressed: bool,
    right_button_pressed: bool,
    middle_button_pressed: bool,
}

fn rand_num() -> i32 {
    // 0, 20, 40, 60...
    let valid_coords: Vec<i32> = (0..581).step_by(20).collect();
    let mut rng = rand::thread_rng();
    let ret: i32 = *valid_coords.choose(&mut rng).unwrap();
    ret
}

fn get_closest_coord(mut num: i32) -> i32 {
    // let valid_coords: Vec<i32> = (0..581).step_by(20).collect();
    // let mut last_coord = 0;
    // for (index, coord) in (&valid_coords).into_iter().enumerate() {
    //     if num < last_coord {
    //         if index < 2 {return 0}
    //         return valid_coords[index-2].clone()
    //     }
    //     last_coord = *coord;
    // }
    // last_coord
    while num % 20 != 0 { num -= 1 }
    num
}

fn under_is_clear(x: i32, y: i32, blocks: Vec<Block>) -> (bool, bool, bool, bool, bool) {
    let mut bottom_left_empty = true;
    let mut bottom_empty = true;
    let mut bottom_right_empty = true;
    let mut right_empty = true;
    let mut left_empty = true;

    for block in blocks {
        if x == block.x && y + 20 == block.y {
            bottom_empty = false;
        }
        if x+20 == block.x && y + 20 == block.y {
            bottom_right_empty = false;
        }
        if x-20 == block.x && y + 20 == block.y {
            bottom_left_empty = false;
        }
        if x+20 == block.x && y == block.y {
            right_empty = false;
        }
        if x-20 == block.x && y == block.y {
            left_empty = false;
        }
    }
    return (left_empty, bottom_left_empty, bottom_empty, bottom_right_empty, right_empty)
}

fn delete_block(x: i32, y: i32, blocks: &mut Vec<Block>) {
    let mut index = 0;
    while index < blocks.len() {
        if x == blocks[index].x && y == blocks[index].y {
            blocks.remove(index);
            return
        }
        index += 1;
    }
}

fn get_mouse_info(rl: &raylib::RaylibHandle) -> Mouse {
    let x = rl.get_mouse_x();
    let y = rl.get_mouse_y();
    let left_button_pressed = rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON);
    let right_button_pressed = rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_RIGHT_BUTTON);
    let middle_button_pressed = rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_MIDDLE_BUTTON);
    Mouse {x, y, left_button_pressed, right_button_pressed, middle_button_pressed}
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(600, 600)
        .title("Falling sim")
        .build();
    
    rl.set_target_fps(60);
    rl.enable_cursor();

    let mut blocks: Vec<Block> = vec![];

    while !rl.window_should_close() {
        let mouse_info = get_mouse_info(&rl);
        update_game(&mouse_info, &mut blocks);
        draw_game(&mut rl, &thread, mouse_info, &blocks);
    }
}

fn update_game(mouse: &Mouse, blocks: &mut Vec<Block>) {
    // add a new block if left mouse button is pressed
    if mouse.left_button_pressed || mouse.right_button_pressed {
        blocks.push(
            Block {
                x: get_closest_coord(mouse.x),
                y: get_closest_coord(mouse.y),
                block_type: {
                    if mouse.left_button_pressed {
                        Type::Sand
                    } else {
                        Type::Wall
                    }
                }
            }
        )
    } else if mouse.middle_button_pressed == true {
        delete_block(get_closest_coord(mouse.x), get_closest_coord(mouse.y), blocks);
    }

    if blocks.len() == 0 {return} // fail early if no blocks have been spawned

    // make blocks fall
    let mut cur_block = 0;
    while cur_block < blocks.len() {
        // fall
        if blocks[cur_block].y <= 560 && blocks[cur_block].block_type == Type::Sand { // 580 is the lowest it can go && only apply physics to sand type
            // fall stright down
            match under_is_clear(blocks[cur_block].x, blocks[cur_block].y, blocks.clone()) {
                (_, _, true, _, _) => {
                    blocks[cur_block].y += 20;
                },
                (true, true, _, _, _) => {
                    blocks[cur_block].y += 20;
                    blocks[cur_block].x -= 20;
                }
                (_, _, _, true, true) => {
                    blocks[cur_block].y += 20;
                    blocks[cur_block].x += 20;
                }
                _ => {}
            }
        }

        cur_block += 1
    }
}

fn draw_game(rl: &mut raylib::RaylibHandle, thread: &raylib::RaylibThread, mouse: Mouse, blocks: &Vec<Block>) {
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::WHITE);
    for block in blocks {
        d.draw_rectangle(block.x, block.y, BLOCK_SIZE, BLOCK_SIZE, {
            if block.block_type == Type::Sand {
                Color::YELLOW
            } else {
                Color::GRAY
            }
        });
    }
}
