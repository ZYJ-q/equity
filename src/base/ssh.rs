use log:: warn;
use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;

#[warn(dead_code)]
pub struct SshClient {
    addr: String,
    port: String,
    username: String,
    password: String,
    root_path: String,
    root_name: String,
    sess: Session,
}

#[warn(unused_variables)]
impl SshClient {
    pub fn new(
        addr: &str,
        port: &str,
        username: &str,
        password: &str,
        root_path: &str,
        root_name: &str,
    ) -> Self {
        let tcp = TcpStream::connect(format!("{}:{}", addr, port)).unwrap();
        let mut sess = Session::new().unwrap();

        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();

        sess.userauth_password(username, password).unwrap();
        assert!(sess.authenticated());
        Self {
            addr: String::from(addr),
            port: String::from(port),
            username: String::from(username),
            password: String::from(password),
            root_path: String::from(root_path),
            root_name: String::from(root_name),
            sess: sess,
        }
    }

    pub fn search_py_ps(&mut self) -> bool {
        if !self.sess.authenticated() {
            let tcp = TcpStream::connect(format!("{}:{}", self.addr, self.port)).unwrap();
            self.sess = Session::new().unwrap();

            self.sess.set_tcp_stream(tcp);
            self.sess.handshake().unwrap();

            self.sess.userauth_password(&self.username, &self.password).unwrap();

            assert!(self.sess.authenticated());
        }

        let mut res = String::new();
        let command = format!("ps aux | grep {}", self.root_name);

        let mut channel = self.sess.channel_session().unwrap();
        channel.exec(&command).unwrap();
        channel.read_to_string(&mut res).unwrap();
        channel.wait_close().unwrap();
        // print!("ssh res {}", res);

        let strs = res.split("\n");
        for str in strs {
            // print!("str python {}", str);
            // 判断字符串中是否包含 python 字段
            if str.contains("python") {
                return true;
            }
        }
        return false;
    }

    pub fn download_log(&mut self) -> String {
        let log_path = format!("{}{}.log", self.root_path, self.root_name);
        let mut contents = Vec::new();

        let (mut remote_file, stat) = self.sess.scp_recv(Path::new(&log_path)).unwrap();

        if stat.size() == 0 {
            warn!("no log file {}", log_path);
            return String::from("");
        }

        remote_file.read_to_end(&mut contents).unwrap();

        remote_file.send_eof().unwrap();
        remote_file.wait_eof().unwrap();
        remote_file.close().unwrap();
        remote_file.wait_close().unwrap();

        return self.find_error(contents);
    }

    fn find_error(&self, contents: Vec<u8>) -> String {
        let text = String::from_utf8(contents).unwrap();

        let strs = text.split("\n");
        for str in strs {
            if (str.contains("error on") || str.contains("Error on") || str.contains("[ERR]")) && !str.contains("ping_listen_key") {
                return String::from(str);
            }
        }

        return String::from("");
    }

    pub fn get_root_name(&self) -> String {
        return self.root_name.clone();
    }
}

// #[allow(dead_code)]
// fn remote(addr: &str, username: &str, password: &str) {
//     // Connect to the target SSH server
//     let tcp = TcpStream::connect(addr).unwrap();
//     let mut sess = Session::new().unwrap();
//     sess.set_tcp_stream(tcp);
//     sess.handshake().unwrap();

//     sess.userauth_password(username, password).unwrap();
//     assert!(sess.authenticated());

//     // command
//     let mut channel = sess.channel_session().unwrap();
//     channel.exec("ps aux").unwrap();
//     let mut s = String::new();
//     channel.read_to_string(&mut s).unwrap();
//     println!("{}", s);
//     channel.wait_close().unwrap();
//     println!("{}", channel.exit_status().unwrap());

//     upload a file and download a file
//     let mut remote_file = sess
//         .scp_send(Path::new("remote.txt"), 0o644, 10, None)
//         .unwrap();
//     remote_file.write(b"1234567890").unwrap();

//     let (mut remote_file, stat) = sess.scp_recv(Path::new("remote.txt")).unwrap();
//     println!("remote file size: {}", stat.size());
//     let mut contents = Vec::new();
//     remote_file.read_to_end(&mut contents).unwrap();
//     println!("{:?}", String::from_utf8(contents).unwrap());

//     remote_file.send_eof().unwrap();
//     remote_file.wait_eof().unwrap();
//     remote_file.close().unwrap();
//     remote_file.wait_close().unwrap();
// }
