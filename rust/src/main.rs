use frunk::{hlist_pat, HList};
use mysql::{prelude::*, PooledConn};
use simple_xlsx_writer::{Row, WorkBook};
use std::fs::File;
use std::io::Write;

const PAGE_SIZE: i32 = 100;
const TOTAL_RESULTS: i32 = 3000;

type RowType = HList!(
    i32, String, String, String, String, String, String, String, String, String, String, String,
    String, String, String, String, String, String, String, String, String, String, String, String,
    String, String, String, String, String, String, String, String, String, String, String, String,
    String, String, String, String, String, String, String, String, String, String, String, String,
    String, String, String, String, String, String, String, String, String, String, String, String,
    String, String, String, String, String, String, String, String, String, String, String, String,
    String, String, String, String, String, String, String, String, String, String, String, String,
    String, String, String, String, String, String, String, String, String, String, String, String,
    String, String, String, String, String, String, String, String, String, String, String, String,
    String, String, String, String, String, String, String, String, String, String, String, String,
    String, String, String,
);

fn get(mut conn: PooledConn, offset: i32) -> Vec<Row<'static>> {
    let query = format!(
        "SELECT id,
    email,
    name,
    name1,
    name2,
    name3,
    name4,
    name5,
    name6,
    name7,
    name8,
    name9,
    name10,
    name11,
    name12,
    name13,
    name14,
    name15,
    name17,
    name18,
    name19,
    name100,
    name101,
    name102,
    name103,
    name104,
    name105,
    name106,
    name107,
    name108,
    name109,
    name110,
    name111,
    name112,
    name113,
    name114,
    name115,
    name116,
    name117,
    name118,
    name119,
    name120,
    name16,
    name20,
    name21,
    name22,
    name23,
    name24,
    name25,
    name26,
    name27,
    name28,
    name29,
    name30,
    name31,
    name32,
    name33,
    name34,
    name35,
    name36,
    name37,
    name38,
    name39,
    name40,
    name41,
    name42,
    name43,
    name44,
    name45,
    name46,
    name47,
    name48,
    name49,
    name50,
    name51,
    name52,
    name53,
    name54,
    name55,
    name56,
    name57,
    name58,
    name59,
    name60,
    name61,
    name62,
    name63,
    name64,
    name65,
    name66,
    name67,
    name68,
    name69,
    name70,
    name71,
    name72,
    name73,
    name74,
    name75,
    name76,
    name77,
    name78,
    name79,
    name80,
    name81,
    name82,
    name83,
    name84,
    name85,
    name86,
    name87,
    name88,
    name89,
    name90,
    name91,
    name92,
    name93,
    name94,
    name95,
    name96,
    name97,
    name98,
    name99
    FROM User2     \
    LIMIT {} \
    OFFSET {}",
        PAGE_SIZE, offset
    );
    let query_result = conn
        .query_map(query, |row: RowType| {
            let hlist_pat![
                id, email, name, name1, name2, name3, name4, name5, name6, name7, name8, name9,
                name10, name11, name12, name13, name14, name15, name17, name18, name19, name100,
                name101, name102, name103, name104, name105, name106, name107, name108, name109,
                name110, name111, name112, name113, name114, name115, name116, name117, name118,
                name119, name120, name16, name20, name21, name22, name23, name24, name25, name26,
                name27, name28, name29, name30, name31, name32, name33, name34, name35, name36,
                name37, name38, name39, name40, name41, name42, name43, name44, name45, name46,
                name47, name48, name49, name50, name51, name52, name53, name54, name55, name56,
                name57, name58, name59, name60, name61, name62, name63, name64, name65, name66,
                name67, name68, name69, name70, name71, name72, name73, name74, name75, name76,
                name77, name78, name79, name80, name81, name82, name83, name84, name85, name86,
                name87, name88, name89, name90, name91, name92, name93, name94, name95, name96,
                name97, name98, name99
            ] = row;

            let a = [
                id.to_string(),
                email,
                name,
                name1,
                name2,
                name3,
                name4,
                name5,
                name6,
                name7,
                name8,
                name9,
                name10,
                name11,
                name12,
                name13,
                name14,
                name15,
                name17,
                name18,
                name19,
                name100,
                name101,
                name102,
                name103,
                name104,
                name105,
                name106,
                name107,
                name108,
                name109,
                name110,
                name111,
                name112,
                name113,
                name114,
                name115,
                name116,
                name117,
                name118,
                name119,
                name120,
                name16,
                name20,
                name21,
                name22,
                name23,
                name24,
                name25,
                name26,
                name27,
                name28,
                name29,
                name30,
                name31,
                name32,
                name33,
                name34,
                name35,
                name36,
                name37,
                name38,
                name39,
                name40,
                name41,
                name42,
                name43,
                name44,
                name45,
                name46,
                name47,
                name48,
                name49,
                name50,
                name51,
                name52,
                name53,
                name54,
                name55,
                name56,
                name57,
                name58,
                name59,
                name60,
                name61,
                name62,
                name63,
                name64,
                name65,
                name66,
                name67,
                name68,
                name69,
                name70,
                name71,
                name72,
                name73,
                name74,
                name75,
                name76,
                name77,
                name78,
                name79,
                name80,
                name81,
                name82,
                name83,
                name84,
                name85,
                name86,
                name87,
                name88,
                name89,
                name90,
                name91,
                name92,
                name93,
                name94,
                name95,
                name96,
                name97,
                name98,
                name99,
            ];
            let mut row_xlsx: Row = Row::new();
            for item in a.iter() {
                row_xlsx.add_cell(item.clone().into());
            }
            return row_xlsx;
        })
        .unwrap();
    query_result
}

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
    workbook
        .get_new_sheet()
        .write_sheet(|sheet_writer| {
            loop {
                let conn = pool.get_conn().unwrap();
                let query_result = get(conn, offset);
                println!("1 {}, {}", offset, query_result.len());

                for row_formatted in query_result {
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
