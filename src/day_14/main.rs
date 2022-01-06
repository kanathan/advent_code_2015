use regex::Regex;
use std::{collections::HashMap, hash::Hash};

fn main() {
    let input = include_str!("input");

    let reindeer = parse_input(input);

    println!("{}",get_farthest(&reindeer, 2503));
    println!("{}",get_score(&reindeer, 2503));
}

fn get_farthest(reindeers: &Vec<Reindeer>, time: u32) -> u32 {
    let mut distances = Vec::new();

    for reindeer in reindeers {
        let full_segments = time / (reindeer.move_time + reindeer.rest_time);
        let time_remaining = time - (reindeer.move_time + reindeer.rest_time) * full_segments;
        let partial_move_time = reindeer.move_time.min(time_remaining);
        distances.push(reindeer.speed * reindeer.move_time * full_segments + partial_move_time * reindeer.speed);
    }

    distances.sort();
    *distances.iter().last().unwrap()
}

fn get_score(reindeers: &Vec<Reindeer>, time: u32) -> u32 {
    let mut distances = HashMap::new();
    let mut scores = HashMap::new();
    let mut timers = HashMap::new();

    for reindeer in reindeers {
        distances.insert(reindeer, 0);
        scores.insert(reindeer, 0);
        timers.insert(reindeer, (reindeer.move_time, true));
    }

    for _cur_time in 1..=time {
        let mut max_dist = 0;
        for reindeer in reindeers {
            /*let segment_time = cur_time % (reindeer.move_time + reindeer.rest_time + 1);
            if segment_time <= reindeer.move_time { 
                *distances.get_mut(reindeer).unwrap() += reindeer.speed;
            }
            if distances[reindeer] > max_record.0 {
                max_record = (distances[reindeer], &reindeer)
            }*/

            let timer = timers.get_mut(reindeer).unwrap();
            if timer.1 {
                *distances.get_mut(reindeer).unwrap() += reindeer.speed;
                timer.0 -= 1;
                if timer.0 == 0 {
                    timer.0 = reindeer.rest_time;
                    timer.1 = false;
                }
            } else {
                timer.0 -= 1;
                if timer.0 == 0 {
                    timer.0 = reindeer.move_time;
                    timer.1 = true;
                }
            }
            max_dist = max_dist.max(distances[reindeer]);
        }
        for (reindeer, dist) in distances.iter() {
            if *dist >= max_dist {
                *scores.get_mut(reindeer).unwrap() += 1;
            }
        }
    }

    *scores.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1
}

fn parse_input(input: &str) -> Vec<Reindeer> {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut reindeer = Vec::new();

    for line in input.lines() {
        let mut caps_iter = re.captures_iter(line);
        let new_reindeer = Reindeer {
            name: line.split_once(" ").unwrap().0.to_string(),
            speed: caps_iter.next().unwrap().get(1).unwrap().as_str().parse().unwrap(),
            move_time: caps_iter.next().unwrap().get(1).unwrap().as_str().parse().unwrap(),
            rest_time: caps_iter.next().unwrap().get(1).unwrap().as_str().parse().unwrap(),
        };
        reindeer.push(new_reindeer);        
    }

    reindeer
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Reindeer {
    name: String,
    speed: u32,
    move_time: u32,
    rest_time: u32,
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
    "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\n\
    Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

    #[test]
    fn test1() {
        let reindeer = parse_input(INPUT);

        assert_eq!(get_farthest(&reindeer, 1000), 1120);
    }

    #[test]
    fn test2() {
        let reindeer = parse_input(INPUT);

        println!("{}",get_farthest(&reindeer, 1000));

        assert_eq!(get_score(&reindeer, 1000), 689);

        
    }
}