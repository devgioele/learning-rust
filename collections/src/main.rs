use std::collections::HashMap;

fn main() {
    playing_with_vectors();
    playing_with_strings();
    playing_with_hashmaps();
}

fn playing_with_vectors() {
    // -- Create a new vector
    // Using the macro
    let mut v = vec![3, 4];
    // Manually
    // let mut v = Vec::new();
    // v.push(3);
    // v.push(4)

    // -- Append another vector
    let v2 = vec![100, 307];
    // v2.as_slice() is necessary, because the compiler cannot
    // coerce Vec<T> to &[T], as it would with String and &str.
    v.extend_from_slice(v2.as_slice());
    // TODO: Why does it not matter if I use &v2.as_slice()?
    println!("v2 = {:?}", v2);

    // -- Modify the elements of the vector
    // Using explicitly a mutable borrow
    let first = &mut v[0];
    *first += 3;
    // One liner for the same thing
    *&mut v[0] = *&mut v[0] + 3;
    // Using the mutable indexing operation
    v[1] += 1;
    // This would not work, because &v[1] is an immutable borrow
    // and we cannot use it to assign a value to the borrowed
    // element
    // *&v[1] += 1;

    println!("v = {:?}", v);
}

fn playing_with_strings() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // The format macro doesn't take ownership of any string
    let concat1 = format!("{}-{}-{}", s1, s2, s3);
    // Signature of the add function:
    // fn add(self, s: &str) -> String
    // Notice that it takes ownership of self,
    // making s1 invalid after the following statement.
    let concat2 = s1 + "-" + &s2 + "-" + &s3;
    println!("s1 = {}\ns2 = {}", concat1, concat2);
}

fn playing_with_hashmaps() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    // The method `zip` combines the current iterator with another given iterator
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    // Print score of team blue
    print_score(&scores, "Blue");
    // Overwriting a value
    scores.insert("Blue".to_string(), 18);
    // Inserting score for team yellow and red only if not present yet
    scores.entry("Yellow".to_string()).or_insert(77);
    scores.entry("Red".to_string()).or_insert(77);
    // Print all scores
    for (key, value) in &scores {
        println!("Team {}: {}", key, value);
    }
    // Multiplying score of team Red
    let score_red = scores.entry("Red".to_string()).or_insert(0);
    *score_red *= 2;
    // Printing all scores with a more functional approach
    scores
        .iter()
        .for_each(|(key, value)| println!("Team {}: {}", key, value));
}

fn print_score(scores: &HashMap<String, i32>, team: &str) {
    if let Some(score_blue) = scores.get("Blue") {
        println!("Score of team {}: {}", team, score_blue);
    } else {
        println!("Team {} does not exist.", team);
    }
}
