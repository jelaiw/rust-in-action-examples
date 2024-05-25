fn main() {
    let needle = 0o204; // Funny octal. See author source code.
    let haystack = [1, 1, 2, 5, 15, 52, 132, 203, 877, 4140, 21147]; // Different haystack, too.

    for item in &haystack {
        if *item == needle {
            println!("{}", item);
        }
    }
}
