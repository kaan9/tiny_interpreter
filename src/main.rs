use std::collections::HashMap;
use std::io::{self, BufRead};

#[macro_use]
extern crate lazy_static;

struct GlobalState {
    vars: HashMap<String, i32>,
}

impl GlobalState {
    fn new() -> Self {
        GlobalState {
            vars: HashMap::new(),
        }
    }
    fn update_var(&mut self, name: String, new_val: i32) -> &mut Self {
        self.vars.insert(name, new_val);
        self
    }
    fn val(&self, name: &str) -> Option<i32> {
        match self.vars.get(name) {
            None => None,
            Some(&v) => Some(v),
        }
    }
}

mod match_expr {
    use regex::Regex;
    pub fn statement(line: &str) -> Result<(&str, i32), ()> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?sx)^(?:if)|(?:while) \s*
                    \((?P<cond>.+)\) \s*
                    $"
            )
            .unwrap();
        }
        match RE.captures(line) {
            None => Err(()),
            Some(cap) => Ok((
                cap.name("var").unwrap().as_str(),
                cap.name("val").unwrap().as_str().parse::<i32>().unwrap(),
            )),
        }
    }
    pub fn assignment(line: &str) -> Result<(&str, i32), ()> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^\s*(?P<var>[_a-zA-Z]\w*)\s*=\s*(?P<val>-?\d+)\s*$").unwrap();
        }
        match RE.captures(line) {
            None => Err(()),
            Some(cap) => Ok((
                cap.name("var").unwrap().as_str(),
                cap.name("val").unwrap().as_str().parse::<i32>().unwrap(),
            )),
        }
    }
    pub fn printvar(line: &str) -> Result<&str, ()> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\s*print\s+(?P<var>[_a-zA-Z]\w*)\s*$").unwrap();
        }
        match RE.captures(line) {
            None => Err(()),
            Some(cap) => Ok(cap.name("var").unwrap().as_str()),
        }
    }
    pub fn printval(line: &str) -> Result<i32, ()> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\s*print\s+(?P<val>-?\d+)\s*$").unwrap();
        }
        match RE.captures(line) {
            None => Err(()),
            Some(cap) => Ok(cap.name("val").unwrap().as_str().parse::<i32>().unwrap()),
        }
    }
    // pub fn ifcond(line: &str) -> Result<(&str, &str), ()> {
    //     lazy_static! {
    //         static ref RE: Regex = Regex::new(
    //             r"(?x)
    //         ^\s*if\s*(\s*(?P<val>[(-?\d+)()])\s*$"
    //         )
    //         .unwrap();
    //     }
    //     match RE.captures(line) {
    //         None => Err(()),
    //         Some(cap) => Ok(cap.name("val").unwrap().as_str().parse::<i32>().unwrap()),
    //     }
    // }
}

fn interp_statement(gstate: &mut GlobalState, input: &str) {
    if let Ok((var, val)) = match_expr::assignment(&input) {
        gstate.update_var(String::from(var), val);
        println!("set {} to {}", var, val)
    } else if let Ok(var) = match_expr::printvar(&input) {
        match gstate.val(var) {
            None => println!("unknown variable {}", var),
            Some(val) => println!("{}", val),
        }
    } else if let Ok(val) = match_expr::printval(&input) {
        println!("{}", val)
    } else if input.len() != 0 {
        println!("Syntax error")
    }
}

fn main() {
    let mut gstate = GlobalState::new();
    let stdin = io::stdin();

    //std::fs::File::open("/f").unwrap()
    let mut input = String::new();
    let mut in_scope: bool = false;
    for line in stdin.lock().lines().map(|line| line.unwrap()) {
        input += &line;
        if in_scope {
            if line.contains(r"}") {
                in_scope = false;
                {}
            }
        } else {
            if line.contains(r"{") {
                in_scope = true;
                continue;
            }
        }
        interp_statement(&mut gstate, &input);
        input.clear();
    }
}
