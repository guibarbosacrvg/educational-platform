use std::io::Write;

use actix_web::{web, HttpResponse};
use serde_derive::{Deserialize, Serialize};
use tempfile::{Builder, NamedTempFile};
use tokio::process::Command;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CompileRequest {
    code: String,
}

#[derive(Deserialize)]
pub struct Language {
    language: String,
}

#[derive(Serialize, Deserialize)]
pub struct CompileResponse {
    output_code: String,
    output_run: String,
}

enum LanguageExecution {
    Compile { compile_command: Vec<String> },
    Interpret { command: Vec<String> },
}

impl LanguageExecution {
    async fn execute(
        &self,
        code: &str,
        exec_name: &str,
        file_extension: &str,
    ) -> Result<String, String> {
        match self {
            LanguageExecution::Compile { compile_command } => {
                let source_file: NamedTempFile = file_handler(code, file_extension)?;
                let source_path: &str = source_file.path().to_str().ok_or("Invalid filename")?;
                let exec_path: String = format!("src/tmp/{}", exec_name); // Specify the directory and exec name here, might need to change this later

                println!("Source Path: {}", source_path); // Debug print, can be removed later
                println!("Exec Path: {}", exec_path); // Debug print, can be removed later

                let mut complete_compile_command = compile_command.clone();
                complete_compile_command.push(source_path.to_string()); // Add the source file path
                complete_compile_command.push("-o".to_string()); // Add the -o flag
                complete_compile_command.push(exec_path.clone()); // Add the path for the output executable

                let compile_args_str: Vec<&str> =
                    complete_compile_command.iter().map(AsRef::as_ref).collect();

                if std::path::Path::new(source_path).exists() {
                    println!("Confirmed: Source file exists at {}", source_path);
                } else {
                    println!("Warning: Source file does not exist at {}", source_path);
                }
                println!("Executing Compile Command: {:?}", complete_compile_command);
                execute_command(compile_args_str[0], &compile_args_str[1..]).await?;
                execute_command(&exec_path, &[]).await
            }
            LanguageExecution::Interpret { command } => {
                let file = file_handler(code, file_extension)?;
                let filename = file.path().to_str().ok_or("Invalid filename")?;

                let mut args = command.iter().map(String::as_str).collect::<Vec<&str>>();
                args.push(filename); // Add the filename as the last argument

                println!("Executing Interpret Command: {:?}", args); // Debug print
                execute_command(args[0], &args[1..]).await // Execute the command with arguments
            }
        }
    }
}

/* TODO: Maybe look into using a custom config file and load it using the config crate, use this to address the hardcoding issue */

fn create_http_response(success: bool, code: &str, message: String) -> HttpResponse {
    let response = CompileResponse {
        output_code: format!("Received code: {}", code),
        output_run: message,
    };

    if success {
        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::BadRequest().json(response)
    }
}

async fn execute_command(command: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(command)
        .args(args)
        .output()
        .await
        .map_err(|e| format!("Error executing command: {}", e))?;

    let stdout: String =
        String::from_utf8(output.stdout).map_err(|e| format!("Error parsing stdout: {}", e))?;

    let stderr: String =
        String::from_utf8(output.stderr).map_err(|e| format!("Error parsing stderr: {}", e))?;

    if !output.status.success() {
        return Err(format!("Error executing command: {}", stderr));
    }

    Ok(stdout)
}

fn file_handler(code: &str, extension: &str) -> Result<NamedTempFile, String> {
    let unique_id: String = Uuid::new_v4().to_string();
    let mut file: NamedTempFile = Builder::new()
        .prefix(unique_id.as_str())
        .suffix(&format!(".{}", extension))
        .rand_bytes(Default::default())
        .tempfile()
        .map_err(|e| format!("Error creating tempfile: {}", e))?;

    file.write_all(code.as_bytes())
        .map_err(|e| format!("Error writing to tempfile: {}", e))?;

    Ok(file)
}

fn clear_exec_file(exec_path: String) -> std::io::Result<()> {
    match std::fs::remove_file(&exec_path) {
        Ok(_) => {
            println!("Successfully deleted file: {}", exec_path);
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to delete file: {}. Error: {}", exec_path, e);
            Err(e)
        }
    }
}

async fn compile_c_tmpfile(req: web::Json<CompileRequest>) -> HttpResponse {
    let exec_name: String = Uuid::new_v4().to_string();
    let exec_path: String = format!("src/tmp/{}", exec_name);

    let c_execution: LanguageExecution = LanguageExecution::Compile {
        compile_command: vec!["gcc".to_string()],
    };

    let response: HttpResponse = match c_execution.execute(&req.code, &exec_name, "c").await {
        Ok(file_output) => create_http_response(true, &req.code, file_output),
        Err(error) => create_http_response(false, &req.code, error),
    };

    if let Err(e) = clear_exec_file(exec_path.clone()) {
        eprintln!("Error clearing executable: {}", e);
    }

    response
}

async fn interpret_python_tmpfile(req: web::Json<CompileRequest>) -> HttpResponse {
    let python_execution = LanguageExecution::Interpret {
        command: vec!["python3".to_string()],
    };

    match python_execution.execute(&req.code, "", "py").await {
        Ok(file_output) => create_http_response(true, &req.code, file_output),
        Err(error) => create_http_response(false, &req.code, error),
    }
}

async fn compile_go_tmpfile(req: web::Json<CompileRequest>) -> HttpResponse {
    let go_execution = LanguageExecution::Interpret {
        command: vec!["go".to_string(), "run".to_string()],
    };

    match go_execution.execute(&req.code, "", "go").await {
        Ok(file_output) => create_http_response(true, &req.code, file_output),
        Err(error) => create_http_response(false, &req.code, error),
    }
}

async fn compile_haskell_tmpfile(req: web::Json<CompileRequest>) -> HttpResponse {
    let exec_name = Uuid::new_v4().to_string();
    let exec_path = format!("src/tmp/{}", exec_name);

    let haskell_execution = LanguageExecution::Compile {
        compile_command: vec!["ghc".to_string()],
    };

    let response = match haskell_execution.execute(&req.code, &exec_name, "hs").await {
        Ok(file_output) => create_http_response(true, &req.code, file_output),
        Err(error) => create_http_response(false, &req.code, error),
    };

    if let Err(e) = clear_exec_file(exec_path.clone()) {
        eprintln!("Error clearing executable: {}", e);
    }
    response
}

async fn compile_rust_tmpfile(req: web::Json<CompileRequest>) -> HttpResponse {
    let exec_name = Uuid::new_v4().to_string();
    let exec_path = format!("src/tmp/{}", exec_name);

    let rust_execution = LanguageExecution::Compile {
        compile_command: vec!["rustc".to_string()],
    };

    let response = match rust_execution.execute(&req.code, &exec_name, "rs").await {
        Ok(file_output) => create_http_response(true, &req.code, file_output),
        Err(error) => create_http_response(false, &req.code, error),
    };

    if let Err(e) = clear_exec_file(exec_path.clone()) {
        eprintln!("Error clearing executable: {}", e);
    }
    response
}

async fn interpret_js_tmpfile(req: web::Json<CompileRequest>) -> HttpResponse {
    let js_execution = LanguageExecution::Interpret {
        command: vec!["node".to_string()],
    };

    match js_execution.execute(&req.code, "", "js").await {
        Ok(file_output) => create_http_response(true, &req.code, file_output),
        Err(error) => create_http_response(false, &req.code, error),
    }
}

pub async fn run_code(
    req: web::Json<CompileRequest>,
    language: web::Path<Language>,
) -> HttpResponse {
    println!("Received code: {}", req.code);
    match language.language.as_str() {
        "cpp" => compile_c_tmpfile(req).await,
        "python" => interpret_python_tmpfile(req).await,
        //        "java" => compile_java(req).await,
        "javascript" => interpret_js_tmpfile(req).await,
        "rust" => compile_rust_tmpfile(req).await,
        "go" => compile_go_tmpfile(req).await,
        "haskell" => compile_haskell_tmpfile(req).await,
        _ => HttpResponse::BadRequest().body("Language not supported"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_interpret_python_happy_path() {
        let mut app = test::init_service(
            App::new().route("/run/python", web::post().to(interpret_python_tmpfile)),
        )
        .await;
        let request = CompileRequest {
            code: "print('Hello, world!')".to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/run/python")
            .set_json(&request)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        println!("Response Status: {:?}", resp.status());
        println!("Response Headers: {:?}", resp.headers());
        assert!(
            resp.status().is_success(),
            "Response was not successful. Status: {:?}",
            resp.status()
        );

        let body = test::read_body(resp).await;
        println!("Response Body: {:?}", String::from_utf8_lossy(&body));
    }

    #[actix_rt::test]
    async fn test_interpret_python_sad_path() {
        let mut app = test::init_service(
            App::new().route("/run/python", web::post().to(interpret_python_tmpfile)),
        )
        .await;
        let request = CompileRequest {
            code: "print('Hello, world!'".to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/run/python")
            .set_json(&request)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        println!("Response Status: {:?}", resp.status());
        println!("Response Headers: {:?}", resp.headers());
        assert!(
            resp.status().is_client_error(),
            "Response was not a client error. Status: {:?}",
            resp.status()
        );

        let body = test::read_body(resp).await;
        println!("Response Body: {:?}", String::from_utf8_lossy(&body));
    }

    #[actix_rt::test]
    async fn test_compile_c_happy_path() {
        let mut app =
            test::init_service(App::new().route("/run/cpp", web::post().to(compile_c_tmpfile)))
                .await;
        let request = CompileRequest {
            code: "#include <stdio.h>\nint main() { printf(\"Hello, world!\"); return 0; }"
                .to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/run/cpp")
            .set_json(&request)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        println!("Response Status: {:?}", resp.status());
        println!("Response Headers: {:?}", resp.headers());
        assert!(
            resp.status().is_success(),
            "Response was not successful. Status: {:?}",
            resp.status()
        );

        let body = test::read_body(resp).await;
        println!("Response Body: {:?}", String::from_utf8_lossy(&body));
    }

    #[actix_rt::test]
    async fn test_compile_c_sad_path() {
        let mut app =
            test::init_service(App::new().route("/run/cpp", web::post().to(compile_c_tmpfile)))
                .await;
        let request = CompileRequest {
            code: "#include <stdio.h>\nint main() { printf(\"Hello, world!\"); return 0;"
                .to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/run/cpp")
            .set_json(&request)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        println!("Response Status: {:?}", resp.status());
        println!("Response Headers: {:?}", resp.headers());
        assert!(
            resp.status().is_client_error(),
            "Response was not a client error. Status: {:?}",
            resp.status()
        );

        let body = test::read_body(resp).await;
        println!("Response Body: {:?}", String::from_utf8_lossy(&body));
    }

    #[actix_rt::test]
    async fn test_interpret_js_happy_path() {
        let mut app = test::init_service(
            App::new().route("/run/javascript", web::post().to(interpret_js_tmpfile)),
        )
        .await;
        let request = CompileRequest {
            code: "console.log('Hello, world!')".to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/run/javascript")
            .set_json(&request)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        println!("Response Status: {:?}", resp.status());
        println!("Response Headers: {:?}", resp.headers());
        assert!(
            resp.status().is_success(),
            "Response was not successful. Status: {:?}",
            resp.status()
        );

        let body = test::read_body(resp).await;
        println!("Response Body: {:?}", String::from_utf8_lossy(&body));
    }

    #[actix_rt::test]
    async fn test_interpret_js_sad_path() {
        let mut app = test::init_service(
            App::new().route("/run/javascript", web::post().to(interpret_js_tmpfile)),
        )
        .await;
        let request = CompileRequest {
            code: "console.log('Hello, world!'".to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/run/javascript")
            .set_json(&request)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        println!("Response Status: {:?}", resp.status());
        println!("Response Headers: {:?}", resp.headers());
        assert!(
            resp.status().is_client_error(),
            "Response was not a client error. Status: {:?}",
            resp.status()
        );

        let body = test::read_body(resp).await;
        println!("Response Body: {:?}", String::from_utf8_lossy(&body));
    }

    #[actix_rt::test]
    async fn test_compile_rust_happy_path() {
        let mut app =
            test::init_service(App::new().route("/run/rust", web::post().to(compile_rust_tmpfile)))
                .await;
        let request = CompileRequest {
            code: "fn main() { println!(\"Hello, world!\"); }".to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/run/rust")
            .set_json(&request)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        println!("Response Status: {:?}", resp.status());
        println!("Response Headers: {:?}", resp.headers());
        assert!(
            resp.status().is_success(),
            "Response was not successful. Status: {:?}",
            resp.status()
        );

        let body = test::read_body(resp).await;
        println!("Response Body: {:?}", String::from_utf8_lossy(&body));
    }

    #[actix_rt::test]
    async fn test_compile_rust_sad_path() {
        let mut app =
            test::init_service(App::new().route("/run/rust", web::post().to(compile_rust_tmpfile)))
                .await;
        let request = CompileRequest {
            code: "fn main() { println!(\"Hello, world!\") }".to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/run/rust")
            .set_json(&request)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        println!("Response Status: {:?}", resp.status());
        println!("Response Headers: {:?}", resp.headers());
        assert!(
            resp.status().is_client_error(),
            "Response was not a client error. Status: {:?}",
            resp.status()
        );

        let body = test::read_body(resp).await;
        println!("Response Body: {:?}", String::from_utf8_lossy(&body));
    }

    #[actix_rt::test]
    async fn test_compile_go_happy_path() {
        let mut app =
            test::init_service(App::new().route("/run/go", web::post().to(compile_go_tmpfile)))
                .await;
        let request = CompileRequest {
            code: "package main\nimport \"fmt\"\nfunc main() { fmt.Println(\"Hello, world!\") }"
                .to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/run/go")
            .set_json(&request)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        println!("Response Status: {:?}", resp.status());
        println!("Response Headers: {:?}", resp.headers());
        assert!(
            resp.status().is_success(),
            "Response was not successful. Status: {:?}",
            resp.status()
        );

        let body = test::read_body(resp).await;
        println!("Response Body: {:?}", String::from_utf8_lossy(&body));
    }

    #[actix_rt::test]
    async fn test_compile_go_sad_path() {
        let mut app =
            test::init_service(App::new().route("/run/go", web::post().to(compile_go_tmpfile)))
                .await;
        let request = CompileRequest {
            code: "package main\nimport \"fmt\"\nfunc main() { fmt.Println(\"Hello, world!\")"
                .to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/run/go")
            .set_json(&request)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        println!("Response Status: {:?}", resp.status());
        println!("Response Headers: {:?}", resp.headers());
        assert!(
            resp.status().is_client_error(),
            "Response was not a client error. Status: {:?}",
            resp.status()
        );

        let body = test::read_body(resp).await;
        println!("Response Body: {:?}", String::from_utf8_lossy(&body));
    }
}
