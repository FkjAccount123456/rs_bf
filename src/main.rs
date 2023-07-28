use std::io;

fn run_bf(code: Vec<char>) {
    let mut ptr: usize = 32768;
    let mut mem: Vec<i64> = vec![];
    let mut i: usize = 0;
    for _ in 0..65536 {
        mem.push(0);
    }
    while i < code.len() {
        if code[i] == '+' {
            mem[ptr] += 1;
        } else if code[i] == '-' {
            mem[ptr] -= 1;
        } else if code[i] == '<' {
            ptr -= 1;
        } else if code[i] == '>' {
            ptr += 1;
        } else if code[i] == '.' {
            println!("{}", mem[ptr]);
        } else if code[i] == ',' {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            mem[ptr] = input.trim().parse().unwrap();
        } else if code[i] == '[' {
            if mem[ptr] == 0 {
                let mut l: usize = 1;
                let mut r: usize = 0;
                i += 1;
                while i < code.len() {
                    if code[i] == '[' {
                        l += 1;
                    } else if code[i] == ']' {
                        r += 1;
                        if l > 0 {
                            r -= 1;
                            l -= 1;
                        }
                    }
                    if l == 0 && r == 0 {
                        break;
                    }
                    i += 1;
                }
            }
        } else if code[i] == ']' {
            let mut l: usize = 0;
            let mut r: usize = 1;
            i -= 1;
            loop {
                if code[i] == ']' {
                    r += 1;
                }
                if code[i] == '[' {
                    l = l + 1;
                    if r != 0 {
                        r = r - 1;
                        l = l - 1;
                    }
                }
                if l == 0 && r == 0 {
                    break;
                }
                if i == 0 {
                    break;
                }
                i = i - 1;
            }
            i = i - 1;
        }
        i += 1;
    }
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    run_bf(input.chars().collect());
}
