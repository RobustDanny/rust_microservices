use warp::Filter;
use warp::Reply;
use warp::reply;
use warp::http::{Response, Uri, StatusCode};
use test_db::{connect_to_db, DataBaseFunctions};

#[tokio::main]
async fn main() {
    let _db = connect_to_db().expect("Failed to connect to database");

    let main_page = warp::get()
    .and(warp::path::end())
    .map(||{
        let html = include_str!("pages/main_page.html");

        Response::builder()
        .header("Content-Type", "text/html")
                .body(html)
                .unwrap()
    });

    let login_page = warp::get()
        .and(warp::path("sign_in"))
        .map(|| {
            let html = include_str!("pages/sign_in.html");
            Response::builder()
                .header("Content-Type", "text/html")
                .body(html)
                .unwrap()
        });

    let submit_sign_up = warp::post()
        .and(warp::path("sign_up"))
        .and(warp::body::form())
        .map(|user: test_db::UserSignUp| {
            // println!("Form submitted for user: {}", user.email_sign_up);
            
            let db = connect_to_db().expect("Failed to connect to database");
            
            match db.add_user(user) {
                Ok(_) => Box::new(warp::redirect(Uri::from_static("/sign_in"))) as Box<dyn Reply>,
                Err(e) => {
                    eprintln!("Error adding user: {}", e);
                    Box::new(reply::with_status(
                        format!("Failed to add user: {}", e),
                        StatusCode::INTERNAL_SERVER_ERROR,
                    )) as Box<dyn Reply>
                }
            }
        });
    
    let submit_sign_in = warp::post()
        .and(warp::path("sign_in"))
        .and(warp::body::form())
        .map(|user: test_db::UserSignIn | {

            // println!("Form submitted for user: {}", user.email_sign_in);
            
            let db = connect_to_db().expect("Failed to connect to database");
            
            match db.check_user_sign_in(user) {
                Ok(true) => Box::new(warp::redirect(Uri::from_static("/"))) as Box<dyn Reply>,
                Ok(false) => Box::new(warp::redirect(Uri::from_static("/sign_in"))) as Box<dyn Reply>,
                Err(e) => {
                    eprintln!("Error adding user: {}", e);
                    Box::new(reply::with_status(
                        format!("Failed to add user: {}", e),
                        StatusCode::INTERNAL_SERVER_ERROR,
                    )) as Box<dyn Reply>
                }
            }
        });

    let forgot_password = warp::get()
    .and(warp::path("forgot_password"))
    .map(|| {
        let html = include_str!("pages/forgot_password.html");
        Response::builder()
            .header("Content-Type", "text/html")
            .body(html)
            .unwrap()
        
    });

    let submit_forgot_password = warp::post()
    .and(warp::path("forgot_password"))
    .and(warp::body::form())
    .map(|user: test_db::UserCheck| {
        
        let db = connect_to_db().expect("Failed to connect to database");
        
        // println!("Received user data: {:?}", &user); 

        match db.check_user_forgot_password(user) {
            Ok(true) => {
                println!("✓ User exists - sending password reset email");
                warp::reply::html("<html><body>User exists - Reset link sent!</body></html>")
            },
            Ok(false) => {
                println!("✗ User does not exist");
                warp::reply::html("<html><body>Email not found</body></html>")
            },
            Err(e) => {
                println!("Database error: {:?}", e);
                warp::reply::html("<html><body>Error checking user</body></html>")
            },
        }        
    });

    let routes = main_page
    .or(forgot_password)
    .or(submit_forgot_password)
    .or(login_page)
    .or(submit_sign_up)
    .or(submit_sign_in);
    
    println!("http://localhost:3030/sign_in");
    println!("http://localhost:3030/forgot_password");
    println!("http://localhost:3030");

    warp::serve(routes)
        .run(([127,0,0,1], 3030))
        .await;
}


