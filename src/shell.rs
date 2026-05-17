pub fn fake_response(cmd: &str) -> String {
    match cmd.trim() {
        "whoami" => "root\n".into(),
        "uname -a" => "Linux ubuntu 5.15.0-91-generic x86_64 GNU/Linux\n".into(),
        "pwd" => "/root\n".into(),
        "ls" => "server.py\nbackup.tar.gz\nconfig\n".into(),
        "id" => "uid=0(root) gid=0(root) groups=0(root)\n".into(),
        "cat /etc/passwd" => {
            "root:x:0:0:root:/root:/bin/bash\nubuntu:x:1000:1000::/home/ubuntu:/bin/bash\n".into()
        }
        "exit" => "logout\n".into(),
        _ => format!("bash: {}: command not found\n", cmd.trim()),
    }
