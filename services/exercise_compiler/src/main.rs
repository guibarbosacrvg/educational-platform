use std::fs::File;
use std::io::Write;

use tokio::process::Command;
use actix_web::{web, http, App, HttpResponse, HttpServer};
use actix_cors::Cors;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CompileRequest {
    code: String,
}

#[derive(Deserialize)]
struct Language {
    language: String,
}

#[derive(Serialize)]
struct CompileResponse {
    output_code: String,
    output_run: String,
}

async fn compile_c(req: web::Json<CompileRequest>) -> HttpResponse {
    println!("Received code: {}", req.code);
    let filename: &str = "src/tmp/tmp.cpp"; 
    let mut file: File = File::create(filename).expect("Could not create the file");
    file.write_all(req.code.as_bytes()).expect("Could not  write to file.");
    let output: std::process::Output = Command::new("g++")
        .arg(filename)
        .arg("-o")
        .arg("src/tmp/tmp")
        .output()
        .await
        .expect("Error compiling");

    if !output.status.success() {
        let error_message: String = format!("g++ failed: {}", String::from_utf8_lossy(&output.stderr));
        return HttpResponse::BadRequest().json(CompileResponse {                
            output_code: req.code.clone(),
            output_run: error_message,
        });
    }

    let file_output: String = match Command::new("./src/tmp/tmp").output().await {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(e) => format!("Failed to execute compiled program: {}", e),
    };

    HttpResponse::Ok().json(CompileResponse {
        output_code: format!("Received code: {}", req.code),
        output_run: format!("{}", file_output),
    })
}


async fn interpret_python(req: web::Json<CompileRequest>) -> HttpResponse {
    println!("Received Python code: {}", req.code);
    let filename: &str = "src/tmp/tmp.py";
    let mut file: File = File::create(filename).expect("Could not create the file");
    file.write_all(req.code.as_bytes()).expect("Could not  write to file.");

    let file_output = match Command::new("python3")
    .arg(filename)
    .output()
    .await {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(e) => format!("Failed to execute python program: {}", e),
    };


    HttpResponse::Ok().json(CompileResponse {
        output_code: format!("Received python code: {}", req.code),
        output_run: format!("{}", file_output),
    })
}


async fn compile_java(req: web::Json<CompileRequest>) -> HttpResponse {
    println!("Received Java code: {}", req.code);
    let filename: &str = "src/tmp/Tmp.java";
    let mut file: File = File::create(filename).expect("Could not create the file");
    file.write_all(req.code.as_bytes()).expect("Could not  write to file.");

    let output: std::process::Output = Command::new("javac")
        .arg(filename)
        .output()
        .await
        .expect("Error compiling");

    if !output.status.success() {
        let error_message: String = format!("javac failed: {}", String::from_utf8_lossy(&output.stderr));
        return HttpResponse::BadRequest().json(CompileResponse {                
            output_code: req.code.clone(),
            output_run: error_message,
        });
    }

    let output: std::process::Output = Command::new("java")
        .arg("-cp")
        .arg("src/tmp")
        .arg("Tmp")
        .output()
        .await
        .expect("Error running");

    let file_output = if output.status.success() {
        format!(
            "{}",
            String::from_utf8_lossy(&output.stdout),
        )
    } else {
        format!("Failed to execute compiled program: {}", String::from_utf8_lossy(&output.stderr))
    };
    
    HttpResponse::Ok().json(CompileResponse {
        output_code: format!("Received code: {}", req.code),
        output_run: format!("{}", file_output),
    })
}

async fn interpret_js(req: web::Json<CompileRequest>) -> HttpResponse {
    println!("Received JS code: {}", req.code);
    let filename: &str = "src/tmp/tmp.js";
    let mut file: File = File::create(filename).expect("Could not create the file");
    println!("Created file");
    file.write_all(req.code.as_bytes()).expect("Could not  write to file.");
    println!("Wrote to file");
    
    let output = Command::new("node")
    .arg(filename)
    .output()
    .await;

    let file_output = match output {
    Ok(output) => {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        
        if !stderr.is_empty() {
            format!("Error: {}", stderr)
        } else {
            stdout
        }
    },
    Err(e) => format!("Failed to execute node program: {}", e),
};


    HttpResponse::Ok().json(CompileResponse {
        output_code: format!("Received JS code: {}", req.code),
        output_run: format!("{}", file_output),
    })
}

async fn compile_rust(req: web::Json<CompileRequest>) -> HttpResponse {
    println!("Received Rust code: {}", req.code);
    let filename: &str = "src/tmp/tmp.rs";
    let mut file: File = File::create(filename).expect("Could not create the file");
    file.write_all(req.code.as_bytes()).expect("Could not  write to file.");

    let output: std::process::Output = Command::new("rustc")
        .arg("-o")
        .arg("src/tmp/tmp")
        .arg(filename)
        .output()
        .await
        .expect("Error compiling");

    if !output.status.success() {
        let error_message: String = format!("rustc failed: {}", String::from_utf8_lossy(&output.stderr));
        return HttpResponse::BadRequest().json(CompileResponse {                
            output_code: req.code.clone(),
            output_run: error_message,
        });
    }

    let output: std::process::Output = Command::new("./src/tmp/tmp").output().await.expect("Error running");

    let file_output = if output.status.success() {
        format!(
            "{}",
            String::from_utf8_lossy(&output.stdout),
        )
    } else {
        format!("Failed to execute compiled program: {}", String::from_utf8_lossy(&output.stderr))
    };
    
    HttpResponse::Ok().json(CompileResponse {
        output_code: format!("Received code: {}", req.code),
        output_run: format!("{}", file_output),
    })
}

async fn compile_go(req: web::Json<CompileRequest>) -> HttpResponse {
    println!("Received Go code: {}", req.code);
    let filename: &str = "src/tmp/tmp.go";
    let mut file: File = File::create(filename).expect("Could not create the file");
    file.write_all(req.code.as_bytes()).expect("Could not  write to file.");

    let output: std::process::Output = Command::new("go")
        .arg("build")
        .arg("-o")
        .arg("src/tmp/tmp")
        .arg(filename)
        .output()
        .await
        .expect("Error compiling");

    if !output.status.success() {
        let error_message: String = format!("go failed: {}", String::from_utf8_lossy(&output.stderr));
        return HttpResponse::BadRequest().json(CompileResponse {                
            output_code: req.code.clone(),
            output_run: error_message,
        });
    }

    let output: std::process::Output = Command::new("./src/tmp/tmp").output().await.expect("Error running");

    let file_output = if output.status.success() {
        format!(
            "{}",
            String::from_utf8_lossy(&output.stdout),
        )
    } else {
        format!("Failed to execute compiled program: {}", String::from_utf8_lossy(&output.stderr))
    };
    
    HttpResponse::Ok().json(CompileResponse {
        output_code: format!("Received code: {}", req.code),
        output_run: format!("{}", file_output),
    })
}

async fn compile_haskell(req: web::Json<CompileRequest>) -> HttpResponse {
    println!("Received Haskell code: {}", req.code);
    let filename: &str = "src/tmp/tmp.hs";
    let mut file: File = File::create(filename).expect("Could not create the file");
    file.write_all(req.code.as_bytes()).expect("Could not  write to file.");

    let output: std::process::Output = Command::new("ghc")
        .arg("-o")
        .arg("src/tmp/tmp")
        .arg(filename)
        .output()
        .await
        .expect("Error compiling");

    if !output.status.success() {
        let error_message: String = format!("ghc failed: {}", String::from_utf8_lossy(&output.stderr));
        return HttpResponse::BadRequest().json(CompileResponse {                
            output_code: req.code.clone(),
            output_run: error_message,
        });
    }

    let output: std::process::Output = Command::new("./src/tmp/tmp").output().await.expect("Error running");

    let file_output = if output.status.success() {
        format!(
            "{}",
            String::from_utf8_lossy(&output.stdout),
        )
    } else {
        format!("Failed to execute compiled program: {}", String::from_utf8_lossy(&output.stderr))
    };
    
    HttpResponse::Ok().json(CompileResponse {
        output_code: format!("Received code: {}", req.code),
        output_run: format!("{}", file_output),
    })
}

async fn run_code(req: web::Json<CompileRequest>, language: web::Path<Language>) -> HttpResponse {
    println!("Received code: {}", req.code);
    match language.language.as_str() {
        "cpp" => compile_c(req).await,
        "python" => interpret_python(req).await,
        "java" => compile_java(req).await,
        "javascript" => interpret_js(req).await,
        "rust" => compile_rust(req).await,
        "go" => compile_go(req).await,
        "haskell" => compile_haskell(req).await,
        _ => HttpResponse::BadRequest().body("Language not supported"),
    }
}

async fn greet() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000") // Permitir origem do frontend
            .allowed_methods(vec!["GET", "POST"]) // Métodos permitidos
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        
        App::new()
            .wrap(cors)
            .route("/", web::get().to(greet))
            .service(web::resource("/run/{language}").route(web::post().to(run_code)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
