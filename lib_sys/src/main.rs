
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
  let library = Connection::open_in_memory()?;
        library.execute(
            "CREATE TABLE book (id INTEGER,title TEXT,auther TEXT,page INTEGER,publisher TEXT,price INTEGER)",
            params![],
        )?;
        let mut stmt = library.prepare(
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
        println!("1:テーブル作成");
        println!("2:レコード追加");
        println!("3:レコード検索");
        println!("4:レコード削除");
        println!("5:レコード編集");
        println!("6asc レコードを価格の昇順で表示");
        println!("6desc レコードを価格の降順で表示");
        println!("7:レコード検索(範囲指定)");
        println!("0:終了");
        // ........
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
         if &guess.trim() == &"2" {

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
            let mut stmt = library.prepare("insert into book(id,title,auther,page,publisher,price) values (?,?,?,?,?,?)")?;
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

        } else if &guess.trim() == &"3" {
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
                let mut n = library.prepare(r"SELECT * FROM book where id == ? ")?;
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
                let mut n = library.prepare(
                    &format!("SELECT * FROM book where title LIKE '%{}%' ", find).to_string(),
                )?;
                let iter = n.query_map(params![], |row| {
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
                let mut n = library.prepare(
                    &format!("SELECT * FROM book where auther LIKE '%{}%' ", find).to_string(),
                )?;
                let iter = n.query_map(params![], |row| {
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
                let mut n = library.prepare(r"SELECT * FROM book where page ==? ")?;
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
                let mut n = library.prepare(
                    &format!("SELECT * FROM book where publisher LIKE '%{}%' ", find).to_string(),
                )?;
                let iter = n.query_map(params![], |row| {
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
                let mut n = library.prepare(r"SELECT * FROM book where price== ?% ")?;
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
        } else if &guess.trim() == &"4" {
          loop{
            println!("↓↓");
            println!("↓↓");
            let mut stmt = library.prepare("select * from book")?;
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
            println!("00:終了");
            println!("idを入力してください");
            let mut num = String::new();
            // 入力
            io::stdin().read_line(&mut num).ok();
            // 入力したidの値を数値に変換
            let id: i32 = num.trim().parse().ok().unwrap();
            // sql文の作成
            let del_string = format!("delete from book where id = {}",id);
            library.execute(&del_string,params![])?;
            println!("delete OK.");
            if &num.trim() == &"00"{
              break
            } else {
              continue
            }
          }
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
                let mut stmt = library.prepare(&se)?;
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
                let mut stmt = library.prepare(&se)?;
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
                let mut stmt = library.prepare(&se)?;
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
                let mut stmt = library.prepare(&se)?;
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
                let mut stmt = library.prepare(&se)?;
                let mut rows = stmt.query(params![])?;

                while let Some(row) = rows.next()?{
                    price = row.get(0)?;
                }
            }

            let s = format!("update book set title = '{}', auther = '{}', page = '{}', publisher = '{}', price = '{}' where id = {}", title, auther, page, publisher, price, id);
            library.execute(&s, params![])?;
        } else if &guess.trim() == &"6asc" {
            let mut stmt = library.prepare(
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
        } else if &guess.trim() == &"6desc" {
            let mut stmt = library.prepare(
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
        } else if &guess.trim() == &"7" {
            // /////////////////////////////////////////////////////////////////////
            println!("値を検索するカラムを選択してください");
            println!("1:id");
            println!("2:ページ数");
            println!("3:価格");

            let mut col = String::new();
            io::stdin()
                .read_line(&mut col)
                .expect("Failed to read line.");
            let col = col.trim();
            println!("下限の値を指定してください");
            let mut min = String::new();
            io::stdin()
                .read_line(&mut min)
                .expect("Failed to read line.");
            let min = min.trim();
            println!("上限の値を指定してください");
            let mut max = String::new();
            io::stdin()
                .read_line(&mut max)
                .expect("Failed to read line.");
            let max = max.trim();

            if col == "1" {
                let mut n = library.prepare(r"SELECT * FROM book where id BETWEEN ? AND ? ")?;
                let iter = n.query_map(params![min, max], |row| {
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
                let mut n = library.prepare(r"SELECT * FROM book where page LIKEBETWEEN ? AND ? ")?;
                let iter = n.query_map(params![min, max], |row| {
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
                let mut n = library.prepare(r"SELECT * FROM book where price LIKE BETWEEN ? AND ? ")?;
                let iter = n.query_map(params![min, max], |row| {
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
        }else if &guess.trim() == &"8" {
            println!("↓↓");
            println!("↓↓");
            let mut stmt = library.prepare("select * from book")?;
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
        } else if &guess.trim() == &"0" {
          break
        } else {
            continue;
        }
    }
    Ok(())
}