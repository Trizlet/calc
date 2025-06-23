use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty(){
        eprintln!("Usage: Calc <expression>");
        return;
    }

    let expression = args.join(" ");
    let expression = expression.trim();

    // println!("{}",expression);

    match meval::eval_str(expression){
        Ok(result)=>println!("{}",result),
        Err(e)=>eprintln!("Error: {}",e),
    }
}
