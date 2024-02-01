use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;


#[derive(Deserialize)]
struct HCFparams{
    m:u64,
    n:u64
}

#[actix_web::main]

async fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/hcf", web::post().to(post_gcd))
    });

    println!("Serving on http://localhost:3000...");
    let _ = server
        .bind("127.0.0.1:3000").expect("error binding server to address")
        .run().await;
}

async fn get_index() -> HttpResponse {

    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/hcf" method="post">
                <input type="text" name="m"/>
                <input type="text" name="n"/>
                <button type="submit">Compute HCF</button>
                </form>
            "#,
        )
}

async fn post_gcd(form:web::Form<HCFparams>)-> HttpResponse{
    if form.n ==0 || form.m==0{
        return HttpResponse::BadRequest()
        .content_type("text/html")
        .body("Donot compute HCF with a '0'");
    }

    let response = format!("The greatest common divisor of the numbers {} and {}  is <b> {}</b> \n",form.m, form.n, hcf(form.m,form.n));

    HttpResponse::Ok().content_type("text/html").body(response)
}

fn hcf(mut m:u64,mut n:u64)->u64{
    assert!(m!=0 && n!=0);
    while m!=0 {
        if m<n{
            let t=m;
            m=n;
            n=t;
        }
        m=m%n;
    }
    n

}