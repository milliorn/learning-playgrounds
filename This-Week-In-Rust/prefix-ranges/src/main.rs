// Importing the BTreeSet data structure from the standard library
use std::collections::BTreeSet;

// The main function is where the program starts executing
fn main() {
    // We create a set of words inside curly braces {}
    let set = {
        // Inside the set, we create a new empty set called 'set' using BTreeSet
        let mut set = BTreeSet::new();

        // We start adding some words to the set using the insert function
        set.insert("ABC");
        set.insert("DEF");
        set.insert("DEG");
        set.insert("HIJ");
        set.insert("KLM");
        set.insert("NOP");

        // Finally, we return the completed set
        set
    };

    // Now, we use a loop to go through each word in the set
    // We start at the word "DEF" and go up to, but not including, the word "N"
    for elem in set.range("DEF".."N") {
        // Inside the loop, we print each word on a separate line
        println!("{elem}");
    }

    // Create a new BTreeSet and insert some strings into it
    let mut set = BTreeSet::new();
    set.insert("apple".to_owned());
    set.insert("banana".to_owned());
    set.insert("cherry".to_owned());
    set.insert("orange".to_owned());
    set.insert("peach".to_owned());
    set.insert("plum".to_owned());

    // Call the 'prefixed' function to get a new set with only the elements that start with "o"
    let prefix = "o";
    let result_set = prefixed(set, prefix);

    // Print the elements in the resulting set
    for elem in result_set {
        println!("{}", elem);
    }

    let prefix = "abc";
    let upper_bound = upper_bound_from_prefix(prefix);

    println!("Prefix: {}", prefix);
    println!("Upper Bound: {:?}", upper_bound); // Prints "abcd"
}

fn prefixed(mut set: BTreeSet<String>, prefix: &str) -> BTreeSet<String> {
    // Split off the elements in the set that have the given prefix
    let mut set = set.split_off(prefix);

    // Find the first element in the set that does not start with the given prefix
    let not_in_prefix = (&set).iter().find(|s| !s.starts_with(prefix));

    // If such an element is found, store it in the variable 'not_in_prefix' as an owned String
    let not_in_prefix = not_in_prefix.map(|s| s.to_owned());

    // If 'not_in_prefix' is Some, meaning there is an element that does not start with the prefix,
    // split off all elements after that element from the set
    if let Some(not_in_prefix) = not_in_prefix {
        set.split_off(&not_in_prefix);
    }

    // Return the modified set with only the elements that have the given prefix
    set
}

fn upper_bound_from_prefix(prefix: &str) -> Option<String> {
    // Iterate over the indices of the characters in the prefix in reverse order
    for i in (0..prefix.len()).rev() {
        // Check if we can get the substring from the current index to the end of the prefix
        if let Some(last_char_str) = prefix.get(i..) {
            // Create a slice containing the rest of the prefix from the beginning to the current index
            let rest_of_prefix = {
                // Make sure that the current index is at a valid char boundary
                debug_assert!(prefix.is_char_boundary(i));
                &prefix[0..i]
            };

            // Extract the first character from the last_char_str substring
            let last_char = last_char_str
                .chars()
                .next()
                .expect("last_char_str will contain at least one char");

            // Increment the last character to get the next possible character
            let last_char_incr = (last_char..=char::MAX).nth(1);

            if let Some(last_char_incr) = last_char_incr {
                // Create a new string by combining the rest of the prefix and the incremented last character
                let new_string = format!("{rest_of_prefix}{last_char_incr}");

                // Return the new string as Some, indicating success
                return Some(new_string);
            } else {
                // Last character is the highest possible code point.
                // Go to the second-to-last character and continue the loop.
                continue;
            }
        }
    }

    // If no valid string was found, return None
    None
}
