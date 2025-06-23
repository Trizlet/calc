use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty(){
        eprintln!("Usage: Calc <expression>");
        return;
    }

    let expression = args.join(" ").trim().to_string();
    // println!("{}",expression);

    match meval::eval_str(expression){
        Ok(result)=>{
            if !result.is_finite() || result.abs()>1e12{
                    eprintln!("Result is undefined or overflowed");
                    return;
            }

            let rounded = (result*1e9).round()/1e9;

            if (rounded - rounded.trunc()).abs()<1e-9{
                println!("{}",rounded.trunc() as i64);
            }
            else {
                println!("{}",rounded);
            }
        }
        Err(e)=>{
            eprintln!("Error: {}",e);
        }
    }
}
