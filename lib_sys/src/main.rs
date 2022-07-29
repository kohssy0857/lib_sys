// #[derive(Debug)]
use rusqlite::{params, Connection, Result};
use std::io;
struct Book {
    id: i32,
    title: String,
    auther: String,
    page: i32,
    publisher: String,
    price: i32,
}

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
    stmt.execute(params![
        10,
        "ノルウェーの森",
        "村上春樹",
        459,
        "幻冬舎",
        1400,
    ])?;
    println!("テーブルを作成しました。");
    loop {
        println!("1 テーブル作成");
        println!("2 レコード追加");
        println!("6asc レコードを価格の昇順で表示");
        println!("6desc レコードを価格の降順で表示");
        // ........
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        let mut guess = guess.trim();
        if guess == "1" {
        } else if guess == "2" {
        } else if guess == "3" {
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line.");
        } else if guess == "4" {
        } else if guess == "5" {
        } else if guess == "6asc" {
            let mut stmt = cn.prepare(
                "select * from book order by price asc"
            )?;
            let mut rows = stmt.query(params![])?;
            while let Some(row) = rows.next()? {
                let id: i32 = row.get(0)?;
                let title: String = row.get(1)?;
                let auther: String = row.get(2)?;
                let page: i32 = row.get(3)?;
                let publisher: String = row.get(4)?;
                let price: i32 = row.get(5)?;
                println!("id: {}, title: {}, author: {}, page: {}, publisher: {}, price: {}",
                    id, title, auther, page, publisher, price);
            }
        } else if guess == "6desc" {
            let mut stmt = cn.prepare(
                "select * from book order by price desc"
            )?;
            let mut rows = stmt.query(params![])?;
            while let Some(row) = rows.next()? {
                let id: i32 = row.get(0)?;
                let title: String = row.get(1)?;
                let auther: String = row.get(2)?;
                let page: i32 = row.get(3)?;
                let publisher: String = row.get(4)?;
                let price: i32 = row.get(5)?;
                println!("id: {}, title: {}, auther: {}, page: {}, publisher: {}, price: {}",
                    id, title, auther, page, publisher, price);
            }
        } else {
            continue;
        }
    }
    Ok(())
}