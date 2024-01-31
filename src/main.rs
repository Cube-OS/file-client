use std::process::Command;

fn main() {

    let args: Vec<String> = std::env::args().collect();

    let host_ip = &args[2];
    let max_no_chunks = &args[4];
    let location = &args[5];
    let remote_location = &args[6];

    let host_ip_clone = host_ip.clone();
    let max_no_chunks_clone = max_no_chunks.clone();
    let location_clone = location.clone();
    let remote_location_clone = remote_location.clone();

    let time_now = std::time::SystemTime::now();

    let output = std::thread::spawn(move || {
        Command::new("./kubos-file-client")
            .arg("-r")
            .arg(host_ip_clone)
            .arg("-m")
            .arg(max_no_chunks_clone)
            .arg("upload")
            .arg(location_clone)
            .arg(remote_location_clone)
            .stdout(std::process::Stdio::piped())
            .spawn() 
            .expect("failed to execute process")
            .wait_with_output().unwrap()
    });

    std::thread::sleep(std::time::Duration::from_millis(50));
    let host_ip_clone = host_ip.clone();
    let max_no_chunks_clone = max_no_chunks.clone();
    let location_clone = location.clone();
    let remote_location_clone = remote_location.clone();

    let output2 = std::thread::spawn(move || {
        Command::new("./kubos-file-client")
        .arg("-r")
        .arg(host_ip_clone)
        .arg("-m")
        .arg(max_no_chunks_clone)
        .arg("-P")
        .arg("8081")
        .arg("upload")
        .arg(location_clone)
        .arg(remote_location_clone)
        .stdout(std::process::Stdio::piped())
        .spawn() 
        .expect("failed to execute process")
        .wait_with_output().unwrap()
    });

    std::thread::sleep(std::time::Duration::from_millis(50));
    let host_ip_clone = host_ip.clone();
    let max_no_chunks_clone = max_no_chunks.clone();
    let location_clone = location.clone();
    let remote_location_clone = remote_location.clone();

    let output3 = std::thread::spawn(move || {
        Command::new("./kubos-file-client")
        .arg("-r")
        .arg(host_ip_clone)
        .arg("-m")
        .arg(max_no_chunks_clone)
        .arg("-P")
        .arg("8082")
        .arg("upload")
        .arg(location_clone)
        .arg(remote_location_clone)
        .stdout(std::process::Stdio::piped())
        .spawn() 
        .expect("failed to execute process")
        .wait_with_output().unwrap()
    });

    std::thread::sleep(std::time::Duration::from_millis(50));
    let host_ip_clone = host_ip.clone();
    let max_no_chunks_clone = max_no_chunks.clone();
    let location_clone = location.clone();
    let remote_location_clone = remote_location.clone();

    let output4 = std::thread::spawn(move || {
        Command::new("./kubos-file-client")
        .arg("-r")
        .arg(host_ip_clone)
        .arg("-m")
        .arg(max_no_chunks_clone)
        .arg("-P")
        .arg("8083")
        .arg("upload")
        .arg(location_clone)
        .arg(remote_location_clone)
        .stdout(std::process::Stdio::piped())
        .spawn() 
        .expect("failed to execute process")
        .wait_with_output().unwrap()
    });

    let output = output.join().unwrap();
    let output2 = output2.join().unwrap();
    let output3 = output3.join().unwrap();
    let output4 = output4.join().unwrap();

    print!("{}", String::from_utf8_lossy(&output.stdout));
    print!("{}", String::from_utf8_lossy(&output2.stdout));

    println!("Time taken: {:?}", time_now.elapsed().unwrap());
}

