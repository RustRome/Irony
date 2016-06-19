extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate rand;
extern crate liquid;

use iron::prelude::*;
use iron::status;
use staticfile::Static;
use router::Router;
use mount::Mount;
use rand::distributions::{IndependentSample, Range};

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;


use liquid::{Renderable, Context, Value, Template};


fn fortune() -> String {
	let between = Range::new(0, 1_000);
	let mut rng = rand::thread_rng();
	let mut res = between.ind_sample(&mut rng);


	loop {
		let f = File::open("/usr/share/games/fortunes/fortunes").unwrap();
		let reader = BufReader::new(f);
		let mut string_res = String::new();
		for line in reader.lines() {
			let line = line.unwrap();
			if line == "%" {
				if res == 0 {
					return string_res;
				} else {
					res -= 1;
				}
			}
			else if res == 0 {
				string_res.push_str(&line);
				string_res.push_str("</br>");
			}
		}
	}
}


fn fortune_handler(req: &mut Request,template:liquid::Template) -> IronResult<Response> {
	let mut context = Context::new();
	context.set_val("content",liquid::Value::Str( fortune()));
	let output = template.render(&mut context).unwrap().unwrap();
	let ct = iron::modifiers::Header(iron::headers::ContentType::html());
	Ok(Response::with((status::Ok,output,ct)))
}

struct IndexPage {
	pub template: Template,
}

impl IndexPage {
	fn new() -> IndexPage {
		let mut input = String::new();
		File::open("./index.liquid").unwrap().read_to_string(&mut input).unwrap();
		let options:liquid::LiquidOptions = Default::default();
		let template = liquid::parse(&input, options).unwrap();
		IndexPage{template: template }
	}
}

fn main() {
	let mut router = Router::new();
	router.get("/",|req: & mut Request| -> IronResult<Response> {
		let index_page = IndexPage::new();
		fortune_handler(req,index_page.template)
	});
	let mut mount = Mount::new();
	mount.mount("/style.css",Static::new(Path::new("./style.css")));
	mount.mount("/background.png", Static::new(Path::new("./background.png")));
	mount.mount("/",router);
	let server = Iron::new(mount);
	server.http("localhost:3000").unwrap();
}
