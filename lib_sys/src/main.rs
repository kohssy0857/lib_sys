use rusqlite::{params, Connection, Result};
use std::io;
#[derive(Debug)]
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
    let mut stmt =
        cn.prepare("INSERT INTO book (id,title,auther,page,publisher,price) VALUES (?,?,?,?,?,?)")?;
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
        let guess = guess.trim();
        if guess == "1" {
        } else if guess == "2" {
        } else if guess == "3" {
            println!("値を検索するカラムを選択してください");
            println!("1:id");
            println!("2:タイトル");
            println!("3:作者");
            println!("4:ページ数");
            println!("5:出版社");
            println!("6:価格");

            let mut col = String::new();
            io::stdin()
                .read_line(&mut col)
                .expect("Failed to read line.");
            let col = col.trim();
            println!("検索したい文字列、値を入力してください");
            let mut find = String::new();
            io::stdin()
                .read_line(&mut find)
                .expect("Failed to read line.");
            let find = find.trim();
            if col == "1" {
                let mut n = cn.prepare(r"SELECT * FROM book where id == ? ")?;
                let iter = n.query_map(params![&find], |row| {
                    Ok(Book {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        auther: row.get(2)?,
                        page: row.get(3)?,
                        publisher: row.get(4)?,
                        price: row.get(5)?,
                    })
                })?;
                for it in iter {
                    println!("{:?}", it);
                }
            } else if col == "2" {
                let mut n = cn.prepare(r"SELECT * FROM book where title LIKE '%?%'")?;
                let iter = n.query_map(params![&find], |row| {
                    Ok(Book {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        auther: row.get(2)?,
                        page: row.get(3)?,
                        publisher: row.get(4)?,
                        price: row.get(5)?,
                    })
                })?;
                for it in iter {
                    println!("{:?}", it);
                }
            } else if col == "3" {
                let mut n = cn.prepare(r"SELECT * FROM book where auther LIKE '%?%' ")?;
                let iter = n.query_map(params![&find], |row| {
                    Ok(Book {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        auther: row.get(2)?,
                        page: row.get(3)?,
                        publisher: row.get(4)?,
                        price: row.get(5)?,
                    })
                })?;
                for it in iter {
                    println!("{:?}", it);
                }
            } else if col == "4" {
                let mut n = cn.prepare(r"SELECT * FROM book where page ==? ")?;
                let iter = n.query_map(params![&find], |row| {
                    Ok(Book {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        auther: row.get(2)?,
                        page: row.get(3)?,
                        publisher: row.get(4)?,
                        price: row.get(5)?,
                    })
                })?;
                for it in iter {
                    println!("{:?}", it);
                }
            } else if col == "5" {
                let mut n = cn.prepare(r"SELECT * FROM book where publisher LIKE '%?%' ")?;
                let iter = n.query_map(params![&find], |row| {
                    Ok(Book {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        auther: row.get(2)?,
                        page: row.get(3)?,
                        publisher: row.get(4)?,
                        price: row.get(5)?,
                    })
                })?;
                for it in iter {
                    println!("{:?}", it);
                }
            } else if col == "6" {
                let mut n = cn.prepare(r"SELECT * FROM book where price== ?% ")?;
                let iter = n.query_map(params![&find], |row| {
                    Ok(Book {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        auther: row.get(2)?,
                        page: row.get(3)?,
                        publisher: row.get(4)?,
                        price: row.get(5)?,
                    })
                })?;
                for it in iter {
                    println!("{:?}", it);
                }
            }
        } else if guess == "4" {
        } else if guess == "5" {
        } else {
            continue;
        }
    }
    Ok(())
}
