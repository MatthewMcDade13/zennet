use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub mod conn;
pub mod http1;

pub fn read_all(stream: &mut TcpStream, mut n: u32) -> anyhow::Result<String> {
    let mut buf = vec![0u8; n as usize];

    while n > 0 {
        let rv = stream.read(&mut buf)?;
        if rv == 0 {
            break;
        }
        n = n - (rv as u32);
    }

    let s = String::from_utf8(buf)?;
    Ok(s)
}

pub fn write_string(stream: &mut TcpStream, msg: &str) -> anyhow::Result<()> {
    let writestr = msg.as_bytes();
    stream.write_all(writestr)?;
    Ok(())
}
