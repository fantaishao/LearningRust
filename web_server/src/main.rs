use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use web_server::ThreadPool;
use web_server::api;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to address");
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        // for stream in listener.incoming().take(2){
            match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("Failed to establish a connection: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();

    let request_line = &http_request[0];

    // extract the path and query parameters
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        eprintln!("Invalid request line: {}", request_line);
        return;
    }

    let full_path = parts[1];
    let path_parts: Vec<&str> = full_path.split('?').collect();
    let path = path_parts[0];
    let query = if path_parts.len() > 1 {
        Some(path_parts[1])
    } else {
        None
    };



    let (status_line, filename) = match path {
        "/" => ("HTTP/1.1 200 OK", "index.html"),
        "/api/treedata" => {
            // API endpoint for tree data
            let tree_data = api::get_tree_data();
            return send_json_response(stream, tree_data);
        }
        "/api/getNode" => {
            // extract node ID from query parameters
            if let Some(query_str) = query {
                let params: Vec<&str> = query_str.split('&').collect();
                let mut node_id = None;

                for param in params {
                    let kv: Vec<&str> = param.split('=').collect();
                    if kv.len() == 2 && kv[0] == "id" {
                        node_id = Some(kv[1]);
                        break;
                    }
                }

                if let Some(id) = node_id {
                    let node_data = api::get_node(id);
                    return send_json_response(stream, node_data);
                } else {
                    let error_response = r#"{"error": "Missing node ID parameter"}"#.to_string();
                    return send_json_response(stream, error_response);
                }
            } else {
                let error_response = r#"{"error": "Missing node ID parameter"}"#.to_string();
                return send_json_response(stream, error_response);
            }
        }
        "/api/deleteNode" => {
            if let Some(query_str) = query {
                let params: Vec<&str> = query_str.split('&').collect();
                let mut node_id = None;

                for param in params {
                    let kv: Vec<&str> = param.split('=').collect();
                    if kv.len() == 2 && kv[0] == "id" {
                        node_id = Some(kv[1]);
                        break;
                    }
                }

                if let Some(id) = node_id {
                    let node_data = api::delete_node(id);
                    return send_json_response(stream, node_data);
                } else {
                    let error_response = r#"{"error": "Missing node ID parameter"}"#.to_string();
                    return send_json_response(stream, error_response);
                }
            } else {
                let error_response = r#"{"error": "Missing node ID parameter"}"#.to_string();
                return send_json_response(stream, error_response);
            }
        }
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => {
            eprintln!("Failed to read the file: {}", filename);
            return;
        }
        
    };
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}


fn send_json_response(mut stream: TcpStream, json_data: String) {
    let status_line = "HTTP/1.1 200 OK";
    let length = json_data.len();
    let response = format!(
        "{status_line}\r\nContent-Length:{length}\r\nContent-Type:application/json\r\nAccess-Control-Allow-Origin:*\r\n\r\n{json_data}"
    );

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to send response: {}", e);
    }
}