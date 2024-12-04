use core::str;
use nom::IResult;

fn mul_parser(input: &[u8]) -> IResult<&[u8], &[u8]> {
    nom::bytes::complete::tag("mul")(input)
}

fn digit(input: &[u8]) -> IResult<&[u8], i32> {
    nom::combinator::map_res(
        nom::bytes::complete::take_while_m_n(1, 3, nom::character::is_digit),
        |s: &[u8]| str::from_utf8(s).unwrap().parse::<i32>(),
    )(input)
}

fn open_paren(input: &[u8]) -> IResult<&[u8], &[u8]> {
    nom::bytes::complete::tag("(")(input)
}

fn close_paren(input: &[u8]) -> IResult<&[u8], &[u8]> {
    nom::bytes::complete::tag(")")(input)
}

fn comma(input: &[u8]) -> IResult<&[u8], &[u8]> {
    nom::bytes::complete::tag(",")(input)
}

// parse mul(1,2)
fn mul_expr(input: &[u8]) -> IResult<&[u8], (i32, i32)> {
    let (input, _) = mul_parser(input)?;
    let (input, _) = open_paren(input)?;
    let (input, a) = digit(input)?;
    let (input, _) = comma(input)?;
    let (input, b) = digit(input)?;
    let (input, _) = close_paren(input)?;
    Ok((input, (a, b)))
}

fn part_one() -> i32 {
    let mut input = &include_bytes!("input.txt")[..];
    let mut result = 0;

    loop {
        if input.is_empty() {
            break;
        }

        match mul_expr(input) {
            Ok((i, (a, b))) => {
                result += a * b;
                input = i;
            }
            Err(err) => {
                err.map(|e| {
                    if !e.input.is_empty() {
                        input = &e.input[1..];
                    }
                    ()
                });
            },
        }
    }

    result
}

fn main() {
    println!("Part one: {}", part_one());
}
