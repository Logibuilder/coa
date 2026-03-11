use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, Write, Stdout, BufReader, stdout};



const Acceuil : &str = r#"<!DOCTYPE html>

<html lang="fr">
<head>
<meta charset="UTF-8">
<title>Mini serveur test</title>

<link rel="stylesheet" href="style.css">

<style>
body {
    font-family: Arial, sans-serif;
    text-align: center;
    margin-top: 60px;
}

h1 {
    margin-bottom: 40px;
}

.container {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 15px;
}

button {
    padding: 12px 20px;
    font-size: 16px;
    cursor: pointer;
}
</style>

</head>
<body>

<h1>Test des ressources du mini serveur</h1>

<div class="container">

<a href="page.html"><button>HTML</button></a>

<a href="style.css"><button>CSS</button></a>

<a href="script.js"><button>JavaScript</button></a>

<a href="image.jpg"><button>JPG</button></a>

<a href="image.png"><button>PNG</button></a>

<a href="image.svg"><button>SVG</button></a>

<a href="font.woff"><button>WOFF</button></a>

<a href="font.woff2"><button>WOFF2</button></a>

</div>


</body>
</html>
"#;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(&stream);

    let mut first_line_request = String::new();

    reader.read_line(&mut first_line_request)?;

    if !first_line_request.is_empty() {
        println!("REçu : {}", first_line_request);
        stdout().flush()?;
    }
    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 12\r\n\r\nHello world";

    stream.write_all(response.as_bytes())?;

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