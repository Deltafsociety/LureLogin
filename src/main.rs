mod db;
mod geo;
mod shell;
mod tui;

use chrono::Utc;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

use db::log_attempt;
use geo::get_country;
use shell::fake_response;

async fn handle_client(stream: TcpStream) {
    let peer = stream.peer_addr().unwrap();
    let ip = peer.ip().to_string();

    let country = get_country(&ip);

    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    writer
        .write_all(b"SSH-2.0-OpenSSH_8.2p1 Ubuntu-4ubuntu0.5\r\n")
        .await
        .unwrap();

    writer.write_all(b"login: ").await.unwrap();

    let mut username = String::new();
    reader.read_line(&mut username).await.unwrap();

    writer.write_all(b"password: ").await.unwrap();

    let mut password = String::new();
    reader.read_line(&mut password).await.unwrap();

    let username = username.trim();
    let password = password.trim();

    println!(
        "[+] {} {} -> {}:{}",
        country, ip, username, password
    );

    writer
        .write_all(b"\nWelcome to Ubuntu 22.04 LTS\n")
        .await
        .unwrap();

    loop {
        writer.write_all(b"root@ubuntu:~# ").await.unwrap();

        let mut cmd = String::new();

        let bytes = reader.read_line(&mut cmd).await.unwrap();

        if bytes == 0 {
            break;
        }

        let cmd = cmd.trim();

        let timestamp = Utc::now().to_rfc3339();

        log_attempt(
            &timestamp,
            &ip,
            &country,
            username,
            password,
            cmd,
        );

        let response = fake_response(cmd);

        writer.write_all(response.as_bytes()).await.unwrap();
}
