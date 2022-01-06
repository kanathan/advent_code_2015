use serde_json::Value;

fn main() {
    let input = include_str!("input");

    let data: Value = serde_json::from_str(input).unwrap();

    let numbers = get_numbers(&data);
    let mut sum = 0;
    for num in numbers {
        sum += num;
    }
    println!("{}",sum);

    let numbers = get_numbers_no_red(&data);
    let mut sum = 0;
    for num in numbers {
        sum += num;
    }
    println!("{}",sum);
}

fn get_numbers(data: &Value) -> Vec<i64> {
    let mut output = Vec::new();
    match data {
        Value::Number(val) => output.push(val.as_i64().unwrap()),
        Value::Array(vec) => for val in vec.iter() { output.append(&mut get_numbers(val)) },
        Value::Object(obj) => for (_, val) in obj.iter() { output.append(&mut get_numbers(val)) },
        _ => (),
    }
    output
}

fn get_numbers_no_red(data: &Value) -> Vec<i64> {
    let mut output = Vec::new();
    match data {
        Value::Number(val) => output.push(val.as_i64().unwrap()),
        Value::Array(vec) => for val in vec.iter() { output.append(&mut get_numbers_no_red(val)) },
        Value::Object(obj) => {
            for (_, val) in obj.iter() {
                if *val == Value::String("red".to_string()) {
                    return output;
                }
            }
            for (_, val) in obj.iter() { output.append(&mut get_numbers_no_red(val)) }
        },
        _ => (),
    }
    output
}


#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test1() {
        fn dive(data: &Value) {
            println!("{:?}",data);
            if *data == Value::String("red".to_string()) {
                println!("Found Match!");
            }
            match data {
                Value::Number(val) => println!("Number: {}",val),
                Value::Array(vec) => for val in vec.iter() { dive(val) },
                Value::Object(obj) => for (_, val) in obj.iter() { dive(val) },
                _ => (),
            }
        }

        let input = r#"[1,{"c":"red","b":2},3]"#;

        let data: Value = serde_json::from_str(input).unwrap();
        dive(&data);
    }
}