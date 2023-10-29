fn main() {
    println!("Hello, world!");

    let haystack1 = "hello world";
    let needle1 = "world";
    let haystack2 = "Hello World";

    run_search("naive_search", haystack1, needle1, naive_search);
    run_search("naive_search", haystack2, needle1, naive_search);
    run_search("sum_search", haystack1, needle1, sum_search);
    run_search("sum_search", haystack2, needle1, sum_search);
    run_search("std_search", haystack1, needle1, std_search);
    run_search("std_search", haystack2, needle1, std_search);
    run_search("weird_search", haystack1, needle1, weird_search);
    run_search("weird_search", haystack2, needle1, weird_search);
}

fn run_search<F>(name: &str, haystack: &str, needle: &str, search_fn: F)
where
    F: Fn(&str, &str) -> bool,
{
    let result = search_fn(haystack, needle);
    println!("{} result: {}", name, result);
}

// Define your search functions here
fn naive_search(haystack: &str, needle: &str) -> bool {
    let [haystack, needle] = [haystack, needle].map(str::as_bytes);

    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

// Time complexity: O(H * N) // same as naive
// Space complexity: O(1)    // same as naive
fn sum_search(haystack: &str, needle: &str) -> bool {
    // Treat corner cases
    if needle.is_empty() {
        return true;
    } else if needle.len() >= haystack.len() {
        return haystack == needle;
    }

    let [haystack, needle] = [haystack, needle].map(str::as_bytes);

    let mut windows = haystack.windows(needle.len());

    // Unwrap Safety:
    //   We know that `0 < needle.len() < haystack.len()`, there is at
    //   least one window.
    let first_window = windows.next().unwrap();

    let sum_slice = |slice: &[u8]| -> u64 { slice.iter().copied().map(u64::from).sum() };

    let needle_sum = sum_slice(needle);
    let mut window_sum = sum_slice(first_window);

    // Short-circuit the expensive check to skip it
    if needle_sum == window_sum && first_window == needle {
        return true;
    }

    // Now, for the rest of the windows.
    for (removed_element_index, window) in windows.enumerate() {
        // Unwrap Safety:
        //   We know that `needle.len() > 0`, every window is non-empty.
        window_sum += *window.last().unwrap() as u64;
        window_sum -= haystack[removed_element_index] as u64;

        // If the sum doesn't match, skip the check
        if needle_sum != window_sum {
            continue;
        }
        // Check equality (expensive check)
        if window == needle {
            return true;
        }
    }

    false
}

fn std_search(haystack: &str, needle: &str) -> bool {
    haystack.contains(needle)
}

fn weird_search(haystack: &str, needle: &str) -> bool {
    let [haystack, needle] = [haystack, needle].map(str::as_bytes);

    let sum_slice = |slice: &[u8]| -> u64 { slice.iter().copied().map(u64::from).sum() };
    let needle_sum = sum_slice(needle);

    haystack.windows(needle.len()).any(|haystack_window| {
        let window_sum = sum_slice(haystack_window);
        window_sum == needle_sum && haystack_window == needle
    })
}
