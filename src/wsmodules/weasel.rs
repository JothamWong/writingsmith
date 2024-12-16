use crate::structs::{Message, MessageType, Module};
use std::{fs::File, io::{BufRead, BufReader}};
use phf::{phf_set};

static WEASEL_WORDS: phf::Set<&'static str> = phf_set! {
    "many",
    "various",
    "very",
    "fairly",
    "several",
    "extremely",
    "exceedingly",
    "quite",
    "remarkably",
    "few",
    "surprisingly",
    "mostly",
    "largely",
    "huge",
    "tiny",
    "excellent",
    "interestingly",
    "significantly",
    "substantially",
    "clearly",
    "vast",
    "relatively",
    "completely"
};

pub struct WeaselModule {}

impl Module for WeaselModule {
    fn check(filepath: &str) -> Vec<Message> {
        let mut messages: Vec<Message> = Vec::new();
        let reader = BufReader::new(File::open(filepath).expect("cannot open file"));
        let mut line_no: u32 = 0;
        const RED: &str = "\x1B[31m";
        const RESET: &str = "\x1B[0m";

        for line_result in reader.split(b'.') {
            line_no += 1;
            match line_result {
                Ok(bytes) => {
                    if let Ok(line) = String::from_utf8(bytes) {
                        let mut marked_sentence = line.clone();
                        let mut found_weasels: Vec<&str> = Vec::new();

                        // We do one pass and see if we find weasel words
                        for word in line.split_whitespace() {
                            let clean_word = word.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase();
                            if WEASEL_WORDS.contains(clean_word.as_str()) {
                                found_weasels.push(word);
                            }
                        }

                        if !found_weasels.is_empty() {
                            for word in found_weasels.iter() {
                                let colored_word = format!("{}{}{}", RED, word, RESET);
                                marked_sentence = marked_sentence.replace(word, &colored_word);
                            }
                        }

                        messages.push(Message{
                            message_type: MessageType::WeaselWord,
                            line_no: line_no,
                            message: marked_sentence,
                        });
                    }
                }
                Err(e) => {
                    panic!("Error reading line: {}", e);
                }
            }
        }
        messages
    }
}