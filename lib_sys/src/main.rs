use std::io;


fn main() {
    loop{
        println!("1 テーブル作成");
        println!("2 レコード追加");
        // ........
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line.");
        if &guess=="1"{

        }else if &guess=="2"{
            
        }else if &guess=="3"{

        }else if &guess=="4"{

        }else if &guess=="5"{

        }else{
            continue;
        }
    }

}
