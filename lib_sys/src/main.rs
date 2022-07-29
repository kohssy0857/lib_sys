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
    stmt.execute(params![
        10,
        "ノルウェーの森",
        "村上春樹",
        459,
        "幻冬舎",
        1400,
    ])?;
    loop {
        println!("1 テーブル作成");
        println!("2 レコード追加");
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

            println!("タイトルを修正しますか?y/n");
            let mut t = String::new();
            io::stdin().read_line(&mut t).ok();
            let mut title = String::new();
            if t.trim() == "y" {
                println!("タイトルを入力してください。");
                io::stdin().read_line(&mut title).ok();
            } else {
                let se = format!("select title from book where id = {}", id);
                let mut stmt = cn.prepare(&se)?;
                let mut rows = stmt.query(params![])?;

                while let Some(row) = rows.next()?{
                    title = row.get(0)?;
                }
            }

            println!("著者を修正しますか?y/n");
            let mut a = String::new();
            io::stdin().read_line(&mut a).ok();
            let mut auther = String::new();
            if a.trim() == "y" {
                println!("著者を入力してください。");
                io::stdin().read_line(&mut auther).ok();
            } else {
                let se = format!("select auther from book where id = {}", id);
                let mut stmt = cn.prepare(&se)?;
                let mut rows = stmt.query(params![])?;

                while let Some(row) = rows.next()?{
                    auther = row.get(0)?;
                }
            }

            println!("ページ数を修正しますか?y/n");
            let mut p = String::new();
            io::stdin().read_line(&mut p).ok();
            let mut number_page = String::new();
            let mut page:i32 = 0;

            if p.trim() == "y" {
                println!("ページ数を入力してください。");
                io::stdin().read_line(&mut number_page).ok();
                page = number_page.trim().parse().ok().unwrap();
            } else {
                let se = format!("select page from book where id = {}", id);
                let mut stmt = cn.prepare(&se)?;
                let mut rows = stmt.query(params![])?;

                while let Some(row) = rows.next()?{
                    page = row.get(0)?;
                }
            }

            println!("出版社を修正しますか?y/n");
            let mut p = String::new();
            io::stdin().read_line(&mut p).ok();
            let mut publisher = String::new();
            if p.trim() == "y" {
                println!("出版社を入力してください。");
                io::stdin().read_line(&mut publisher).ok(); 
            } else {
                let se = format!("select publisher from book where id = {}", id);
                let mut stmt = cn.prepare(&se)?;
                let mut rows = stmt.query(params![])?;

                while let Some(row) = rows.next()?{
                    publisher = row.get(0)?;
                }
            }

            println!("値段を修正しますか?y/n");
            let mut p = String::new();
            io::stdin().read_line(&mut p).ok();
            let mut number_price = String::new();
            let mut price: i32 = 0;

            if p.trim() == "y" {
                println!("値段を入力してください。");
                io::stdin().read_line(&mut number_price).ok(); 
                price = number_price.trim().parse().ok().unwrap();
            } else {
                let se = format!("select price from book where id = {}", id);
                let mut stmt = cn.prepare(&se)?;
                let mut rows = stmt.query(params![])?;

                while let Some(row) = rows.next()?{
                    price = row.get(0)?;
                }
            }

            let s = format!("update book set title = '{}', auther = '{}', page = '{}', publisher = '{}', price = '{}' where id = {}", title, auther, page, publisher, price, id);
            cn.execute(&s, params![])?;
        } else {
            continue;
        }
    }
    Ok(())
}
