use simple_xlsx_writer::{Row, WorkBook};
use std::fs::File;
use std::io::Write;
mod row_xlsx;

const PAGE_SIZE: i32 = 50;
const TOTAL_RESULTS: i32 = 30;

fn main() {
    println!("start");
    // env::set_var("RUST_BACKTRACE", "1");

    let mut files = File::create("example.xlsx").unwrap();
    let mut workbook = WorkBook::new(&mut files).unwrap();
    // let header_style = workbook.create_cell_style((255, 255, 255), (0, 0, 0));

    let mut offset = 0;
    let mut counter = 0;
    let url = mysql::Opts::from_url("mysql://root:mauFJcuf5dhRMQrjj@localhost:3306/base").unwrap();
    let pool = mysql::Pool::new(url).unwrap();
    let style = workbook.create_cell_style((255, 255, 255), (125, 125, 125));
    workbook
        .get_new_sheet()
        .write_sheet(|sheet_writer| {
            loop {
                let conn = pool.get_conn().unwrap();
                let rows = row_xlsx::get_rows(conn, offset, PAGE_SIZE);
                println!("1 {}, {}", offset, rows.len());

                for row in rows {
                    let mut row_formatted = Row::new();
                    row.iter().for_each(|value| {
                        row_formatted.add_cell((value.to_string(), &style).into())
                    });
                    sheet_writer.write_row(row_formatted).unwrap();
                    counter += 1;
                }

                println!("counter: {}", counter);
                offset += PAGE_SIZE;

                if offset >= TOTAL_RESULTS {
                    break;
                }
            }

            Ok(())
        })
        .expect("write excel error!");

    workbook.finish().expect("close excel error!");
    files.flush().unwrap();

    println!("finished");
}
