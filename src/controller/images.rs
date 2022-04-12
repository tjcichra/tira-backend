use crate::service;
use rocket::data::Data;
use rocket::data::ToByteUnit;
use rocket::response::stream::ByteStream;

//TODO: Remove 512KB limit
#[post("/images/<file_name>", data = "<data>")]
pub async fn upload_image_endpoint(
    // conn: TiraDbConn,
    // cookies: &CookieJar<'_>,
    // upload_image_json: Json<UploadImage>,
    // file_name: String,
    file_name: &str,
    data: Data<'_>,
) -> std::io::Result<()> {
    let tim = data.open(8.megabytes()).into_bytes().await?;
        // .stream_to(tokio::io::stdout())
    
    if !tim.is_complete() {
        println!("there are bytes remaining in the stream");
    }

    let j = tim.into_inner();
    // controller::authentication(&conn, cookies).await.unwrap();
    service::images::upload_image(file_name, j).await;
    Ok(())
}

#[get("/images/<file_name>")]
pub async fn retrieve_image_endpoint(file_name: &str) -> ByteStream![Vec<u8>] {
    let tim = service::images::load_image(file_name).await;
    ByteStream! {
        yield tim;
    }
}
