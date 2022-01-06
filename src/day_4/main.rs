use md5;
use std::sync::{Arc, Mutex};
use std::time;
use std::thread;
use std::sync::mpsc;

fn main() {
    /*let time_now = time::Instant::now();
    let key = "yzbqklnj";
    let mut val = 0;
    loop {
        let full_key = format!("{}{}",key,val);
        let hash_string = format!("{:?}",md5::compute(full_key));
        if &hash_string[0..6] == "000000" {
            break;
        }
        val += 1;
    }
    let elapsed_time = time_now.elapsed();
    println!("Value is {}",val);
    println!("Took {} secs",elapsed_time.as_secs_f32());*/

    let partial_key = "yzbqklnj";
    calculate_hash(partial_key, "000000");
}


fn calculate_hash(partial_key: &str, hash_start: &str) {
    let time_now = time::Instant::now();
    let thread_count = 6;
    let answer = Arc::new(Mutex::new(None));
    let cur_val = Arc::new(Mutex::new(0));
    let (tx,rx) = mpsc::channel();
    let mut handles = Vec::new();

    for _ in 0..thread_count {
        let tx = tx.clone();
        let answer = answer.clone();
        let cur_val = cur_val.clone();
        let partial_key = partial_key.to_string();
        let hash_start = hash_start.to_string();

        handles.push(thread::spawn(move || {
            loop {
                {
                    let answer = answer.lock().unwrap();
                    if answer.is_some() { break };
                }
                let key_val; 
                {
                    let answer = answer.lock().unwrap();
                    if answer.is_some() { break };
                    let mut cur_val = cur_val.lock().unwrap();
                    key_val = *cur_val;
                    *cur_val += 1;
                }
                let full_key = format!("{}{}",partial_key,key_val);
                let hash_string = format!("{:?}",md5::compute(full_key));
                if hash_string.starts_with(&hash_start) {
                    println!("Found an answer: Val = {}, Hash = {}",key_val,hash_string);
                    {
                        let mut answer = answer.lock().unwrap();
                        if answer.is_none() {
                            *answer = Some(key_val);
                        } else if answer.unwrap() > key_val {
                            *answer = Some(key_val);
                        }
                        
                    }
                    break;
                }
            }
            tx.send(()).unwrap();
        }));

    }

    loop {
        if rx.recv_timeout(time::Duration::from_secs(5)).is_ok() {
            break;
        }
        let latest_value = {
            *cur_val.lock().unwrap()
        };
        println!("{}: Cur value = {}",time_now.elapsed().as_secs_f32(),latest_value);
    }
    
    println!("Found an answer! Waiting for other threads to complete...");
    for handle in handles {
        handle.join().unwrap();
    }
    println!("All threads complete!");

    {
        let answer = answer.lock().unwrap();
        println!("Answer is {}",answer.unwrap());
    }
    let elapsed_time = time_now.elapsed();
    println!("Took {} secs",elapsed_time.as_secs_f32());
}


#[cfg(test)]
mod test {
    


    #[test]
    fn test1() {
        
    }
}