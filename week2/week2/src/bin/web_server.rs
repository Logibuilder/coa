use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, Write, Stdout, BufReader, stdout};
use std::fs; 
use std::path::Path;


fn get_content_type(extention: &str) -> &str {
    match extention {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "jpg" => "image/jpeg",
        "png" => "image/png",
        "svg" => "image/svg+xml",
        "woff" => "font/woff",
        "woff2" => "font/woff2", 
        _ => "application/octet-stream", 
    }
}
/**
pour une requete GET, on doit retourner le content-type en fonction de l'extension du fichier demandé
*/
fn get_response_data(content_type : &str) {

} 

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(&stream);

    let mut first_line_request = String::new();

    reader.read_line(&mut first_line_request)?;

    if !first_line_request.is_empty() {
        println!("REçu : {}", first_line_request);
    }

    let parts: Vec<&str> = first_line_request.split_whitespace().collect();

    if parts.len() < 3 {

        stream.write_all(b"HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\nContent-Length: 11\r\n\r\nBad Request")?;
    }

    if parts[0] != "GET" {
        stream.write_all(b"HTTP/1.1 405 Method Not Allowed\r\nContent-Type: text/plain\r\nContent-Length: 18\r\n\r\nMethod Not Allowed")?;
    } 

    let path_request =  &parts[1]; 

    if *path_request == "/" {

        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 11\r\n\r\nHello world";
        stream.write_all(response.as_bytes())?;
    } else {

        let file_name = &path_request[1..];
        let path = Path::new(file_name);
        if path.exists() && path.is_file() {
            let file_content = fs::read(file_name)?;


            let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
            let content_type = get_content_type(extension);

            let header = format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n", content_type, file_content.len());

            stream.write_all(header.as_bytes())?;
            stream.write_all(&file_content)?;
        } else {
            let body = "404 Not Found";
            let response = format!("HTTP/1.1 {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", body, body.len(), body);
            stream.write_all(response.as_bytes())?;
        }

    }
    stream.flush()?;
    Ok(())
}



fn main() -> std::io::Result<()>  {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("Server started on 127.0.0.1:8080");

    // accept connections and process them serially
    for stream in listener.incoming() {
        println!("Connection established!");
        handle_client(stream?)?;
    }
    Ok(())
}