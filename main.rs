
#[derive(Debug)]
enum Publication {
    Book(Book),
    Magazine(Magazine),
}


#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    page_count: u32,
}


#[derive(Debug)]
struct Magazine {
    title: String,
    issue: u32,
    topic: String,
}

fn main() {
    let publications: Vec<Publication> = vec![
        Publication::Book(Book {
            title: String::from("Rust Programlama"),
            author: String::from("Mustafa KILIÇ"),
            page_count: 300,
        }),
        Publication::Magazine(Magazine {
            title: String::from("Bilgi Dergisi"),
            issue: 42,
            topic: String::from("Programlama"),
        }),
    ];

    for publication in &publications {
        match publication {
            Publication::Book(book) => {
                println!(
                    "Kitap: {} yazar: {}, {} sayfa",
                    book.title, book.author, book.page_count
                );
            }
            Publication::Magazine(magazine) => {
                println!(
                    "Dergi: {} - Sayı: {}, Konu: {}",
                    magazine.title, magazine.issue, magazine.topic
                );
            }
        }
    }
}
