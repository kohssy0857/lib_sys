// #[derive(Debug)]
use rusqlite::{params, Connection, Result};
use std::io;

fn main() -> Result<()> {
    loop {
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
        println!("1 テーブル作成");
        println!("2 レコード追加");
        // ........
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        if &guess == "1" {
        } else if &guess.trim() == &"2" {
          loop{
            println!("追加したいレコードのidを入力してください。");
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
            // println!("{} {} {} {} {} {}",id,title,auther,page,publisher,price); //確認用    
            let mut stmt = cn.prepare("insert into book(id,title,auther,page,publisher,price) values (?,?,?,?,?,?)")?;
            stmt.execute(params![id,title,auther,page,publisher,price])?;
            println!("id {} のレコードを追加しました。",id);
            println!("1:レコードを追加\n2:レコードを追加しない");
            let mut menu = String::new();
            io::stdin()
                .read_line(&mut menu)
                .expect("Failed to read line.");
            if &menu.trim() == &"1" {
            }else if &menu.trim() == &"2" {
              break;
            } 
          }
        } else if &guess == "3" {
        } else if &guess == "4" {
        } else if &guess == "5" {
        } else {
            continue;
        }
    }
    Ok(())
}
