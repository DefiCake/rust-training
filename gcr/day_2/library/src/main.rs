#[derive(Debug)]
struct Library {
  books: Vec<Book>,
}

#[derive(Debug, Clone)]
struct Book {
  title: String,
  year: u16,
}

impl Book {
  fn new(title: &str, year: u16) -> Book {
    Book {
      title: String::from(title),
      year,
    }
  }
}

impl Library {
  fn new() -> Library {
    Library { books: Vec::new() }
  }

  fn len(&self) -> usize {
    self.books.len()
  }

  fn is_empty(&self) -> bool {
    self.books.is_empty()
  }

  fn add_book(&mut self, book: Book) {
    self.books.push(book);
  }

  fn print_books(&self) {
    for (i, book) in self.books.iter().enumerate() {
      println!("Book {}: {:?}", i, book);
    }
  }

  fn oldest_book(&self) -> Option<&Book> {
    if self.is_empty() {
      return None;
    }

    let mut oldest_book: &Book = &self.books[0];

    for book in self.books.iter().skip(1) {
      if book.year < oldest_book.year {
        oldest_book = book;
      }
    }

    Some(oldest_book)
  }
}

fn main() {
  let mut library = Library::new();

  println!("The library is empty: library.is_empty() -> {}", library.is_empty());

  library.add_book(Book::new("Lord of the Rings", 1954));
  library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
  library.add_book(Book::new("Critique of Pure Reason", 1781));

  println!("The library is no longer empty: library.is_empty() -> {}", library.is_empty());

  library.print_books();

  match library.oldest_book() {
    Some(book) => println!("The oldest book is {}", book.title),
    None => println!("The library is empty!"),
  }

  println!("The library has {} books", library.len());
  library.print_books();
}
