fn skewer_decipherer(skewer_array: Vec<&str>) -> Vec<u8> {
    let mut my_vec: Vec<u8> = vec![0, 0];

    for kebab in skewer_array {
        if kebab.contains('x') {
            my_vec[1] += 1;
        } else {
            my_vec[0] += 1;
        }
    }
    return my_vec;
}

fn main() {
    let skewer = vec![
        "--xo--x--ox--",
        "--xx--x--xx--",
        "--oo--o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--",
    ];
    let res = skewer_decipherer(skewer);
    println!(
        "We have {} vegetarian and {} non-veg skewers",
        res[0], res[1]
    );
}
