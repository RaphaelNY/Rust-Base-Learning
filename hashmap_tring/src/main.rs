use std::collections::HashMap;

fn main() {
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
     /// new a hashmap<K,V>. saved in heap, not stack. All K and T need to be the same type.
     /// let scores = HashMap::new();
     /// scores.insert(String::from("Blue"), 10); error, need mut
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let intial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(intial_scores.iter()).collect();
     /// collect way:
     /// let C: HashMap<A, B> = A.inter().zip(B.iter()).collect();
    println!("{:#?}", scores);
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
     /// ownership of hashmap
     /// like i32,these saved in stack, so they have Copy trait. in hashmap, they are copy to hashmap.
     /// like string, these saved in heap, so they have Move trait. in hashmap, they are moved to hashmap.
    let file_name: String = String::from("favoutite_color");
    let file_value: String = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(&file_name, &file_value);
    println!("{:#?}", map);
    println!("{}", file_name); // file_name is not moved to map, so it can be used again.
     /// println!("{:#?}", file_name); error, file_name is moved to map.
     /// or you can used this : map.insert(file_name.clone(), file_value.clone()); to keep the file_name and file_value's ownership.
     /// if you just insert a &str...,in the lifetime of hashmap, the &str... need to be alive.in hashmap just saved &, the value was left in the raw place.
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
     /// get value from hashmap
     /// get() return Option<&V>, input is &_.
     /// if the key is not in the hashmap, return None.
     /// if the key is in the hashmap, return Some(&V)
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(i) => println!("{}", i),
        None => println!("team not found"),
    }
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
     /// loop hashmap
     /// for (key, value) in &hashmap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
     /// update hashmap
     /// insert() will insert a new key-value pair to hashmap.
     /// if the key is exist, the value will be updated.
     /// if the key is not exist, the key-value pair will be inserted.
     /// entry() will return a enum Entry, which has two value: Occupied and Vacant.
     /// Occupied has a method: or_insert(), which will insert a new key-value pair to hashmap.
     /// Vacant has a method: insert(), which will insert a new key-value pair to hashmap.
     /// or_insert() and insert() will return a mutable reference to the value of the key.
     /// if the key is exist, the value will be updated.
     /// if the key is not exist, the key-value pair will be inserted.
    println!("{:#?}", scores);
    scores.insert(String::from("Blue"), 25);
    println!("{:#?}", scores);

     // scores.entry(String::from("Blue")).or_insert(50);
    let e = scores.entry(String::from("Red")); // e is a enum Entry, initial to be a mutable borrow.
    println!("{:#?}", e);
    e.or_insert(50);
    scores.entry(String::from("Red")).or_insert(60);
    println!("{:#?}", scores);
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
     /// or_insert will return a mutable reference to the value of the key.
     /// so you can use this to update the value.
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
             // count is a mutable reference point to the value of the key(just like a pointer,point to the value of 'word').
             // .entry().or_insort() will return a mutable reference to the value of the key.
        *count += 1; // *count is the value of the key.
    }
    println!("{:#?}", map);
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
     // hash function
     // hash function is a way to convert a key to a index of array. defend the DoS attack.but it is not the fastest way.
     // you can use hasher to change the hash function of hashmap.
}
