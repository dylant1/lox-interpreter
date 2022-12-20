pub fn error(line: isize, message: &str) -> bool {
   println!("Test error");
   report(line, "", message);
   true
}

pub fn report(line: isize, where_: &str, message: &str) {
    println!("Error at line {}: {} {}", line, where_, message);
}

