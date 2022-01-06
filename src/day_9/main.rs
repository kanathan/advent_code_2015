use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input");

    println!("Shortest: {}",get_shortest_route(input));
    println!("Longest: {}",get_longest_route(input));
}


fn get_shortest_route(input: &str) -> usize {
    let route_map = parse_input(input);
    let mut routes = get_routes(&route_map);

    routes.sort_by(|a, b| a.1.cmp(&b.1) );
    routes[0].1
}

fn get_longest_route(input: &str) -> usize {
    let route_map = parse_input(input);
    let mut routes = get_routes(&route_map);

    routes.sort_by(|a, b| a.1.cmp(&b.1) );
    routes.iter().last().unwrap().1
}


fn get_routes(route_map: &HashMap<String,Vec<(String, usize)>>) -> Vec<(String, usize)> {

    fn route_step(
        src: &str,
        route_map: &HashMap<String,Vec<(String, usize)>>,
        visited: &HashSet<String>,
        route_str: &str,
        dist: usize) -> Vec<(String, usize)> {
        
        let mut routes = Vec::new();
        for (dst, new_dist) in route_map.get(src).unwrap() {
            if !visited.contains(dst) {
                let route_str = format!("{} -> {}", route_str, dst);
                let dist = dist + new_dist;
                let mut visited = visited.clone();
                visited.insert(dst.to_string());
                if visited.len() == route_map.len() {
                    return vec![(route_str, dist)];
                }
                routes.append(&mut route_step(dst, route_map, &visited, &route_str, dist));
            }
        }
        routes
    }

    let mut routes = Vec::new();
    for src in route_map.keys() {
        routes.append(&mut route_step(src, route_map, &HashSet::from([src.clone()]), src, 0));
    }

    routes
}


fn parse_input(input: &str) -> HashMap<String,Vec<(String, usize)>> {
    let re = Regex::new(r"^(\w+) to (\w+) = (\d+)").unwrap();
    let mut route_map = HashMap::new();

    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let src = caps.get(1).unwrap().as_str();
        let dst = caps.get(2).unwrap().as_str();
        let dist = caps.get(3).unwrap().as_str().parse().unwrap();
        route_map.entry(src.to_string()).or_insert(Vec::new()).push((dst.to_string(), dist));
        route_map.entry(dst.to_string()).or_insert(Vec::new()).push((src.to_string(), dist));
    }

    route_map
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
    "London to Dublin = 464\n\
    London to Belfast = 518\n\
    Dublin to Belfast = 141";


    #[test]
    fn test1() {
        assert_eq!(get_shortest_route(INPUT), 605);
    }

    #[test]
    fn test2() {
        assert_eq!(get_longest_route(INPUT), 982);
    }
}