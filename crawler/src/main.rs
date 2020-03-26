extern crate url;
// #[cfg(feature = "http")]
// extern crate reqwest;

extern crate percent_encoding;

mod page;
mod website;
mod configuration;
mod robots;

use website::Website;


fn main() {
    let mut site = Website::new("https://www.codercto.com");
    site.configuration.verbose = true;
    site.crawl();
        // site.get_pages()
    

}