extern crate structopt;
extern crate rand;
extern crate unicode_segmentation;

use structopt::StructOpt;
use rand::Rng;
use unicode_segmentation::UnicodeSegmentation;

#[derive(StructOpt)]
struct Opt {
    /// Input the string to print
    #[structopt(default_value = "default")]
    message: String,
}

fn main() {
    let options                 = Opt::from_args();
    let message                 = options.message.clone();
    let message_length          = message.len();
    let mut border              = String::from("+");
    let mut default_message     = String::new();
    let final_msg: String;
    let chars                   = vec!['⋩', '⋪', '⋫', '⋬', '⋭', '⋮', '⋰', '⋯', '⋱',
                                       '⋲', '⋳', '⋴', '⋵', '⋶', '⋷', '⋸', '⋹', '⋺',
                                       '⋻', '⋼', '⋽', '⋾', '⋿', '⁰', '∱', '∳', '∾'];
    if message == "default" {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let i = rng.gen_range(0, chars.len());
            let char = chars[i as usize];
            default_message.push(char);
        }
    }

    let default_message_as_vec  = UnicodeSegmentation::graphemes(default_message.as_str(), true).collect::<Vec<&str>>();
    let default_message_length  = default_message_as_vec.len();

    if message == "default" {
        for _ in 0..default_message_length+2 {
            border.push('-');
        }
        final_msg = default_message;
    } else {
        for _ in 0..message_length+2 {
            border.push('-');
        }
        final_msg = message;
    }

    border.push('+');
    let ascii_art = format!(r#"
                     {}
                     | {} |
                     {}
                            .-.
             .-""`""-.    |(@ @)
          _/`oOoOoOoOo`\_ \ \-/
         '.-=-=-=-=-=-=-.' \/ \
     jgs   `-=.=-.-=.=-'    \ /\
              ^  ^  ^       _H_ \"#, border, final_msg, border);
    println!("{}", ascii_art);
}
