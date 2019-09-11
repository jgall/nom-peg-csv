#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate nom;
extern crate nom_peg;
use nom::{AsChar, ErrorKind, IResult, InputTakeAtPosition};
use nom_peg::grammar;

pub fn csv_parser() -> impl Fn(&str) -> Result<(&str, Vec<Vec<String>>), nom::Err<&str>> {
    let csv = grammar! {
        comma: &'input str = ","
        dquote: &'input str = "\""
        lf: &'input str = "\u{0A}"
        ws: &'input str = (sp|htab)
        sp: &'input str = " "
        htab: &'input str = "\u{09}"
        cr: &'input str = "\u{0D}"
        crlf: &'input str = cr lf => {"\u{0D}\u{0A}"}
        dquote2: &'input str = dquote dquote => {"\"\""}

        textdata: &'input str = ::text_data
        non_escaped: String = (textdata)* => {println!("{:?}", &result); result.join("")}
        escaped: String = dquote <content: (textdata|comma|cr|lf|dquote2)*> dquote => {format!("\"{}\"", content.join(""))}
        field: String = (escaped|non_escaped)
        record: Vec<String> = <first: field> <rest: (comma field)*> => {
            let mut res: Vec<String> = vec![first];
            res.append(&mut rest.into_iter().map(|(_, r)| r).collect::<Vec<String>>());
            res
        }
        maybe_crlf: () = crlf => {}
                       | "" => {}
        file: Vec<Vec<String>> = <first: record> <second: (crlf record)*> maybe_crlf "\u{003}" => {
            let mut res: Vec<Vec<String>> = vec![first];
            res.append(&mut second.into_iter().map(|(_, v)| v).collect::<Vec<Vec<String>>>());
            res
        }
    };
    move |s| csv.file(s)
}

pub fn in_range<T>(start: char, end: char) -> impl Fn(T) -> IResult<T, T>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    move |input| {
        input.split_at_position(|item| {
            let item_char = item.as_char();
            start <= item_char && item_char <= end
        })
    }
}

pub fn text_data<T>(input: T) -> IResult<T, T>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position1(|item| !is_textdata(item.as_char()), ErrorKind::Eof)
}

/// TEXTDATA as seen here: https://tools.ietf.org/html/rfc4180#section-2
fn is_textdata(input: char) -> bool {
    (' ' <= input && input <= '!')
        || ('#' <= input && input <= '+')
        || ('-' <= input && input <= '~')
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_one_row() {
        let parser = csv_parser();
        let parsed = parser("item1,item2\u{003}");
        assert_eq!(parsed.unwrap().1, vec![vec!["item1", "item2"]]);
    }

    #[test]
    fn it_wors_two_row() {
        let parser = csv_parser();
        let parsed = parser("item1,item2\u{0D}\u{0A}row2item1,row2item2\u{003}");
        assert_eq!(
            parsed.unwrap().1,
            vec![vec!["item1", "item2"], vec!["row2item1", "row2item2"]]
        );
    }
    #[test]
    fn it_wors_three_row() {
        let parser = csv_parser();
        let parsed = parser(
            "item1,item2\u{0D}\u{0A}row2item1,row2item2\u{0D}\u{0A}row3item1,row3item2\u{003}",
        );
        assert_eq!(
            parsed.unwrap().1,
            vec![
                vec!["item1", "item2"],
                vec!["row2item1", "row2item2"],
                vec!["row3item1", "row3item2"]
            ]
        );
    }
}
