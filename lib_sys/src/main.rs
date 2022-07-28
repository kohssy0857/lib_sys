// #[derive(Debug)]
use rusqlite::{params, Connection, Result};
use std::io;

fn main() -> Result<()> {
    loop {
        let cn = Connection::open("library.db")?;
        println!("1 テーブル作成");
        println!("2 レコード追加");
        // ........
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        // geussの改行コードを取り除く
        let guess = guess.trim();
        if guess == "1" {
            cn.execute(
                "CREATE TABLE book (id INTEGER,title TEXT,auther TEXT,page INTEGER,publisher TEXT,price INTEGER)",
                params![],
            )?;
            println!("create table");
        } else if guess == "2" {
        } else if guess == "3" {
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line.");
        } else if guess == "4" {
        } else if guess == "5" {
        } else {
            continue;
        }
    }
    Ok()
}