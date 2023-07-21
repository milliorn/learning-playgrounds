use partition::partition;

pub fn sort(input: &mut [i32]) {
    let mut scratchpad = vec![0; input.len()];

    sort_help(input, &mut scratchpad)
}

fn sort_help(initial: &mut [i32], scratchpad: &mut [i32]) {
    let mut stack = vec![0..initial.len()];

    while let Some(range) = stack.pop() {
        let (start, end) = (range.start, range.end);
        let input = &mut initial[start..end];

        if input.len() <= 1 {
            continue;
        }

        // returns the number of elements less than or equal to the pivot
        let n = partition(input, scratchpad);

        if n == input.len() {
            // all elements smaller than or equal to the pivot
            // i.e. the pivot is in the correct position in the array
            stack.push(start..end - 1);
        } else {
            stack.push(start..start + n);
            stack.push(start + n..end);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
