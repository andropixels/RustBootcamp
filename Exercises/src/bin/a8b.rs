//Topic: Getting used to with Struct 

//
//Requirements
//
/*
1- Define a struct Book having two fields author:Author and price:i32, 
2- Define a struct Author having two fields one is name and stars, 
3- you gonna purchase a book only if author has rating >= 4.9 stars .

*/


#[derive (Debug)]
struct Book {
author: Author,
price: i32
}

#[derive (Debug)]
struct Author {
name: String,
stars: f32
}

fn main() {
let mut books_present = Vec::new();
let author_instance = create_author("Morgan Housel".to_owned(), 4.5);
let create_book_instance1 = create_book(author_instance, 399);

let author_instance2 = create_author("James clear".to_owned(), 5.0);
let create_book_instance2 = create_book(author_instance2,499);

books_present.push(create_book_instance1);
books_present.push(create_book_instance2);

for books in books_present {
if books.author.stars >= 4.9 {
println!("{:?}", books);
}
}



}

fn create_book(author: Author, price: i32) -> Book{
Book {author, price}
}

fn create_author(name: String,stars: f32) -> Author {
Author{name, stars}
}