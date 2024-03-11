// implementation of the Burrows-Wheeler Transform
fn rotate(s: &str) -> Vec<String> {
    let mut rotations = Vec::new();
    let mut s = s.to_string();
    for _ in 0..s.len() {
        rotations.push(s.clone());
        let last = s.pop().unwrap();
        s = last.to_string() + &s;
    }
    rotations
}

fn bwt(s: &str) -> String {
    // add $ to the end of the string
    let mut s = s.to_string();
    s.push('$');
    let mut rotations = rotate(s.as_str());
    rotations.sort();
    rotations.iter().map(|s| s.chars().last().unwrap()).collect()
}

fn inverse_bwt(s: &str) -> String {
    let mut table: Vec<String> = Vec::new();
    for _ in 0..s.len() {
        table.push(String::new());
    }
    for _i in 0..s.len() {
        for j in 0..s.len() {
            table[j].insert(0, s.chars().nth(j).unwrap());
        }
        table.sort();
    }
    table.iter().find(|x| x.ends_with("$")).unwrap().to_string()
}


fn main() {
    let s = "banana";
    let bwt = bwt(s);
    println!("{}", bwt);
    let ibwt = inverse_bwt(bwt.as_str());
    println!("{}", ibwt);
}
