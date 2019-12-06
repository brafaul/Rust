use std::io;

mod encrypt;

fn main() {
    let done = false;
    while !done{
        println!("What phrase do you wish to encrypt");
        let mut phrase = String::new();
        io::stdin().read_line(&mut phrase)
            .expect("Failed to read line");
        phrase.make_ascii_lowercase();
        println!("How many places do you want to shift ");
        let mut temp_shift = String::new();
        io::stdin().read_line(&mut temp_shift)
            .expect("Failed to read line");
        let input_num: Option<u8> = temp_shift.trim().parse().ok();
        let shift_count = match input_num{
            Some(num) => num,
            None => {
                println!("Please input a number");
                return;
            }
        };
        if shift_count < 26{
            let enc_phrase:String = encrypt::encrypt(phrase, shift_count);
            println!("{}", enc_phrase);
        } else {
            println!("Please pick a number less than 26");
        }            
    }
}
