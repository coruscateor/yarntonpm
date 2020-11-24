
type StageFn = fn(&String, &mut ParseState) -> bool;

pub struct ParseState {

    pub command: String,
    pub arguments: Vec<String>,
    pub next_fn: StageFn

}

impl ParseState {

    pub fn new() -> ParseState
    {

        ParseState
        {

            command: String::new(),
            arguments: Vec::new(),
            next_fn: empty_fn

        }

    }

    pub fn to_string(&self) -> String
    {

        let mut output: String = String::from("npm ");

        output.push_str(&self.command[..]);

        for item in &self.arguments
        {

            output.push(' ');

            output.push_str(&item[..]);

        }
        
        output

    }

}

fn empty_fn(_item: &String, _output_tokens: &mut ParseState) -> bool
{

    println!("{}", "Empty Function");

    false

}