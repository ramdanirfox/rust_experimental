use actix_web::{get, HttpResponse, Responder};

fn fn_satu() {
    println!("Terpanggil Fn Satu!");
}

#[utoipa::path(responses((status = OK, body = String)))]
#[get("/baca_tulis_excel")]
pub async fn baca_tulis_excel() -> impl Responder {
    fn_satu();
    fn_tulis_xlsx();
    HttpResponse::Ok().json("Excel Ditulis dan Dibaca!")
}


fn fn_tulis_xlsx() {
    use umya_spreadsheet::*;
    let mut book = new_file();

    // new worksheet
    let _ = book.new_sheet("Sheet2");
    // change value
    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_cell_mut("A1")
        .set_value("TEST1");
    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_cell_mut("B2")
        .set_value_number(1);
    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_cell_mut("C3")
        .set_value_bool(true);
    // or
    book.get_sheet_mut(&0)
        .unwrap()
        .get_cell_mut((1, 1))
        .set_value("TEST1");
    book.get_sheet_mut(&0)
        .unwrap()
        .get_cell_mut((2, 2))
        .set_value_number(1);
    book.get_sheet_mut(&0)
        .unwrap()
        .get_cell_mut((3, 3))
        .set_value_bool(true);

    let path = std::path::Path::new("./yyayaya.xlsx");
    // let path = std::path::Path::new("/mnt/d/DOS/musl");
    let _ = writer::xlsx::write(&book, path);
}
