mod binarization;
mod convert;

fn main() {
    let mut img = gen_ary(4, 4);

    print_ary(&img);

    let line = binarization::conv_to_line(&img);

    print_line(&line);
}

fn gen_ary(x_max: usize, y_max: usize) -> Vec<Vec<u8>> {
    let mut ary = Vec::new();
    for y in 0..y_max {
        let mut x_ary = Vec::new();
        for x in 0..x_max {
            x_ary.push((y * x_max + x) as u8);
        }
        ary.push(x_ary);
    }
    ary
}

fn print_ary(ary: &Vec<Vec<u8>>) {
    let x_max = ary[0].len();
    let y_max = ary.len();

    for y in 0..y_max {
        print!("[{}", ary[y][0]);
        for x in 1..x_max {
            print!(", {}", ary[y][x]);
        }
        println!("],");
    }
}

fn print_line(ary: &Vec<u8>) {
    let l_max = ary.len();

    print!("[{}", ary[0]);
    for l in 1..l_max {
        print!(", {}", ary[l]);
    }
    println!("]");
}
