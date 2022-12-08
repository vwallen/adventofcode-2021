#![allow(dead_code)]

use std::collections::{VecDeque, HashMap};
use indexmap::IndexMap;
use std::ops::{Add, Mul, Div, Rem};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum OpArg {
    W,
    X,
    Y,
    Z,
    Val(i64),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum OpCode {
    Inp(OpArg),
    Add(OpArg, OpArg),
    Mul(OpArg, OpArg),
    Div(OpArg, OpArg),
    Mod(OpArg, OpArg),
    Eql(OpArg, OpArg)
}

use OpArg::*;
use OpCode::*;

#[derive(Debug, Default)]
struct Program {
    registers:HashMap<OpArg, i64>,
    inputs:VecDeque<i64>,
    instructions:VecDeque<OpCode>,
}
impl Program {

    pub fn new() -> Self {
        let mut prog = Self::default();
        prog.reset();
        prog
    }

    fn do_op<F: Fn(i64, i64) -> i64>(&mut self, a:OpArg, b:OpArg, op:F) {
        match a {
            W|X|Y|Z => {
                let val_a = self.registers.get(&a).unwrap();
                match b {
                    W|X|Y|Z => {
                        let val_b = self.registers.get(&b).unwrap();
                        self.registers.insert(a, op(*val_a, *val_b))
                    },
                    Val(val_b) => self.registers.insert(a, op(*val_a, val_b)),
                }
            },
            Val(_) => None,
        };
    }

    fn do_input(&mut self, a:OpArg) {
        match a {
            W|X|Y|Z => self.registers.insert(a, self.inputs.pop_front().unwrap()),
            Val(_) => None,
        };
    }

    fn do_add(&mut self, a:OpArg, b:OpArg) {
        self.do_op(a, b, Add::add);
    }

    fn do_multiply(&mut self, a:OpArg, b:OpArg) {
        self.do_op(a, b, Mul::mul);
    }

    fn do_divide(&mut self, a:OpArg, b:OpArg) {
        self.do_op(a, b, Div::div);
    }

    fn do_modulo(&mut self, a:OpArg, b:OpArg) {
        self.do_op(a, b, Rem::rem);
    }

    fn do_equal(&mut self, a:OpArg, b:OpArg) {
        match a {
            W|X|Y|Z => {
                let val_a = self.registers.get(&a).unwrap();
                match b {
                    W|X|Y|Z => {
                        let val_b = self.registers.get(&b).unwrap();
                        self.registers.insert(a, (*val_a == *val_b).into())
                    },
                    Val(val_b) => self.registers.insert(a, (*val_a == val_b).into()),
                }
            },
            Val(_) => None,
        };
    }

    pub fn run_instruction(&mut self, opcode:OpCode) {
        match opcode {
            Inp(a)    => self.do_input(a),
            Add(a, b) => self.do_add(a, b),
            Mul(a, b) => self.do_multiply(a, b),
            Div(a, b) => self.do_divide(a, b),
            Mod(a, b) => self.do_modulo(a, b),
            Eql(a, b) => self.do_equal(a, b),
        }
    }

    pub fn run(&mut self) {
        let mut this_run = self.instructions.clone();
        while let Some(opcode) = this_run.pop_front() {
            self.run_instruction(opcode);
        }
    }

    pub fn reset(&mut self) {
        self.inputs.clear();
        self.registers.insert(W, 0);
        self.registers.insert(X, 0);
        self.registers.insert(Y, 0);
        self.registers.insert(Z, 0);
    }

    pub fn push_instruction(&mut self, a:OpCode) {
        self.instructions.push_back(a)
    }

    pub fn push_instruction_front(&mut self, a:OpCode) {
        self.instructions.push_front(a)
    }

    pub fn push_input(&mut self, a:i64) {
        self.inputs.push_back(a)
    }

    pub fn get_result(&self, a:OpArg) -> Option<i64> {
        self.registers.get(&a).copied()
    }
    
}

fn parse_source(src: &str) -> Program {
    let mut prog = Program::new();
    for line in src.lines() {
        let (op, args) = line.split_once(' ').unwrap();
        if let Some(args) = args.split_once(' ') {
            let arg0 = match args.0 {
                "w" => W,
                "x" => X,
                "y" => Y,
                "z" => Z,
                _ => panic!("Bad arg0!")
            };
            let arg1 = match args.1 {
                "w" => W,
                "x" => X,
                "y" => Y,
                "z" => Z,
                a   => Val(a.parse::<i64>().unwrap())
            };
            let op = match op {
                "inp" => Inp(arg0),
                "add" => Add(arg0, arg1),
                "mul" => Mul(arg0, arg1),
                "div" => Div(arg0, arg1),
                "mod" => Mod(arg0, arg1),
                "eql" => Eql(arg0, arg1),
                _ => panic!("Bad op!"),
            };
            prog.push_instruction(op);
        } else {
            let arg0 = match args {
                "w" => W,
                "x" => X,
                "y" => Y,
                "z" => Z,
                _ => panic!("Bad arg0!")
            };
            let op = match op {
                "inp" => Inp(arg0),
                "add"|"mul"|"div"|"mod"|"eql" => panic!("Not enough args!"),
                _ => panic!("Bad op!"),
            };
            prog.push_instruction(op);
        }
    }
    prog
}

fn parse_source_per_input(src: &str) -> Vec<Program> {
    let mut progs = Vec::new();

    for subsrc in src.split("add z y\n") {
        let mut prog = parse_source(subsrc);
        prog.push_instruction(Add(Z, Y));
        progs.push(prog);
    }

    progs
}

fn number_to_digits(n:i64) -> Vec<i64> {
    n.to_string().chars().map(|d| d.to_digit(10).unwrap() as i64).collect()
}

// The source is the same pattern repeated 14 times with 3 different parameters
// and one retained value from previous iterations, so it's faster just to hard
// code that and retain either the highest or lowest retained value that resolves
// to a new retained value. indexmap::IndexMap is a must because the insertion
// order is significant when looping over previous retained values.
fn the_smart_way(a:isize, b:isize, c:isize, zs:IndexMap<isize, String>, asc:bool) -> IndexMap<isize, String> {
    let mut res:IndexMap<isize, String> = IndexMap::new();
    for (zk, zv) in zs.into_iter() {
        for mut w in 1..=9 {
            w = if asc { 10 - w } else { w };
            let (mut x, mut y, mut z) = (0, 25, zk);
                                 // inp w
            x += z;              // add x z
            x %= 26;             // mod x 26
            z /= a;              // div z a <---- a
            x += b;              // add x b <---- b
            x = (x == w).into(); // eql x w
            x = (x == 0).into(); // eql x 0
            y *= x;              // mul y x
            y += 1;              // add y 1
            z *= y;              // mul z y
            y *= 0;              // mul y 0
            y += w;              // add y w
            y += c;              // add y c <---- c
            y *= x;              // mul y x
            z += y;              // add z y
            res.insert(z, format!("{}{}", zv, w));
        }
    }
    res
}

pub fn part_1() -> Option<String> {
    let mut res = IndexMap::new();
    res.insert(0, String::from(""));
    res = the_smart_way(1, 12, 9, res, false);
    res = the_smart_way(1, 12, 4, res, false);
    res = the_smart_way(1, 12, 2, res, false);
    res = the_smart_way(26, -9, 5, res, false);
    res = the_smart_way(26, -9, 1, res, false);
    res = the_smart_way(1, 14, 6, res, false);
    res = the_smart_way(1, 14, 11, res, false);
    res = the_smart_way(26, -10, 15, res, false);
    res = the_smart_way(1, 15, 7, res, false);
    res = the_smart_way(26, -2, 12, res, false);
    res = the_smart_way(1, 11, 15, res, false);
    res = the_smart_way(26, -15, 9, res, false);
    res = the_smart_way(26, -9, 12, res, false);
    res = the_smart_way(26, -3, 12, res, false);

    res.get(&0).map(String::from)
}

pub fn part_2() -> Option<String> {
    let mut res = IndexMap::new();
    res.insert(0, String::from(""));
    res = the_smart_way(1, 12, 9, res, true);
    res = the_smart_way(1, 12, 4, res, true);
    res = the_smart_way(1, 12, 2, res, true);
    res = the_smart_way(26, -9, 5, res, true);
    res = the_smart_way(26, -9, 1, res, true);
    res = the_smart_way(1, 14, 6, res, true);
    res = the_smart_way(1, 14, 11, res, true);
    res = the_smart_way(26, -10, 15, res, true);
    res = the_smart_way(1, 15, 7, res, true);
    res = the_smart_way(26, -2, 12, res, true);
    res = the_smart_way(1, 11, 15, res, true);
    res = the_smart_way(26, -15, 9, res, true);
    res = the_smart_way(26, -9, 12, res, true);
    res = the_smart_way(26, -3, 12, res, true);

    res.get(&0).map(String::from)
}

    /*
fn part_1() {
    // this is great and all, but actually building and running the APL
    // results in O(n) performance at best, so the whole puzzle is a red
    // herring to trigger a O(9**14) process
    let src = include_str!("monad.prog");
    let mut prog = parse_source(src);

    let mut is_valid = false;
    let mut last_d = 0;

    for m in 0..9999999 {
        let m2 = 9999999 - m;
        let a = number_to_digits(m2);
        if a.contains(&0) { continue; }

        for n in 0..9999999 {
            let n2 = 9999999 - n;
            let b = number_to_digits(n2);
            if b.contains(&0) { continue; }

            prog.reset();

            for i in a.iter() { prog.push_input(*i); }
            for j in b.iter() { prog.push_input(*j); }

            prog.run();

            is_valid = prog.get_result(Z) == Some(0);
            if is_valid {
                println!("Valid serial number: {}{}", m2, n2);
                break;
            }
            if last_d == 1_000_000 {
                println!("{}{}", m2, n2);
                last_d = 0;
            }
            last_d += 1;
        }   
    }
}
    */


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_simple_program() {
        let mut prog = Program::new();
        prog.push_instruction(Inp(X));
        prog.push_instruction(Add(X, Val(-1)));
        prog.push_input(0);
        prog.run();
        assert_eq!(prog.get_result(X), Some(-1));

        prog.reset();
        prog.push_input(100);
        prog.run();
        assert_eq!(prog.get_result(X), Some(99));
    }

    #[test]
    fn test_is_3x_program() {
        let mut prog = Program::new();
        prog.push_instruction(Inp(Z));
        prog.push_instruction(Inp(X));
        prog.push_instruction(Mul(Z, Val(3)));
        prog.push_instruction(Eql(Z, X));

        prog.push_input(1);
        prog.push_input(3);
        prog.run();
        assert_eq!(prog.get_result(Z), Some(1));

        prog.reset();
        prog.push_input(3);
        prog.push_input(1);
        prog.run();
        assert_eq!(prog.get_result(Z), Some(0))
    }

    #[test]
    fn test_binary_manipulation_program() {
        let mut prog = Program::new();
        prog.push_instruction(Inp(W));
        prog.push_instruction(Add(Z, W));
        prog.push_instruction(Mod(Z, Val(2)));
        prog.push_instruction(Div(W, Val(2)));
        prog.push_instruction(Add(Y, W));
        prog.push_instruction(Mod(Y, Val(2)));
        prog.push_instruction(Div(W, Val(2)));
        prog.push_instruction(Add(X, W));
        prog.push_instruction(Mod(X, Val(2)));
        prog.push_instruction(Div(W, Val(2)));
        prog.push_instruction(Mod(W, Val(2)));

        prog.push_input(0b101);
        prog.run();
        assert_eq!(prog.get_result(Z), Some(1));
        assert_eq!(prog.get_result(Y), Some(0));
        assert_eq!(prog.get_result(X), Some(1));
        assert_eq!(prog.get_result(W), Some(0));
    }
}
