use std::borrow::Cow;
use std::str::FromStr;

use nom::{Err,error::ErrorKind,IResult};
use nom::bytes::complete::{take_while, tag};
use nom::branch::alt;
use nom::character::complete::{multispace0, multispace1, alphanumeric1};
use nom::combinator::{map_res, opt};

#[derive(Debug, Clone)]
pub enum Command {
    ShowCpuCache,
    ShowRegs,
    Step(u32),
    Continue,
    Disassemble(u32),
    Goto(u32),
    ShowMem(u32),
    Label,
    AddLabel(String, u32),
    RemoveLabel(String),
    Breakpoint,
    AddBreakpoint(u32),
    RemoveBreakpoint(u32),
    Watchpoint,
    AddWatchpoint(u32),
    RemoveWatchpoint(u32),
    Exit,
    Repeat,
}

impl FromStr for Command {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match command(s) {
            Ok((_, c)) => Ok(c),
            err => Err(format!("Unable to parse command: {:?}", err).into()),
        }
    }
}

fn command(input: &str) -> IResult<&str, Command> {
    alt((
        show_cpu_cache,
        show_regs,
        step,
        continue_,
        disassemble,
        goto,
        show_mem,
        label,
        add_label,
        remove_label,
        breakpoint,
        add_breakpoint,
        remove_breakpoint,
        watchpoint,
        add_watchpoint,
        remove_watchpoint,
        exit,
        repeat,
    ))(input)
}

fn eof(input: &str) -> IResult<&str, ()> {
    if input.is_empty() {
        Ok((input, ()))
    } else {
        Err(Err::Error(error_position!(input, ErrorKind::Eof)))
    }
}

fn show_cpu_cache(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag("show_cpu_cache"), tag("scc")))(input)?;
    let (input, _) = eof(input)?;

    Ok((input, Command::ShowCpuCache))
}

fn show_regs(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag("show_regs"), tag("r")))(input)?;
    let (input, _) = eof(input)?;

    Ok((input, Command::ShowRegs))
}

fn step(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag("step"), tag("s")))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, steps) = opt(u32_)(input)?;
    let (input, _) = eof(input)?;

    let steps = match steps {
        Some(s) => s,
        None => 1,
    };

    Ok((input, Command::Step(steps)))
}

fn continue_(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag("continue"), tag("c")))(input)?;
    let (input, _) = eof(input)?;

    Ok((input, Command::Continue))
}

fn goto(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag("goto"), tag("g")))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, addr) = u32_hex(input)?;

    Ok((input, Command::Goto(addr)))
}

fn disassemble(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag("disassemble"), tag("disasm"), tag("d")))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, count) = u32_(input)?;
    let (input, _) = eof(input)?;
    
    Ok((input, Command::Disassemble(count)))
}

fn show_mem(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag("showmem"), tag("m")))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, addr) = u32_hex(input)?;

    Ok((input, Command::ShowMem(addr)))
}

fn label(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag("label") , tag("l")))(input)?;
    let (input, _) = eof(input)?;

    Ok((input, Command::Label))
}

fn add_label(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag("addlabel"), tag("al")))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, label) = alphanumeric1(input)?;
    let (input, _) = multispace1(input)?;
    let (input, addr) = u32_hex(input)?;
    let (input, _) = eof(input)?;

    Ok((input, Command::AddLabel(label.to_string(), addr)))
}

fn remove_label(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag("removelabel"), tag("rl")))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, label) = alphanumeric1(input)?;
    let (input, _) = eof(input)?;

    Ok((input, Command::RemoveLabel(label.to_string())))
}

fn breakpoint(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag("breakpoint"), tag("b")))(input)?;
    let (input, _) = eof(input)?;

    Ok((input, Command::Breakpoint))
}

fn add_breakpoint(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag("addbreakpoint"), tag("ab")))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, addr) = u32_hex(input)?;
    let (input, _) = eof(input)?;

    Ok((input, Command::AddBreakpoint(addr)))
}

fn remove_breakpoint(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag("removebreakpoint"), tag("rb")))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, addr) = u32_hex(input)?;
    let (input, _) = eof(input)?;

    Ok((input, Command::RemoveBreakpoint(addr)))
}

fn watchpoint(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag("watchpoint"), tag("w")))(input)?;
    let (input, _) = eof(input)?;

    Ok((input, Command::Watchpoint))
}

fn add_watchpoint(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag("addwatchpoint"), tag("aw")))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, addr) = u32_hex(input)?;
    let (input, _) = eof(input)?;

    Ok((input, Command::AddWatchpoint(addr)))
}

fn remove_watchpoint(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag("removewatchpoint"), tag("rw")))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, addr) = u32_hex(input)?;
    let (input, _) = eof(input)?;

    Ok((input, Command::RemoveWatchpoint(addr)))
}

fn exit(input: &str) -> IResult<&str, Command> {
    let (input, _) = alt((tag("exit"), tag("x")))(input)?;
    let (input, _) = eof(input)?;

    Ok((input, Command::Exit))
}

fn repeat(input: &str) -> IResult<&str, Command> {
    let (input, _) = eof(input)?;

    Ok((input, Command::Repeat))
}

fn u32_(input: &str) -> IResult<&str, u32> {
    map_res(
        take_while(|x: char| x.is_ascii_digit()),
        u32::from_str
    )(input)
}

fn u32_hex(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("0x")(input)?;
    let (input, digits) = take_while(|x:char| x.is_ascii_hexdigit())(input)?;

    let ret = u32::from_str_radix(&digits, 16);

    match ret {
        Ok(n) => Ok((input, n)),
        Err(_) => Err(Err::Error(error_position!(input, ErrorKind::Eof))),
    }
}
