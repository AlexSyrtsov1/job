use std::time;
use std::collections::HashMap;

use actix_web::web::Query;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::cookie::Key;
use actix_session::SessionMiddleware;
use actix_session::storage::SessionStore;
use serde::Deserialize;
use uuid::Uuid;
use serde_json::{Value, Error};

#[derive(Deserialize)]
pub struct InputData
{
    name: String,
    email: String,
}


pub async fn index(name: web::Path<String>, search_request: web::Json<HashMap<String, serde_json::Value>>) -> impl Responder
{
    let file_path = format!("C:/Users/syrtsov_ayu/projects/site/server/appearence/{}/{}.html", &name, &name);
    let body = std::fs::read_to_string(file_path).unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

pub async fn main_page() -> impl Responder
{
    let body = std::fs::read_to_string("C:/Users/syrtsov_ayu/projects/site/server/appearence/main/main.html").unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

pub async fn styles(name: web::Path<String>) -> impl Responder
{
    let file_path = format!("C:/Users/syrtsov_ayu/projects/site/server/appearence/{}/content.css", &name);
    let body = std::fs::read_to_string(file_path).unwrap();
    HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .body(body)
}

pub async fn scripts(name: web::Path<String>) -> impl Responder
{
    let file_path = format!("C:/Users/syrtsov_ayu/projects/site/server/appearence/{}/{}.js", &name, &name);
    let body = std::fs::read_to_string(file_path).unwrap();
    HttpResponse::Ok()
        .content_type("application/javascript; charset=utf-8")
        .body(body)
}

pub async fn png(file: web::Path<String>) -> impl Responder
{
    let image = format!("C:/Users/syrtsov_ayu/projects/site/server/appearence/images/{}.png", &file);
    let body = std::fs::read(image).unwrap();
    HttpResponse::Ok()
    .content_type("image/png")
    .body(body)
}

pub async fn svg(file: web::Path<String>) -> impl Responder
{
    let image = format!("C:/Users/syrtsov_ayu/projects/site/server/appearence/images/{}.svg", &file);
    let body = std::fs::read(image).unwrap();
    HttpResponse::Ok()
    .content_type("image/svg+xml")
    .body(body)
}

pub async fn fonts(file: web::Path<String>) -> impl Responder
{
    let image = format!("C:/Users/syrtsov_ayu/projects/site/server/appearence/fonts/{}.ttf", &file);
    let body = std::fs::read(image).unwrap();
    HttpResponse::Ok()
    .content_type("font/ttf")
    .body(body)
}

pub async fn find(search_request: web::Json<HashMap<String, serde_json::Value>>) -> impl Responder
{
    HttpResponse::Ok()
    .content_type("text/html, charset=utf-8")
    .body(format!(r#"<div class="card-container">
                    <div class="card-content">
                        <h1 style="font-size: 1.1rem; margin: 0 0 .5rem 0;">
                            index
                        </h1>
                        <h2 id="tech-name" style="font-size: 1.3rem; margin: 0 0 1rem 0;">
                            <a class="tech-name-title" href="">
                                {:?}
                            </a>
                        </h2>
                        <p style="font-size: 1.1rem; margin: 0 0 .3rem 0;">
                            автор 1/автор 2/.../автор 3
                        </p>
                        <p style="font-size: .9rem; margin: 0 0 1rem 0; color: #5f5f5f;">
                            2024
                        </p>
                        <div class="category-tags">
                            <a href="" class="category-tag">Аграрно-технологический институт</a>
                            <a href="" class="category-tag">Агрокомплекс</a>
                        </div>
                    </div>
                </div>"#, search_request.into_inner())) // q.into_inner()["y"].as_array().unwrap()[0].to_string()
}

pub async fn update(name: web::Path<String>) -> impl Responder
{
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Actix Web Example</title>
            <script>
                pub async function updateElement() {
                    const response = await fetch('/main/update');
                    const data = await response.text();
                    document.getElementById('myElement').innerText = data;
                }
            </script>
        </head>
        <body>
            <h1 id="myElement">Original Text</h1>
            <button onclick="updateElement()">Update Text</button>

            <h1>AJAX POST Request Example</h1>
            <form id="dataForm">
                <label for="name">Name:</label>
                <input type="text" id="name" name="name" required>
                <br>
                <label for="email">Email:</label>
                <input type="email" id="email" name="email" required>
                <br>
                <button type="submit">Submit</button>
            </form>
            <div id="response"></div>

            <script>
                document.getElementById('dataForm').addEventListener('submit', function(event) {
                    event.preventDefault(); // Prevent the form from submitting the traditional way

                    const name = document.getElementById('name').value;
                    const email = document.getElementById('email').value;

                    // Create a data object to send
                    const data = {
                        name: name,
                        email: email
                    };

                    // Make the AJAX POST request
                    fetch('http://127.0.0.1:8081/main/result', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify(data)
                })
                .then(response => {
                    if (!response.ok) {
                        throw new Error('Network response was not ok ' + response.statusText);
                    }
                    // Wait for the response text to resolve
                    return response.text(); // Return the Promise from response.text()
                })
                .then(text => {
                    // Now we can safely set the inner text
                    document.getElementById('response').innerHTML = text;
                })
                .catch(error => {
                    console.error('There was a problem with the fetch operation:', error);
                    document.getElementById('response').innerText = 'Error: ' + error.message;
                });

                });
            </script>
        </body>
        </html>
    "#)
}

pub async fn poster(name: web::Path<String>, params: web::Json<InputData>) -> impl Responder
{
    let res = format!(
        r#"<div id="shit">our input is name: {}, email: {}, and I have got it in {:?}</div>
        <img src="images/i.png"/>"#,
        params.name,
        params.email,
        time::Instant::now()
    );

    HttpResponse::Ok()
        .content_type("text/html") // Correcting "text/palin" to "text/plain"
        .body(res)
}

pub async fn not_found() -> impl Responder
{
    HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body("<h1>Error 404</h1>")
}