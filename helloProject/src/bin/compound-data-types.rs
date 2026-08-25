// Compound Data Types
// arrays, tuples, slices and strigs(slice strings)
// Arrays
fn main() {
    // Arrays - homogenous collection of elements of fixed size e.g only int arr, only str arr
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Number Array: {:?}", numbers);
    
    let fruits: [&str; 3] = ["apple", "banana", "orange"];
    println!("Fruits Arr: {:?}", fruits);
    println!("Fruits Arr element 1: {}", fruits[0]);
    println!("Fruits Arr element 2: {}", fruits[1]);
    println!("Fruits Arr element 3: {}", fruits[2]);

    // Tuples - heterogenous collection of element of a fixed size
    let human: (String, i32, bool) = ("Alice".to_string(), 30, true);
    println!("Human tuple: {:?}", human);

    let my_mix_tuple = ("Kratos", 23, true, [1,2,3,5]);
    println!("My Mix Tuple: {:?}", my_mix_tuple);

    // Slices - dynamically sized 
    let number_slices: &[i32] = &[1,2,3,4,5];
    println!("Number Slice: {:?}", number_slices);

    let animal_slices: &[&str] = &["cat", "goat", "dog"];
    println!("Animal Slice: {:?}", animal_slices);

    let book_slices: &[&String] = &[&"Harry Potter".to_string(), &"Wimpy Kid".to_string(), &"overgeared".to_string()];
    println!("Book Slice: {:?}", book_slices);

    // Strings vs String Slices (&str) -     
    // Strings are stored on heap. growable or expandable, you can increase or decrease, hence mutable. You can push and delete data from a certain var if you want. Owned string types.
    // String slice stored on the stack, can't have mutable data types. not an owned string. is a reference to string stored somewhere in your code. immutable. used to reference string literals or substrings of sting objects without needing to copy or own the data. specfic size
    // Stack is quicker, heap is slower...stack can't have any mutable dt, heap can have dynamic mutable dt
    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says:{}", stone_cold);

    // B- &str ( String Slice)
    let string: String = String::from("Hello, world!");
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice);
}
