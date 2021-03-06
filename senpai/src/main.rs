use std::{process::exit, time::Duration};

use libtif::image::TifImage;
use pancurses::{initscr, endwin, noecho, init_pair, has_colors, start_color, COLOR_PAIR};


fn main() {
    let tif_file = vec![
        46, 84, 73, 70, 32, 200, 91, 255, 91, 255, 91, 255, 91, 255, 91, 255, 91, 208, 92, 4, 91,
        8, 92, 5, 91, 182, 92, 6, 91, 7, 92, 6, 91, 180, 92, 7, 91, 6, 92, 8, 91, 178, 92, 9, 91,
        4, 92, 10, 91, 176, 92, 11, 91, 2, 92, 12, 91, 175, 92, 25, 91, 175, 92, 25, 91, 175, 92,
        25, 91, 175, 92, 24, 91, 177, 92, 23, 91, 178, 92, 21, 91, 180, 92, 19, 91, 182, 92, 17,
        91, 184, 92, 15, 91, 186, 92, 13, 91, 189, 92, 9, 91, 192, 92, 7, 91, 194, 92, 5, 91, 196,
        92, 3, 91, 198, 92, 1, 91, 255, 91, 255, 91, 255, 91, 148, 95, 17, 91, 3, 95, 9, 91, 12,
        95, 10, 91, 6, 95, 15, 91, 2, 95, 13, 91, 3, 95, 12, 91, 3, 95, 9, 91, 2, 95, 2, 91, 6, 95,
        1, 91, 3, 95, 7, 91, 2, 95, 8, 91, 2, 95, 1, 91, 60, 95, 1, 91, 11, 95, 1, 91, 20, 95, 1,
        91, 8, 95, 1, 91, 6, 95, 1, 91, 6, 95, 1, 91, 6, 95, 1, 91, 2, 95, 1, 91, 11, 95, 1, 91, 3,
        95, 1, 91, 14, 95, 1, 91, 10, 95, 3, 91, 5, 95, 1, 91, 3, 95, 1, 91, 5, 95, 1, 91, 2, 95,
        1, 91, 6, 95, 1, 91, 63, 95, 1, 91, 11, 95, 1, 91, 20, 95, 1, 91, 8, 95, 1, 91, 6, 95, 1,
        91, 6, 95, 1, 91, 6, 95, 1, 91, 2, 95, 1, 91, 11, 95, 1, 91, 3, 95, 1, 91, 14, 95, 1, 91,
        10, 95, 1, 91, 1, 95, 2, 91, 4, 95, 1, 91, 3, 95, 1, 91, 5, 95, 1, 91, 2, 95, 1, 91, 6, 95,
        1, 91, 2, 95, 1, 91, 60, 95, 1, 91, 11, 95, 1, 91, 20, 95, 1, 91, 8, 95, 1, 91, 6, 95, 1,
        91, 6, 95, 1, 91, 6, 95, 1, 91, 2, 95, 1, 91, 11, 95, 1, 91, 3, 95, 12, 91, 3, 95, 9, 91,
        2, 95, 1, 91, 2, 95, 2, 91, 3, 95, 1, 91, 3, 95, 7, 91, 2, 95, 1, 91, 6, 95, 1, 91, 2, 95,
        1, 91, 60, 95, 1, 91, 11, 95, 9, 91, 12, 95, 10, 91, 6, 95, 1, 91, 6, 95, 1, 91, 6, 95, 1,
        91, 2, 95, 1, 91, 11, 95, 1, 91, 14, 95, 1, 91, 3, 95, 1, 91, 10, 95, 1, 91, 3, 95, 2, 91,
        2, 95, 1, 91, 3, 95, 1, 91, 8, 95, 8, 91, 2, 95, 1, 91, 60, 95, 1, 91, 11, 95, 1, 91, 20,
        95, 1, 91, 8, 95, 1, 91, 6, 95, 1, 91, 6, 95, 1, 91, 6, 95, 1, 91, 2, 95, 1, 91, 11, 95, 1,
        91, 14, 95, 1, 91, 3, 95, 1, 91, 10, 95, 1, 91, 4, 95, 2, 91, 1, 95, 1, 91, 3, 95, 1, 91,
        8, 95, 1, 91, 6, 95, 1, 91, 2, 95, 1, 91, 60, 95, 1, 91, 11, 95, 1, 91, 20, 95, 1, 91, 8,
        95, 1, 91, 6, 95, 1, 91, 6, 95, 1, 91, 6, 95, 1, 91, 2, 95, 1, 91, 11, 95, 1, 91, 14, 95,
        1, 91, 3, 95, 1, 91, 10, 95, 1, 91, 5, 95, 3, 91, 3, 95, 1, 91, 8, 95, 1, 91, 6, 95, 1, 91,
        2, 95, 1, 91, 60, 95, 1, 91, 11, 95, 10, 91, 11, 95, 1, 91, 8, 95, 1, 91, 6, 95, 1, 91, 6,
        95, 1, 91, 6, 95, 1, 91, 2, 95, 13, 91, 3, 95, 12, 91, 3, 95, 9, 91, 2, 95, 1, 91, 6, 95,
        2, 91, 3, 95, 1, 91, 8, 95, 1, 91, 6, 95, 1, 91, 2, 95, 1, 91, 247,
    ];

    let window = initscr();

    if window.get_max_y() < 43 || window.get_max_x() < 205 {
        println!("your terminal is too smol, please make it bigger!");
        exit(0);
    }
    if !has_colors() {
        println!("YOUR TERMINAL DOESNT SUPPORT COLORS!");
        exit(1);
    }
    start_color();
    noecho();
    init_pair(1, 0, 0); //black
    init_pair(2, 1, 1); //red
    init_pair(3, 2, 2); //green
    init_pair(4, 3, 3); //yellow
    init_pair(5, 4, 4); //blue
    init_pair(6, 5, 5); //magenta
    init_pair(7, 6, 6); //cyan
    init_pair(8, 7, 7); //white
    init_pair(9, 0, 7);

    let tif_image = TifImage::parse_from_bytes(tif_file).unwrap();
    for (i, pixels) in tif_image.pixels.iter().enumerate() {
        for (j, pixel) in pixels.iter().enumerate() {
            window.attrset(COLOR_PAIR(pixel.as_u8() as u32 + 1));
            window.mvaddch(i as i32, j as i32, ' ');
            window.attroff(COLOR_PAIR(pixel.as_u8() as u32 + 1));

        }
        window.refresh();
        std::thread::sleep(Duration::from_millis(40));
    }

    window.mvprintw(42, 0, "Te Amo Senpai <3");
    window.mvprintw(41, 0, "nao liga pra essa letra cagada kkkkkkkkk");
    window.mvprintw(43, 0, "pressiona qualquer letra pra sair :)");
    window.refresh();
    window.getch();
    endwin();

}
