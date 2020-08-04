use std::io;
use regex::Regex;
use rand::Rng;

fn main() {
    println!("I'm the magic 8-ball. Ask me a question.");

    let mut question = String::new();

    io::stdin()
        .read_line(&mut question)
        .expect("Error reading line.");

    let regex = Regex::new(r"[^.!?]+\?").unwrap();

    if regex.is_match(&question) {
        let answers = vec![
            "It is certain.",
            "It is decidedly so.",
            "Without a doubt.",
            "Yes – definitely.",
            "You may rely on it.",
            "As I see it, yes.",
            "Most likely.",
            "Outlook good.",
            "Yes.",
            "Signs point to yes.",
            "Reply hazy, try again",
            "Ask again later.",
            "Better not tell you now.",
            "Cannot predict now.",
            "Concentrate and ask again.",
            "Don't count on it.",
            "My reply is no.",
            "My sources say no.",
            "Outlook not so good.",
            "Very doubtful."
        ];
    
        let mut rng = rand::thread_rng();
    
        println!("{}", answers[rng.gen_range(0, 20)]);
    } else {
        println!("Not a question.");
    }
}
