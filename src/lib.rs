use std::fs;

#[derive(Debug)]
#[derive(Clone)]
pub struct Word (String, u32);

pub struct Tally {
    list: Vec<Word>
}

pub fn get_tally(file_path: String) -> Vec<Word> {
    let file_contents: String = fs::read_to_string(file_path).expect("No such file");
    let iter: Vec<&str> = file_contents.split_whitespace().collect();
    let mut token = Tally {
        list: Vec::new()
    };
    token.calculate(iter)
}

impl Tally {
    fn calculate(&mut self, iter: Vec<&str>) -> Vec<Word> {
        let mut is_empty_string: Option<usize> = None;
        for word in iter {
            is_empty_string = self.tally(Tally::format_word(word));
        }
        
        match is_empty_string {
            Some(index) => { self.list.remove(index); },
            None => {}
        }
        self.list.clone()
    }

    fn tally(&mut self, val: Result<String, ()>) -> Option<usize> {
        let mut edited: bool = false;
        let word: String = val.unwrap_or_default();
    
        for mut item in &mut self.list {
            if item.0 == word {
                edited = true;
                item.1 += 1;
            }
        };
        if !edited {
            if word == "" { return Some(self.list.len()); };
            let new_word = Word {
                0: word,
                1: 1
            };
            self.list.push(new_word);
        }

        None
    }

    fn format_word(word: &str) -> Result<String, ()> {
        let mut chars: Vec<_> = word.to_string().to_lowercase().chars().collect();
        let alphabets: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

        while !alphabets.contains(&chars[0]) {
            chars.remove(0);
            if chars.len() == 0 {
                return Err(());
            }
        };
        while !alphabets.contains(&chars[chars.len() - 1]) { chars.remove(chars.len() - 1); };

        Ok(chars.into_iter().collect())
    }

    pub fn get_formatted_string(list: Vec<Word>) -> String {
        let mut res: String = String::new();
        let mut total: u32 = 0;

        for entry in &list {
            total += entry.1;
            res.push_str(&format!("{}: {}\n", entry.0, entry.1));
        };

        let mut ret: String = String::from(format!("Total number of words: {}, Total number of unique words: {} \n", total, list.len()));

        ret.push_str(&res);
        ret
    }
}