fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: generate_ast <output directory>");
        process::exit(64);
    }

    let output_dir = &args[1];

    define_ast(output_dir, "Expr", &[
        ("Binary", vec!["left: Box<Expr>", "operator: Token", "right: Box<Expr>"]),
        ("Grouping", vec!["expression: Box<Expr>"]),
        ("Literal", vec!["value: Option<String>"]),
        ("Unary", vec!["operator: Token", "right: Box<Expr>"]),
    ]);
}

fn define_ast(dir: &str, base_name: &str, types: &[(&str, Vec<&str>)]) {


}

fn define_type (file: &mut File, base_name: &str, class_name: &str, fields: &[&str]) {

}
