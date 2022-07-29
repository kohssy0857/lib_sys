
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
        println!("0 終了");
        // ........
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        if &guess.trim() == &"1" {
          println!("1番が押された");
        } else if &guess.trim() == &"2" {
        } else if &guess.trim() == &"3" {
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("Failed to read line.");
        } else if &guess.trim() == &"4" {
          loop{
            println!("↓↓");
            println!("↓↓");
            println!("idを入力してください");
            println!("00 終了");
            let mut stmt = cn.prepare("select * from book")?;
            // クロージャを使用してデータを取得して構造体 book にセット
            for it in stmt.query_map(params![], |row| {
              Ok(Book {
                id: row.get(0)?,
                title: row.get(1)?,
                auther: row.get(2)?,
                page: row.get(3)?,
                publisher: row.get(4)?,
                price: row.get(5)?,
              })
            })? {
              // 表示
              println!("{:?}", it.unwrap());
            }
            let mut num = String::new();
            // 入力
            io::stdin().read_line(&mut num).ok();
            // 入力したidの値を数値に変換
            let id: i32 = num.trim().parse().ok().unwrap();
            // sql文の作成
            let del_string = format!("delete from book where id = {}",id);
            cn.execute(&del_string,params![])?;
            println!("delete OK.");
            if &num.trim() == &"00"{
              break
            } else {
              continue
            }
          }
        } else if &guess.trim() == &"5" {
          
        } else if &guess.trim() == &"0" {
          break
        } else {
            continue;
        }
    }
    Ok(())
}