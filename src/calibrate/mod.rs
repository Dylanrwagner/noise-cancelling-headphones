//Calculate the delay caused by the system, and the amplitude of the sound that leaks
// into the headphones

/// Find the local maxima of the given vector. Return the numbers and
/// their indices
fn local_maxima(input: Vec<f32>) -> Vec<(f32, usize)> {
    if input.len() == 0 {
        return Vec::new();
    }
    else if input.len() == 1 {
        return [(input[0], 0)].to_vec();
    }
    let mut result = Vec::new();
    if input[0] >= input[1] {
        result.push((input[0], 0));
    }
    for i in 1..input.len()-1 {
        if input[i] >= input[i-1] && input[i] >= input[i+1] {
            result.push((input[i], i));
        }
    }
    if input[input.len()-1] >= input[input.len()-2] {
        result.push((input[input.len()-1], input.len()-1));
    }
    result
}

#[test]
fn test_local_maxima() {
    assert_eq!(local_maxima(Vec::new()), Vec::new());
    assert_eq!(local_maxima([1.0,3.0,2.0].to_vec()), [(3.0, 1)].to_vec());
    assert_eq!(local_maxima([5.0,2.0].to_vec()), [(5.0,0)].to_vec());
    assert_eq!(local_maxima([1.0].to_vec()), [(1.0,0)].to_vec());
    assert_eq!(local_maxima([1.0, 3.0, 3.0, 2.0].to_vec()), [(3.0,1),(3.0,2)].to_vec());
}
