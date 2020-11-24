use std::env;
//use std::vec;
mod parse_state; //::ParseState;

//use parse_state::ParseState; //( ParseState );

//type StageFn = fn(&String, &mut Vec<String>) -> bool;

//static mut next_fn: StageFn = check_for_yarn;

fn main() {

    //let mut output_tokens : Vec<String> = Vec::new();

    //let mut output_tokens : Vec<&str> = Vec::new();

    //let mut stage = 0;

    //let mut isGlobal:bool = false;

    let mut p_state: parse_state::ParseState = parse_state::ParseState::new();

    //p_state.nextFn = check_for_yarn;

    //let mut stg_fn: StageFn = check_for_yarn;

    //p_state.nextFn = check_for_yarn;

    p_state.next_fn = skip_first_arg;

    let mut is_in_error = false;

    let args = env::args();

    //let args_size_hint = args.size_hint();

    let mut count = 0;

    for item in args
    {

        /*
        if count == 0
        {

            continue;

        }
        */

        let n_fn = p_state.next_fn;
        
        if !n_fn(&item, &mut p_state)
        {

            //error

            is_in_error = true;

            break;

        }

        //println!("{}", item);

        count += 1;

    }

    //println!("{}", count);

    if count < 2
    {

        println!("{}", "No arguments provided");

        return;

    }


    if !is_in_error
    {

        println!("{}", p_state.to_string());

    }

}

fn check_for_yarn(item: &String, output_tokens: &mut parse_state::ParseState) -> bool
{

    if item == "yarn"
    {

        //let npm_string = String::from("npm");

        //output_tokens.command = npm_string;

        //output_tokens.push(npm_string);

        //assign next function

        output_tokens.next_fn = check_for_second_argument;

        //return true;

    }
    else
    {

        output_tokens.next_fn = parsing_error;

    }

    //parsing error

    //false

    true

}

fn check_for_second_argument(item: &String, output_tokens: &mut parse_state::ParseState) -> bool
{

    let current_str = &item[..];

    match current_str {

        /*

        "run" => {

        },
        "search" => {

        }
        */

        "add" => {

            output_tokens.command = "install".to_string();

        },
        "config" => {

            output_tokens.command = "config".to_string();

        },
        "init" => {

            output_tokens.command = "init".to_string();

        },
        _ => {

            //error

            output_tokens.next_fn = parsing_error;

            return true;

        }

    }

    //next

    output_tokens.next_fn = parse_rest;

    true

    /*
    if item == "yarn"
    {

        let npm_string = String::from("npm");

        output_tokens.arguments.push(npm_string);
        */
        //assign next function
        /*
        return true;

    }
    */
    //parsing error

    //false

}

//other arguments

//rest of arguments

fn parse_rest(item: &String, output_tokens: &mut parse_state::ParseState) -> bool
{

    output_tokens.arguments.push(item.to_string());

    true

}

//#[allow(dead_code)]
fn parsing_error(_item: &String, _output_tokens: &mut parse_state::ParseState) -> bool
{

    println!("{}", "Parsing Error");

    false

}

fn skip_first_arg(_item: &String, output_tokens: &mut parse_state::ParseState) -> bool
{

    output_tokens.next_fn = check_for_yarn;

    true

}

//https://docs.npmjs.com/cli/v6/
//https://yarnpkg.com/cli/config

