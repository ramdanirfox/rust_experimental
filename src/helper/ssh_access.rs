use std::borrow::Cow;
use std::net::ToSocketAddrs;
///
/// Run this example with:
/// cargo run --example client_exec_simple -- -k <private key path> <host> <command>
///
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use russh::keys::*;
use russh::*;
use tokio::io::AsyncWriteExt;


pub async fn initialize_ssh() -> Result<()> {
    // env_logger::builder()
    //     .filter_level(log::LevelFilter::Debug)
    //     .init();

    // CLI options are defined later in this file
    // let cli = Cli::parse();

    // info!("Connecting to {}:{}", cli.host, cli.port);
    // info!("Key path: {:?}", cli.private_key);

    // Session is a wrapper around a russh client, defined down below
    // let mut ssh = Session::connect(
    //     cli.private_key,
    //     cli.username.unwrap_or("root".to_string()),
    //     (cli.host, cli.port),
    // )
    // .await?;
    // info!("Connected");

    // let addr = "localhost:22".to_socket_addrs().unwrap().next().unwrap();
    // let mut ssh = Session::connect_password(AsRef::<str>::as_ref("root"), "root", addr).await?;
    let password_str = std::env::var("SSH_PASSWORD_TEST").unwrap_or("env_undefined".to_string());
    let user_str = std::env::var("SSH_USERNAME_TEST").unwrap_or("env_undefined".to_string());
    let addresses_tuple = (std::env::var("SSH_ADDRESS_TEST").unwrap_or("env_undefined".to_string()), 22);
    println!("Menghubungkan");
    // let t: tokio::task::JoinHandle<Result<()>> = tokio::spawn(async move {
    let mut ssh = Session::connect_password(password_str, user_str, addresses_tuple).await;
    match ssh {
        Ok(mut ssh) => {
            // Handle the successful connection
            // ...
            println!("Connected");
            // let cmd = std::env::var("SSH_COMMAND_TEST").unwrap_or("env_undefined".to_string());
            // let res = ssh.call(cmd.as_str()).await?;
            let res = ssh.call_interactive("ssh user@ssh.address.com").await?;
            println!("Output: {}", std::char::from_u32(res).unwrap());
            ssh.close().await?;
        }
        Err(err) => {
            eprintln!("Error connecting: {}", err);
            return Err(err);
        }
    }
    // Ok(())
    // });
    

    // let code = ssh
    //     .call(
    //         &cli.command
    //             .into_iter()
    //             .map(|x| shell_escape::escape(x.into())) // arguments are escaped manually since the SSH protocol doesn't support quoting
    //             .collect::<Vec<_>>()
    //             .join(" "),
    //     )
    //     .await?;

    // println!("Exitcode: {:?}", code);
    Ok(())
}

pub async fn initialize_sftp() -> Result<()> {
    let password_str = std::env::var("SSH_PASSWORD_TEST").unwrap_or("env_undefined".to_string());
    let user_str = std::env::var("SSH_USERNAME_TEST").unwrap_or("env_undefined".to_string());
    let addresses_tuple = (std::env::var("SSH_ADDRESS_TEST").unwrap_or("env_undefined".to_string()), 22);
    println!("Menghubungkan SFTP");
    // let t: tokio::task::JoinHandle<Result<()>> = tokio::spawn(async move {
    let mut ssh = Session::connect_password(password_str, user_str, addresses_tuple).await;
    match ssh {
        Ok(mut ssh) => {
            // Handle the successful connection
            // ...
            println!("Connected");
            // let cmd = std::env::var("SSH_COMMAND_TEST").unwrap_or("env_undefined".to_string());
            // let res = ssh.call(cmd.as_str()).await?;
            let res = ssh.call_interactive("ssh user@ssh.address.com").await?;
            println!("Output: {}", std::char::from_u32(res).unwrap());
            ssh.close().await?;
        }
        Err(err) => {
            eprintln!("Error connecting: {}", err);
            return Err(err);
        }
    }
    Ok(())
}

struct Client {}

// More SSH event handlers
// can be defined in this trait
// In this example, we're only using Channel, so these aren't needed.
#[async_trait]
impl client::Handler for Client {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &ssh_key::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

/// This struct is a convenience wrapper
/// around a russh client
pub struct Session {
    session: client::Handle<Client>,
}

impl Session {
    // async fn connect<P: AsRef<Path>, A: ToSocketAddrs>(
    //     key_path: P,
    //     user: impl Into<String>,
    //     addrs: A,
    // ) -> Result<Self> {
    //     let key_pair = load_secret_key(key_path, None)?;
    //     let config = client::Config {
    //         inactivity_timeout: Some(Duration::from_secs(5)),
    //         preferred: Preferred {
    //             // kex: Cow::Owned(vec![russh::kex::DH_GEX_SHA256]),
    //             kex: Cow::Owned(vec![russh::kex::DH_G14_SHA256]),
    //             ..Default::default()
    //         },
    //         ..<_>::default()
    //     };

    //     let config = Arc::new(config);
    //     let sh = Client {};

    //     let mut session = client::connect(config, addrs, sh).await?;
    //     let auth_res = session
    //         .authenticate_publickey(user, PrivateKeyWithHashAlg::new(Arc::new(key_pair), None)?)
    //         .await?;

    //     if !auth_res {
    //         anyhow::bail!("Authentication failed: {auth_res:?}");
    //     }

    //     Ok(Self { session })
    // }

    async fn connect_password<P: AsRef<str>, A: ToSocketAddrs + tokio::net::ToSocketAddrs>(
        pw: P,
        user: impl Into<String>,
        addrs: A,
    ) -> Result<Self> {
        // let key_pair = load_secret_key(key_path, None)?;
        let config = client::Config {
            inactivity_timeout: Some(Duration::from_secs(5)),
            preferred: Preferred {
                // kex: Cow::Owned(vec![russh::kex::DH_GEX_SHA256]),
                kex: Cow::Owned(vec![russh::kex::DH_G14_SHA256]),
                ..Default::default()
            },
            ..<_>::default()
        };

        let config = Arc::new(config);
        let sh = Client {};

        let mut session = client::connect(config, addrs, sh).await?;
        let auth_res = session
            .authenticate_password(user, pw.as_ref())
            // .authenticate_publickey(user, PrivateKeyWithHashAlg::new(Arc::new(key_pair), None)?)
            .await?;

        if !auth_res {
            anyhow::bail!("Authentication failed: {auth_res:?}");
        }

        Ok(Self { session })
    }

    async fn call(&mut self, command: &str) -> Result<u32> {
        let mut channel = self.session.channel_open_session().await?;
        channel.exec(true, command).await?;

        let mut code = None;
        let mut stdout = tokio::io::stdout();

        loop {
            // There's an event available on the session channel
            let Some(msg) = channel.wait().await else {
                break;
            };
            match msg {
                // Write data to the terminal
                ChannelMsg::Data { ref data } => {
                    stdout.write_all(data).await?;
                    stdout.flush().await?;
                }
                // The command has returned an exit code
                ChannelMsg::ExitStatus { exit_status } => {
                    code = Some(exit_status);
                    // cannot leave the loop immediately, there might still be more data to receive
                }
                _ => {}
            }
        }
        Ok(code.expect("program did not exit cleanly"))
    }

    async fn call_interactive(&mut self, command: &str) -> Result<u32> {
        let mut channel = self.session.channel_open_session().await?;

        // This example doesn't terminal resizing after the connection is established
        // let (w, h) = termion::terminal_size()?;

        // Request an interactive PTY from the server
        channel
            .request_pty(
                false,
                "xterm".into(),
                128 as u32,
                128 as u32,
                0,
                0,
                &[], // ideally you want to pass the actual terminal modes here
            )
            .await?;
        channel.exec(true, command).await?;

        let mut code = 9999;
        // let mut stdin = tokio_fd::AsyncFd::try_from(0)?;
        // let mut stdout = tokio_fd::AsyncFd::try_from(1)?;
        let mut stdout = tokio::io::stdout();
        // let mut buf = vec![0; 1024];
        let stdin_closed = false;

        // loop {
            // Handle one of the possible events:
            // tokio::select! {
                // There's terminal input available from the user
                // channel.data(&buf[..n]).await?;
                // There's an event available on the session channel
                loop {
                let msg = channel.wait().await;
                match msg {
                        // Write data to the terminal
                        Some(ChannelMsg::Data { ref data }) => {
                            stdout.write_all(data).await?;
                            stdout.flush().await?;
                            println!("Data PTY Diterima {:?}", stdout);
                            // let output = Vec::new();
                            let output_str = String::from_utf8(data.to_vec()).unwrap();
                            // println!("OutputStr {:?}", output_str);
                            if output_str.contains("Are you sure") {
                                println!("Found 'Are you sure' in output");
                                channel.data("yes\n".as_bytes()).await?;
                                break;
                            }
                        }
                        // The command has returned an exit code
                        Some(ChannelMsg::ExitStatus { exit_status }) => {
                            println!("Error Brok {}", exit_status);
                            code = exit_status;
                            if !stdin_closed {
                                channel.eof().await?;
                            }
                            break;
                        }
                        Some(wha) => {println!("Apa ini {:?}", wha);}
                        None => {println!("Udahan"); break;}
                    }
                }

                // let guardian = "SemogaBerkah!";
                println!("Mengisi data...");
                // let guardian = "y";
                // channel.data(guardian.as_bytes()).await?;
                println!("Data Terisi");

                loop {
                    let msg = channel.wait().await;
                    match msg {
                            // Write data to the terminal
                            Some(ChannelMsg::Data { ref data }) => {
                                stdout.write_all(data).await?;
                                stdout.flush().await?;
                                println!("Data PTY Diterima Part2 {:?}", stdout);
                            }
                            // The command has returned an exit code
                            Some(ChannelMsg::ExitStatus { exit_status }) => {
                                println!("Error Brok Part2 {}", exit_status);
                                code = exit_status;
                                if !stdin_closed {
                                    channel.eof().await?;
                                }
                                break;
                            }
                            Some(wha) => {println!("Apa ini Part2 {:?}", wha);}
                            None => {println!("Udahan Part 2"); break;}
                        }
                }

                println!("Data PTY Akhir {:?}", stdout);    
            // }
        // }
        Ok(code)
    }

    async fn close(&mut self) -> Result<()> {
        self.session
            .disconnect(Disconnect::ByApplication, "", "English")
            .await?;
        Ok(())
    }
}

// #[derive(clap::Parser)]
// #[clap(trailing_var_arg = true)]
// pub struct Cli {
//     #[clap(index = 1)]
//     host: String,

//     #[clap(long, short, default_value_t = 22)]
//     port: u16,

//     #[clap(long, short)]
//     username: Option<String>,

//     #[clap(long, short = 'k')]
//     private_key: PathBuf,

//     #[clap(multiple = true, index = 2, required = true)]
//     command: Vec<String>,
// }
