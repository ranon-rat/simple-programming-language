pub mod tokens;

// okay first time that i see that i need to do this
use crate::tokens::{LexerError, Tokens, get_single_token};

fn slash_match(
    out: &mut Vec<Tokens>,
    input: &[u8],
    string_part: &mut String,
    index: &mut usize,
    limit: usize,
    line_comment: &mut bool,      // /
    multiline_comment: &mut bool, // /* */
    inside_string: bool,
) {
    if inside_string {
        string_part.push('/');
        return;
    }
    if *index + 1 >= limit {
        return;
    }

    match input[*index + 1] as char {
        '/' => {
            *line_comment = true;
            *index += 1;
        }
        '*' => {
            *multiline_comment = true;
            *index += 1;
        }
        '=' => {
            out.push(Tokens::DivideTo);
            *index += 1;
        }
        _ => {
            out.push(Tokens::Divide);
        }
    }
    if string_part.len() > 0 {
        out.push(Tokens::Statement(string_part.to_string()));
        string_part.clear();
    }
}

fn quotes_match(
    out: &mut Vec<Tokens>,
    string_part: &mut String,
    inside_string: &mut bool,
    backslash: bool,
    inside_comment: bool,
) {
    if inside_comment {
        return;
    }
    if !*inside_string && string_part.len() > 0 {
        out.push(Tokens::Statement(string_part.to_string()));
        string_part.clear();
        return;
    }

    if *inside_string && !backslash {
        *inside_string = false;
        out.push(Tokens::String(string_part.to_string()));
        string_part.clear();
        return;
    }
    if *inside_string && backslash {
        string_part.push('"');
        return;
    }
    if !*inside_string {
        *inside_string = true;
        return;
    }
}
fn space_match(
    cell: char,
    out: &mut Vec<Tokens>,
    string_part: &mut String,
    inside_string: bool,
    inside_comment: bool,
    line_comment: &mut bool,
) {
    if cell == '\n' {
        *line_comment = false;
    }
    if inside_comment {
        return;
    }
    if inside_string {
        string_part.push(cell);
        return;
    }
    if string_part.len() > 0 {
        out.push(Tokens::Statement(string_part.to_string()));
        string_part.clear();
    }
}

fn arithmetic_match(
    cell: char,
    out: &mut Vec<Tokens>,
    input: &[u8],
    string_part: &mut String,
    index: &mut usize,
    limit: usize,
    multiline_comment: &mut bool,
    line_comment: bool,
    inside_string: bool,
) {
    if *multiline_comment {
        if cell != '*' {
            return;
        }
        if *index + 1 > limit {
            return;
        }
        let next: char = input[*index + 1] as char;
        if next == '/' {
            *multiline_comment = false;
            *index += 1;
        }
        return;
    }
    if line_comment {
        return;
    }

    if !inside_string && string_part.len() > 0 {
        out.push(Tokens::Statement(string_part.to_string()));
        string_part.clear();
    }
    if inside_string {
        string_part.push(cell);
        return;
    }

    match cell {
        '+' => {
            if *index + 1 > limit {
                out.push(Tokens::Add);
                return;
            }
            let next = input[*index + 1] as char;
            match next {
                '+' => {
                    out.push(Tokens::Increment);
                    *index += 1;
                }
                '=' => {
                    out.push(Tokens::AddTo);
                    *index += 1;
                }
                _ => {
                    out.push(Tokens::Add);
                }
            }
        }
        '-' => {
            if *index + 1 > limit {
                out.push(Tokens::Subtract);
                return;
            }
            let next = input[*index + 1] as char;
            match next {
                '-' => {
                    out.push(Tokens::Decrement);
                    *index += 1;
                }
                '=' => {
                    out.push(Tokens::SubtractTo);
                    *index += 1;
                }
                _ => {
                    out.push(Tokens::Subtract);
                }
            }
        }
        '*' => {
            two_cases_arithmetic(
                Tokens::Multiply,
                Tokens::MultiplyTo,
                out,
                input,
                index,
                limit,
            );
        }
        '/' => {
            two_cases_arithmetic(Tokens::Divide, Tokens::Divide, out, input, index, limit);
        }
        '%' => {
            two_cases_arithmetic(Tokens::Mod, Tokens::ModTo, out, input, index, limit);
        }

        _ => {}
    }
}
// x= // its just for that XD
fn two_cases_arithmetic(
    action: Tokens,
    action_to: Tokens,
    out: &mut Vec<Tokens>,
    input: &[u8],
    index: &mut usize,
    limit: usize,
) {
    if *index + 1 > limit {
        out.push(action);
        return;
    }
    let next = input[*index + 1] as char;
    match next {
        '=' => {
            out.push(action_to);
            *index += 1;
        }
        _ => {
            out.push(action);
        }
    }
}
fn escape_characters_match(
    cell: char,
    new: char,
    inside_comment: bool,
    inside_string: bool,
    backslash: bool,
    string_part: &mut String,
) {
    if inside_comment {
        return;
    }
    if inside_string && backslash {
        string_part.push(new);
        return;
    }
    string_part.push(cell);
}
fn single_tokens(
    cell: char,
    new: Tokens,
    out: &mut Vec<Tokens>,
    string_part: &mut String,
    inside_comment: bool,
    inside_string: bool,
) {
    if inside_comment {
        return;
    }
    if inside_string {
        string_part.push(cell);
        return;
    }
    if string_part.len() > 0 {
        out.push(Tokens::Statement(string_part.to_string()));
        string_part.clear();
    }
    out.push(new);
}
fn boolean_operators(
    cell: char,
    out: &mut Vec<Tokens>,
    input: &[u8],
    index: &mut usize,
    limit: usize,
    string_part: &mut String,
    inside_comment: bool,
    inside_string: bool,
) -> Result<(), LexerError> {
    // & | !
    if inside_comment {
        return Ok(());
    }
    if inside_string {
        string_part.push(cell);
        return Ok(());
    }
    if string_part.len() > 0 {
        out.push(Tokens::Statement(string_part.to_string()));
        string_part.clear();
    }
    let next_index = *index + 1;
    let has_next = next_index < limit;
    match cell {
        '&' => {
            if !has_next {
                return Err(LexerError::OutOfBounds);
            }
            let next_token = input[next_index] as char;
            match next_token {
                '&' => {
                    out.push(Tokens::AndBool);
                    *index += 1;
                    Ok(())
                }
                c => return Err(LexerError::UnexpectedOperator(c)),
            }
        }
        '|' => {
            if !has_next {
                return Err(LexerError::OutOfBounds);
            }
            let next_token = input[next_index] as char;
            match next_token {
                '|' => {
                    out.push(Tokens::OrBool);
                    *index += 1;
                    Ok(())
                }
                c => return Err(LexerError::UnexpectedOperator(c)),
            }
        }

        _ => Err(LexerError::Unreachable),
    }
}

// logical operators

fn logical_operators(
    cell: char,
    out: &mut Vec<Tokens>,
    input: &[u8],
    index: &mut usize,
    limit: usize,
    string_part: &mut String,
    inside_comment: bool,
    inside_string: bool,
) {
    if inside_comment {
        return;
    }
    if inside_string {
        string_part.push(cell);
        return;
    }
    if string_part.len() > 0 {
        out.push(Tokens::Statement(string_part.to_string()));
        string_part.clear();
    }
    match cell {
        '=' => two_cases_arithmetic(Tokens::Equals, Tokens::EqualsTo, out, input, index, limit),
        '>' => two_cases_arithmetic(
            Tokens::BiggerThan,
            Tokens::BiggerThanOrEqual,
            out,
            input,
            index,
            limit,
        ),
        '<' => two_cases_arithmetic(
            Tokens::SmallerThan,
            Tokens::SmallerOrEqual,
            out,
            input,
            index,
            limit,
        ),
        '!' => two_cases_arithmetic(
            Tokens::NotBool,
            Tokens::IsDifferent,
            out,
            input,
            index,
            limit,
        ),
        _ => {}
    }
}

pub fn tokenize(program: &String) -> Result<Vec<Tokens>, LexerError> {
    let mut out: Vec<Tokens> = vec![];
    let mut inside_string = false;
    let mut line_comment = false;
    let mut multiline_comment = false;

    let mut backslash = false;
    let program_bytes = program.as_bytes();
    let mut i = 0;
    let limit = program_bytes.len();
    let mut string_part: String = "".to_string();
    while i < limit {
        let cell = program_bytes[i] as char;
        let inside_comment = multiline_comment || line_comment;
        match cell {
            '/' => {
                backslash = false;
                slash_match(
                    &mut out,
                    program_bytes,
                    &mut string_part,
                    &mut i,
                    limit,
                    &mut line_comment,
                    &mut multiline_comment,
                    inside_string,
                );
            }
            '"' => {
                quotes_match(
                    &mut out,
                    &mut string_part,
                    &mut inside_string,
                    backslash,
                    inside_comment,
                );
                backslash = false;
            }
            ' ' | '\n' | '\t' => {
                backslash = false;
                space_match(
                    cell,
                    &mut out,
                    &mut string_part,
                    inside_string,
                    inside_comment,
                    &mut line_comment,
                );
            }
            '*' | '+' | '-' | '%' => {
                backslash = false;
                arithmetic_match(
                    cell,
                    &mut out,
                    program_bytes,
                    &mut string_part,
                    &mut i,
                    limit,
                    &mut multiline_comment,
                    line_comment,
                    inside_string,
                );
            }
            'n' | 't' => {
                // this was about to generate a lot of internal problems haha
                escape_characters_match(
                    cell,
                    if cell == 'n' { '\n' } else { '\t' },
                    inside_comment,
                    inside_string,
                    backslash,
                    &mut string_part,
                );
                backslash = false;
            }
            '\\' => {
                if inside_string && backslash {
                    backslash = false;
                    string_part.push(cell);
                } else {
                    backslash = true;
                }
            }
            ',' | '(' | ')' | '{' | '}' => {
                let token = get_single_token(cell);
                backslash = false;
                single_tokens(
                    cell,
                    token,
                    &mut out,
                    &mut string_part,
                    inside_comment,
                    inside_string,
                );
            }
            '&' | '|' => {
                match boolean_operators(
                    cell,
                    &mut out,
                    program_bytes,
                    &mut i,
                    limit,
                    &mut string_part,
                    inside_comment,
                    inside_string,
                ) {
                    Err(v) => return Err(v),
                    _ => {}
                }
            }
            '!' | '=' | '<' | '>' => {
                logical_operators(
                    cell,
                    &mut out,
                    program_bytes,
                    &mut i,
                    limit,
                    &mut string_part,
                    inside_comment,
                    inside_string,
                );
            }
            ';' => {
                backslash = false;
                if inside_string {
                    string_part.push(cell);
                } else {
                    if string_part.len() > 0 {
                        out.push(Tokens::Statement(string_part.to_string()));
                        string_part.clear();
                    }
                    out.push(Tokens::SemmiColon);
                }
            }
            _ => {
                backslash = false;
                if !inside_comment {
                    string_part.push(cell);
                }
            }
        };

        i += 1;
    }
    Ok(out)
}
