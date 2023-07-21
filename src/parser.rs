use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::IResult;
use nom::multi::separated_list1;
use crate::cpu::{Instruction, Opcode};

pub struct Directives {
    pub ip_register: usize,
}

pub fn parse(input: &str) -> IResult<&str, (Directives, Vec<Instruction>)> {
    let (input, directives) = parse_directives(input)?;
    let (input, instructions) = parse_instructions(input)?;

    Ok((input, (directives, instructions)))
}

fn parse_directives(input: &str) -> IResult<&str, Directives> {
    let (input, _) = tag("#ip ")(input)?;
    let (input, ip_register) = digit1(input)?;
    let (input, _) = line_ending(input)?;

    Ok((input, Directives {
        ip_register: ip_register.parse().unwrap(),
    }))
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, parse_instruction)(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, opcode) = parse_opcode(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, a) = digit1(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, b) = digit1(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, c) = digit1(input)?;

    Ok((input, (opcode, a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap())))
}

fn parse_addr(input: &str) -> IResult<&str, Opcode> {
    let (input, _) = tag("addr")(input)?;
    Ok((input, Opcode::Addr))
}

fn parse_addi(input: &str) -> IResult<&str, Opcode> {
    let (input, _) = tag("addi")(input)?;
    Ok((input, Opcode::Addi))
}

fn parse_mulr(input: &str) -> IResult<&str, Opcode> {
    let (input, _) = tag("mulr")(input)?;
    Ok((input, Opcode::Mulr))
}

fn parse_muli(input: &str) -> IResult<&str, Opcode> {
    let (input, _) = tag("muli")(input)?;
    Ok((input, Opcode::Muli))
}

fn parse_banr(input: &str) -> IResult<&str, Opcode> {
    let (input, _) = tag("banr")(input)?;
    Ok((input, Opcode::Banr))
}

fn parse_bani(input: &str) -> IResult<&str, Opcode> {
    let (input, _) = tag("bani")(input)?;
    Ok((input, Opcode::Bani))
}

fn parse_borr(input: &str) -> IResult<&str, Opcode> {
    let (input, _) = tag("borr")(input)?;
    Ok((input, Opcode::Borr))
}

fn parse_bori(input: &str) -> IResult<&str, Opcode> {
    let (input, _) = tag("bori")(input)?;
    Ok((input, Opcode::Bori))
}

fn parse_setr(input: &str) -> IResult<&str, Opcode> {
    let (input, _) = tag("setr")(input)?;
    Ok((input, Opcode::Setr))
}

fn parse_seti(input: &str) -> IResult<&str, Opcode> {
    let (input, _) = tag("seti")(input)?;
    Ok((input, Opcode::Seti))
}

fn parse_gtir(input: &str) -> IResult<&str, Opcode> {
    let (input, _) = tag("gtir")(input)?;
    Ok((input, Opcode::Gtir))
}

fn parse_gtri(input: &str) -> IResult<&str, Opcode> {
    let (input, _) = tag("gtri")(input)?;
    Ok((input, Opcode::Gtri))
}

fn parse_gtrr(input: &str) -> IResult<&str, Opcode> {
    let (input, _) = tag("gtrr")(input)?;
    Ok((input, Opcode::Gtrr))
}

fn parse_eqir(input: &str) -> IResult<&str, Opcode> {
    let (input, _) = tag("eqir")(input)?;
    Ok((input, Opcode::Eqir))
}

fn parse_eqri(input: &str) -> IResult<&str, Opcode> {
    let (input, _) = tag("eqri")(input)?;
    Ok((input, Opcode::Eqri))
}

fn parse_eqrr(input: &str) -> IResult<&str, Opcode> {
    let (input, _) = tag("eqrr")(input)?;
    Ok((input, Opcode::Eqrr))
}

fn parse_opcode(input: &str) -> IResult<&str, Opcode> {
    alt((
        parse_addr,
        parse_addi,
        parse_mulr,
        parse_muli,
        parse_banr,
        parse_bani,
        parse_borr,
        parse_bori,
        parse_setr,
        parse_seti,
        parse_gtir,
        parse_gtri,
        parse_gtrr,
        parse_eqir,
        parse_eqri,
        parse_eqrr,
    ))(input)
}