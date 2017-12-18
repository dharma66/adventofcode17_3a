fn main() {
    let answer = resolve(325489);
    println!("The answer is: {}", answer);
}

fn resolve(num: u32) -> u32{
    let dimension = get_dimension(num);
println!("Dimension = {}", dimension);

    let low_bound = (dimension - 2) * (dimension - 2) + 1;
    let high_bound = dimension * dimension;

println!("low_bound = {}, high_bound = {}", low_bound, high_bound);
    assert!((num >= low_bound) && (num <= high_bound));

    let distance_a = (dimension - 1) / 2;

    let distance_b = get_quadrant_offset(num, low_bound, high_bound);

println!("distance_a: {}, distance_b: {}", distance_a, distance_b);    
    distance_a + distance_b
}

fn get_dimension(num: u32) -> u32{
    let mut result = 0;
    let mut root = 1;

    while result < num{
        root += 2;
        result = root * root;
    }
    root
}

fn get_quadrant_offset(num: u32, low: u32, high: u32) -> u32{
println!("num: {}", num);

    let quad_len = (high + 1 - low) / 4;

println!("quad_len: {}", quad_len);

    let quadrant;

    if num < low + quad_len{ quadrant = 1;}
    else if num < low + quad_len * 2 { quadrant = 2;}
    else if num < low + quad_len * 3 { quadrant = 3;}
    else { quadrant = 4;};

println!("quadrant: {}", quadrant);

    let start = low + (quad_len * (quadrant - 1));
    let end = low + (quad_len * quadrant) - 1;

    let mid = (end - start) / 2 + start;

println!("start: {}, end: {}, mid: {}", start, end, mid);
    if num > mid {return num - mid;}
    else if num < mid {return mid - num;}
    
    0
}



#[test]
fn test_resolve()
{
    assert_eq!(resolve(12), 3);
    assert_eq!(resolve(23), 2);
    assert_eq!(resolve(1024), 31);
}
