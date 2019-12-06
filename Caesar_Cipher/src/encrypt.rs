pub fn encrypt(phrase: String, shift: u8) -> String{
    let mut enc_phrase = String::new();
    for curr_letter in phrase.chars(){
        let letter:char;
        if curr_letter >= 'A' && curr_letter <= 'Z'{
            letter = (((((curr_letter as u8) - ('A' as u8)) + shift) % 26) + ('A' as u8)) as char;
        }else if curr_letter >= 'a' && curr_letter <= 'z'{            
            letter = (((((curr_letter as u8) - ('a' as u8)) + shift) % 26) + ('a' as u8)) as char;
        }else{
            letter = curr_letter;
        }
        enc_phrase.push(letter);
    }
    enc_phrase
}
