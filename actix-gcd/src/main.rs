use actix_web::{App, get, web, HttpResponse, HttpServer, post, Responder};
use serde::Deserialize;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}


#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

#[get("/")]
async fn get_index() -> impl Responder {
    HttpResponse::Ok().body(
    r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
            <div>
            N: <input type="text" name="n"/>
            </div>
            <div>
            M: <input type="text" name="m"/>
            </div>
            <button>Compute GCD</button>
        </form>
    "#)    
}

#[post("/gcd")]
async fn post_gcd(form: web::Form<GcdParameters>) -> impl Responder {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }
    let response = format!("The GCD of {} and {} is <b>{}</b>\n", form.n, form.m, gcd(form.n, form.m));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()            
            .service(get_index)
            .service(post_gcd)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
