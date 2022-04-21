mod response;
mod server;
use response::Header;
use server::HttpServer;
use std::panic::{self, PanicInfo};
use std::{env, fs};
use yaml_rust::YamlLoader;

const BANNER: &'static str = r#"
    ____             __     _       __     __   _____
   / __ \__  _______/ /_   | |     / /__  / /_ / ___/___  ______   _____  _____
  / /_/ / / / / ___/ __/   | | /| / / _ \/ __ \\__ \/ _ \/ ___/ | / / _ \/ ___/
 / _, _/ /_/ (__  ) /_     | |/ |/ /  __/ /_/ /__/ /  __/ /   | |/ /  __/ /
/_/ |_|\__,_/____/\__/     |__/|__/\___/_.___/____/\___/_/    |___/\___/_/
"#;

fn panic_hook(info: &PanicInfo<'_>) {
    println!("{}", info);
    let msg = match info.payload().downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match info.payload().downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "Box<Any>",
        },
    };

    eprintln!("An error occured: {}", msg);
}

fn main() {
    println!("{}\n", BANNER);
    panic::set_hook(Box::new(|info| {
        panic_hook(info);
    }));

    let mut headers: Vec<Header> = vec![("X-Server".to_string(), "Rust WebServer".to_string())];

    let args: Vec<String> = env::args().collect();
    let config_path = args.get(1).expect("Path to config file is not set");
    let config_doc = YamlLoader::load_from_str(&fs::read_to_string(config_path).unwrap()).unwrap();

    let config = &config_doc[0]["server"];
    let port = config["port"]
        .as_i64()
        .expect("`port` should be an integer") as u16;
    let prefix = config["prefix"]
        .as_str()
        .expect("`prefix` must be a string")
        .to_string();
    let path = config["path"]
        .as_str()
        .expect("`path` must be a string")
        .to_string();

    let headers_vec = config["headers"]
        .as_vec()
        .expect("`headers` should be an array")
        .to_vec();

    let mut header_index = 1;
    for h in headers_vec {
        let h = h
            .as_hash()
            .expect(&format!(
                "Header should be a key-value pair (Header index: {})",
                header_index
            ))
            .front()
            .unwrap();

        headers.push((
            h.0.as_str()
                .expect(&format!(
                    "Header name should be a string (Header index: {})",
                    header_index
                ))
                .to_string(),
            h.1.as_str()
                .expect(&format!(
                    "Header value should be a string (Header index: {})",
                    header_index
                ))
                .to_string(),
        ));

        header_index += 1;
    }

    HttpServer::new(prefix, path, headers).listen(port);
}
