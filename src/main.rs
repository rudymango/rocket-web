#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use rocket::response::content::Json;
use rocket::request::Form;
use rocket::Request;
use rocket_contrib::templates::Template;
use serde::Serialize;

#[get("/")]
fn index() -> Template {
	#[derive(Serialize)]
	struct Context {
		first_name: String,
		last_name: String
	}	
	let context = Context {
		first_name: String::from("Jane"),
		last_name: String::from("Doe")
	};
	Template::render("home", context)	
}

#[get("/hello")]
fn hello() -> Json<&'static str> {
    Json("{
        'status': 'success',
        'message': 'Hello API!'}")
}

#[derive(FromForm, Debug)]
struct Book {
    title: String,
    author: String,
    isbn: String
}

#[post("/book", data = "<book_form>")]
fn new_book(book_form: Form<Book>) -> String {
    let book: Book = book_form.into_inner();
    let mut dummy_db: Vec<Book> = Vec::new();
    dummy_db.push(book);
    format!("Book added successfully: {:?}", dummy_db)
}


#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
		.mount("/", routes![index])
        .mount("/api", routes![hello])
		.attach(Template::fairing())
        .launch();
}
