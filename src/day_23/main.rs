use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input");

    let code = parse_input(input);
    let mut regs = HashMap::from([
        ('a', 0),
        ('b', 0),
    ]);

    run(&code, 0, &mut regs);

    for (reg, val) in regs.iter() {
        println!("{}: {}",reg, val);
    }


    let mut regs = HashMap::from([
        ('a', 1),
        ('b', 0),
    ]);

    run(&code, 0, &mut regs);

    for (reg, val) in regs.iter() {
        println!("{}: {}",reg, val);
    }

}


fn run(code: &Vec<(Op, char)>, mut pc: usize, regs: &mut HashMap<char, i32>) {
    while let Some((op, reg)) = code.get(pc) {
        match op {
            Op::hlf => {*regs.get_mut(reg).unwrap() /= 2; pc += 1},
            Op::tpl => {*regs.get_mut(reg).unwrap() *= 3; pc += 1},
            Op::inc => {*regs.get_mut(reg).unwrap() += 1; pc += 1},
            Op::jmp(offset) => pc = (pc as i32 + offset) as usize,
            Op::jie(offset) => {if *regs.get_mut(reg).unwrap() % 2 == 0 { pc = (pc as i32 + offset) as usize } else { pc += 1 }},
            Op::jio(offset) => {if *regs.get_mut(reg).unwrap() == 1 { pc = (pc as i32 + offset) as usize } else { pc += 1 }},
        }
    }
}


fn parse_input(input: &str) -> Vec<(Op, char)> {
    let re = Regex::new(r"^(?P<op>\w+) (?P<reg>\w)?(?:, )?(?P<offset>[+-]\d+)?$").unwrap();
    let mut output = Vec::new();
    for line in input.lines() {
        let caps = re.captures(line).expect(&format!("Unable to parse: \"{}\"",line));
        
        let op_str = caps.name("op").unwrap().as_str();
        let reg_str = if let Some(m) = caps.name("reg") { m.as_str() } else { "" };
        let offset_str = if let Some(m) = caps.name("offset") { m.as_str() } else { "" };
        let op_tuple = match op_str {
            "hlf" => ((Op::hlf, reg_str.chars().next().unwrap())),
            "tpl" => ((Op::tpl, reg_str.chars().next().unwrap())),
            "inc" => ((Op::inc, reg_str.chars().next().unwrap())),
            "jmp" => {
                let offset = offset_str.parse().unwrap();
                (Op::jmp(offset), '_')
            },
            "jie" => {
                let offset = offset_str.parse().unwrap();
                (Op::jie(offset), reg_str.chars().next().unwrap())
            },
            "jio" => {
                let offset = offset_str.parse().unwrap();
                (Op::jio(offset), reg_str.chars().next().unwrap())
            },
            _ => panic!("Unknown op: {}",line)
        };
        output.push(op_tuple);
    }
    output
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
enum Op {
    hlf,
    tpl,
    inc,
    jmp(i32),
    jie(i32),
    jio(i32),
}



#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
    "inc a\n\
    jio a, +2\n\
    tpl a\n\
    inc a\n\
    jmp +2\n\
    inc b\n\
    inc a";


    #[test]
    fn test1() {
        dbg!(parse_input(INPUT));
    }
}