use std::collections::BTreeMap;
// fn main() {
//     let mut users = BTreeMap::new();
//     users.insert(101, 24);
//     users.insert(102, 2);
//     users.insert(103, 5);
//     println!("user are {:?}", users);
// }

#![allow(unused)]
fn main() {
    use std::collections::BTreeMap;

    // type inference lets us omit an explicit type signature (which
    // would be `BTreeMap<&str, &str>` in this example).
    let mut movie_reviews = BTreeMap::new();

    // review some movies.
    movie_reviews.insert("Office Space",       "Deals with real issues in the workplace.");
    movie_reviews.insert("Pulp Fiction",       "Masterpiece.");
    movie_reviews.insert("The Godfather",      "Very enjoyable.");
    movie_reviews.insert("The Blues Brothers", "Eye lyked it a lot.");

    // check for a specific one.
    // if !movie_reviews.contains_key("Les Misérables") {
    //     println!("sorry we dont have Les Miserables movie review");
    // }

    // oops, this review has a lot of spelling mistakes, let's delete it.
    movie_reviews.remove("The Blues Brothers");

    // look up the values associated with some keys.
    let wanted_list_movies = ["avengers", "Office Space","The Godfather"];

    // println!("wanted list movies are {:?}",wanted_list_movies);

            println!("{:?}",movie_reviews.get("Office Space"));


    // for movie in &wanted_list_movies {
    //     println!("finding reviews for movie: {:?}",movie);



    // }



    for movie in &wanted_list_movies {
        match movie_reviews.get(movie) {
          Some(review) => println!("{movie}: {review}"),
          None => println!("{movie} is unreviewed.")
        }
    }

    // // Look up the value for a key (will panic if the key is not found).
    // println!("Movie review: {}", movie_reviews["Office Space"]);

    // // iterate over everything.
    // for (movie, review) in &movie_reviews {
    //     println!("{movie}: \"{review}\"");
    // }
}

/*
 *
 *              STACK
┌──────────────┐
│    users     │
│              │
│ ptr ─────────┼──────────────┐
│ len          │              │
│ capacity     │              │
└──────────────┘              │
                          ↓
                      HEAP TABLE
        ┌──────────────────────────┐
        │ CONTROL                  │
        │                          │
        │ [C][C][C][C][C][C]       │
        │                          │
        │ KEY/VALUE DATA           │
        │                          │
        │ 101 → Aariv              │
        │ 102 → Koel               │
        │ 103 → Phunsuk            │
        └──────────────────────────┘
* HashMap
    ↓
HASH
    ↓
"WHERE SHOULD THIS KEY BE?"
    ↓
O(1) average


BTreeMap
    ↓
SORTED TREE
    ↓
"WHICH BRANCH SHOULD I TAKE?"
    ↓
O(log n)
*
*
*
 */
