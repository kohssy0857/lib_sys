// #[derive(Debug)]
use rusqlite::{params, Connection, Result};
use std::io;

fn main() -> Result<()> {
    let cn = Connection::open_in_memory()?;
    cn.execute(
        "CREATE TABLE book (id INTEGER,title TEXT,auther TEXT,page INTEGER,publisher TEXT,price INTEGER)",
        params![],
    )?;
    let mut stmt = cn.prepare(
        "INSERT INTO book (id,title,auther,page,publisher,price) VALUES (?,?,?,?,?,?)",
    )?;
    stmt.execute(params![
        20,
        "吾輩は猫である",
        "夏目漱石",
        231,
        "青空文庫",
        1600,
    ])?;
    println!("1 テーブル作成");
    println!("2 レコード追加");
    loop {
        // ........
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        if &guess.trim() == &"1" {
        } else if &guess.trim() == &"2" {
        } else if &guess.trim() == &"3" {
            let mut stmt = cn.prepare("select * from book ")?;
            let mut rows = stmt.query(params![])?;

            while let Some(row) = rows.next()?{
                let id: i32 = row.get(0)?;
                let title: String = row.get(1)?;
                let auther: String = row.get(2)?;
                let page: i32 = row.get(3)?;
                let publisher: String = row.get(4)?;
                let price: i32 = row.get(5)?;
                println!("{} {} {} {} {}", title, auther, page, publisher, price);
            }
        } else if &guess.trim() == &"4" {
        } else if &guess.trim() == &"5" {
            println!("修正したいレコードのidを入力してください。");
            let mut number_id = String::new();
            io::stdin().read_line(&mut number_id).ok();
            let id:i32 = number_id.trim().parse().ok().unwrap();

            println!("タイトルを入力してください。");
            let mut title = String::new();
            io::stdin().read_line(&mut title).ok();

            println!("著者を入力してください。");
            let mut auther = String::new();
            io::stdin().read_line(&mut auther).ok();

            println!("ページ数を入力してください。");
            let mut number_page = String::new();
            io::stdin().read_line(&mut number_page).ok();
            let page:i32 = number_page.trim().parse().ok().unwrap();

            println!("出版社を入力してください。");
            let mut publisher = String::new();
            io::stdin().read_line(&mut publisher).ok(); 

            println!("値段を入力してください。");
            let mut number_price = String::new();
            io::stdin().read_line(&mut number_price).ok(); 
            let price:i32 = number_price.trim().parse().ok().unwrap();

            let s = format!("update book set title = '{}', auther = '{}', page = '{}', publisher = '{}', price = '{}' where id = {}", title, auther, page, publisher, price, id);
            cn.execute(&s, params![])?;
        } else {
            continue;
        }
    }
    Ok(())
}
