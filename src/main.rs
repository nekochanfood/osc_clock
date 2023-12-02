extern crate rosc;
use std::fmt::Debug;
use std::thread;
use serde::{Serialize, Deserialize};
use std::net::{UdpSocket, SocketAddr};
use rosc::{OscPacket, OscMessage, OscType};
use chrono::{Local, Timelike, Datelike};
use std::fs::File;
use std::{env,fs};
use std::io::{self, Read, Write};

static VERSION: f32 = 0.3;


#[derive(Serialize, Deserialize, Debug,  Clone)]
struct Config {
    sender_ip: String,
    sender_port: u16,
    receiver_ip: String,
    receiver_port: u16,
    show_debug_log: bool,
    addresses: Vec<String>,
    update_handle_addresses: Vec<String>,
}

enum LogType {
    INFO,
    WARN,
    ERROR,
    EVENT,
    SEND
}

fn get_fallback_config() -> Config {
    let config = Config {
        sender_ip: "127.0.0.1".to_string(),
        sender_port: 9000,
        receiver_ip: "127.0.0.1".to_string(),
        receiver_port: 9001,
        show_debug_log: false,
        addresses: vec![
            "/avatar/parameters/osc_clock@second_f".to_string(),
            "/avatar/parameters/osc_clock@second_i".to_string(),
            "/avatar/parameters/osc_clock@minute_f".to_string(),
            "/avatar/parameters/osc_clock@minute_i".to_string(),
            "/avatar/parameters/osc_clock@hour24_f".to_string(),
            "/avatar/parameters/osc_clock@hour24_i".to_string(),
            "/avatar/parameters/osc_clock@hour12_f".to_string(),
            "/avatar/parameters/osc_clock@hour12_i".to_string(),
            "/avatar/parameters/osc_clock@hour_isPM".to_string(),
            "/avatar/parameters/osc_clock@day".to_string(),
            "/avatar/parameters/osc_clock@dofw".to_string(),
            "/avatar/parameters/osc_clock@month".to_string(),
            "/avatar/parameters/osc_clock@year".to_string(),
            "/avatar/parameters/osc_clock@year_0".to_string(),
            "/avatar/parameters/osc_clock@year_1".to_string(),
            "/avatar/parameters/osc_clock@year_2".to_string(),
            "/avatar/parameters/osc_clock@year_3".to_string()
        ],
        update_handle_addresses: vec![
            "/avatar/parameters/osc_clock@ForceSync".to_string()
        ]
    };
    return config;
}

fn read_config_json(json_path: &str) -> Result<Config, io::Error> {
    let mut file = File::open(json_path)?;

    let mut json = String::new();
    let _ = file.read_to_string(&mut json)?;
    let config: Config = serde_json::from_str(&json)?;

    Ok(config)
}

fn repair_config_json() -> Result<bool, io::Error> {
    let mut file: File;
    let path = std::path::Path::new("./config.json");
    if path.is_file() {
        fs::remove_file("./config.json")?;
        file = File::create("./config.json")?;
    }else{
        file = File::open("./config.json")?;
    }
    let json = serde_json::to_string_pretty(&get_fallback_config())?;
    file.write_all(json.as_bytes()).expect(&print_log("Failed to write to file".to_string(), LogType::ERROR));
    
    Ok(true)
}

fn main() {
    //JSON修復用
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        if args[1] == "repair" {
            match repair_config_json() {
                Ok(_) => {
                    print_flush(print_log("修復しました".to_string(),LogType::INFO));
                }
                Err(_error) => {
                    print_flush(print_log("Err: config.json の修復に失敗しました。config.jsonが存在する場合、削除してから実行してみてください。".to_string(),LogType::ERROR));
                }
            }
            print!("続行するには何かキーを押してください...");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            return;
        }
    }
    // タイトル
    print!("OSC Clock v{0:.1}\n",VERSION);
    print!("Ctrl+C で終了\n\n");

    let  config;
    match read_config_json("./config.json") {
        Ok(result) => {
            
            config = result;
        }
        Err(_error) => {
            print_flush(print_log("config.json の読み込みに失敗しました。代わりにデフォルトの設定を使用します。".to_string(),LogType::WARN));
            print_flush(print_log("`\\osc_clock.exe repair` でconfig.jsonを初期状態に戻します。".to_string(),LogType::INFO));
            config = get_fallback_config();
        }
    }

    let thread0_sender_ip = config.sender_ip.clone();
    let thread0_sender_port = config.sender_port.clone();
    let thread0_receiver_ip = config.receiver_ip.clone();
    let thread0_receiver_port= config.receiver_port.clone();
    let thread0_update_handle_addresses = config.update_handle_addresses.clone();
    let thread0_addresses = config.addresses.clone();

    let thread1_sender_ip = config.sender_ip.clone();
    let thread1_sender_port = config.sender_port.clone();
    let thread1_addresses = config.addresses.clone();
    let thread1_show_debug_log = config.show_debug_log.clone();

    let thread0 = thread::spawn(move || {
        let receiver_address: SocketAddr = (thread0_receiver_ip.to_string() + ":" + &thread0_receiver_port.to_string()).parse().expect(&print_log("Failed to parse address".to_string(), LogType::ERROR));
        let socket = UdpSocket::bind(receiver_address).expect(&print_log("Failed to bind socket".to_string(), LogType::ERROR));
        
        print_flush(print_log(format!("{} からのパケットを受信します", receiver_address),LogType::INFO));
    
        loop {
            let mut buf = [0; 2048];
            let (size, _) = socket.recv_from(&mut buf).expect(&print_log("Failed to receive data".to_string(), LogType::ERROR));
            match rosc::decoder::decode_udp(&buf[..size]) {
                Ok(packet) => {
                    match packet {
                        
                        (_,OscPacket::Message(msg)) => {
                            let update: bool;
                            match msg.args[0] {
                                OscType::Bool(b) => {
                                    if b {
                                        update = true;
                                    }else{
                                        update = false;
                                    }
                                }
                                _ => {update = true;}
                            }
                            if update {
                                for n in 0..thread0_update_handle_addresses.len() {
                                    if msg.addr.to_string() == thread0_update_handle_addresses[n].to_string() {
                                        print_flush(print_log(format!("パラメータ同期判定アドレスからのパケットを受信:\t{}", msg.addr.to_string()),LogType::EVENT));
                                        let sync_toggle = vec![true,true,true];
                                        composition(thread0_addresses.to_vec(),thread0_sender_ip.to_string(),thread0_sender_port,sync_toggle,false);
                                        print_flush(print_log(format!("同期しました\t({})",Local::now().format("%Y-%m-%d %H:%M:%S")),LogType::SEND));
                                        break;
                                    }
                                }
                            }
                        }
                        (_,OscPacket::Bundle(bundle)) => {
                            print_flush(print_log(format!("Received OSC Bundle: {:?}", bundle),LogType::INFO));
                        }
                    }
                }
                Err(err) => {
                    print_flush(print_log(format!("Error decoding OSC message: {:?}", err),LogType::INFO));
                }
            }   
        }
    });

    let thread1 = thread::spawn(move || {
        // 接続先
        print_flush(print_log(format!("{}:{} に送信します",thread1_sender_ip,thread1_sender_port),LogType::INFO));

        let mut dt = Local::now();

        // 比較用
        let mut current_second = dt.second();
        let mut current_minute = u32::MAX;
        let mut current_hour = u32::MAX;
        let mut current_day = u32::MAX;
            
        // .000 秒まで待つ
        while dt.second() == current_second {
            thread::sleep(std::time::Duration::from_millis(10));
            dt = Local::now();
        }
            
        loop {
            let send_minute: bool;
            let send_hour: bool;
            let send_day: bool;

            while dt.second() == current_second {
                thread::sleep(std::time::Duration::from_millis(10));
                dt = Local::now();
            }
            if dt.minute() != current_minute {
                send_minute = true;
                current_minute = dt.minute();
            }else{
                send_minute = false;
            }
            if dt.hour() != current_hour {
                send_hour = true;
                current_hour = dt.hour();
            }else{
                send_hour = false;
            }
            if dt.day() != current_day {
                send_day = true;
                current_day = dt.day();
            }else{
                send_day = false;
            }
            let sync_toggle = vec![send_minute,send_hour,send_day];
            composition(thread1_addresses.to_vec(),thread1_sender_ip.to_string(),thread1_sender_port,sync_toggle,thread1_show_debug_log);
            current_second = dt.second();
        }
    });
    
    thread0.join().unwrap();
    thread1.join().unwrap();
}

fn make_message(addr: &str,args: Vec<OscType>) -> OscMessage{
    let message = OscMessage {
        addr: addr.to_string(),
        args: args
    };
    return message;

}

fn print_flush(str: String){
    println!("{}",str);
    match io::stdout().flush() {
        Ok(_) => {},
        Err(_) => std::process::exit(1)
    }
}

fn print_log(str: String,log_type: LogType) -> String{
    let prefix;
    match log_type {
        LogType::INFO => {
            prefix = format!("[\x1b[{}mINFO\x1b[m]\t",30+6);
        },
        LogType::WARN => {
            prefix = format!("[\x1b[{}mWARN\x1b[m]\t",30+3);
        },
        LogType::ERROR => {
            prefix = format!("[\x1b[{}mERROR\x1b[m]\t",30+1);
        },
        LogType::EVENT => {
            prefix = format!("[\x1b[{}mEVENT\x1b[m]\t",30+5);
        },
        LogType::SEND => {
            prefix = format!("[\x1b[{}mSEND\x1b[m]\t",30+2);
        },
    }
    return format!("{}{}\n",prefix,str);
}

fn composition(addresses: Vec<String>, ip: String, port: u16,sync_toggle: Vec<bool>,show_debug_log: bool){
    let dt = Local::now();

        let second = dt.second();
        let second_f = make_message(&addresses[0], vec![OscType::Float((second as f32/60.0) as f32)]);
        let second_i = make_message(&addresses[1], vec![OscType::Int((second) as i32)]);


        // minute
        let minute = dt.minute();

        let minute_f = make_message(&addresses[2], vec![OscType::Float((minute as f32/60.0) as f32)]);
        let minute_i = make_message(&addresses[3], vec![OscType::Int((minute) as i32)]);


        // hour24
        let hour24 = dt.hour();

        let hour24_f = make_message(&addresses[4], vec![OscType::Float((hour24 as f32/24.0) as f32)]);
        let hour24_i = make_message(&addresses[5], vec![OscType::Int((hour24) as i32)]);


        //  hour12 & isPM
        let is_pm = dt.hour12();

        let hour12 = if is_pm.1 == 12 {0} else {is_pm.1};

        let hour12_f = make_message(&addresses[6], vec![OscType::Float(((hour12 as f32/12.0) as f32) + ((minute as f32/60.0)/12.0))]);
        let hour12_i = make_message(&addresses[7], vec![OscType::Int((hour12) as i32)]);

        let hour_is_pm = make_message(&addresses[8], vec![OscType::Bool((is_pm.0) as bool)]);


        // day
        let day = dt.day();

        let day_i = make_message(&addresses[9], vec![OscType::Int((day) as i32)]);


        // dofw
        let dofw = dt.weekday();

        let dofw_i = make_message(&addresses[10], vec![OscType::Int((dofw) as i32)]);


        // month
        let month = dt.month();

        let month_i = make_message(&addresses[11], vec![OscType::Int((month) as i32)]);


        // year
        let year = dt.year();

        let year_i = make_message(&addresses[12], vec![OscType::Int((year) as i32)]);


        // splitted year
        let year_0 = make_message(&addresses[13], vec![OscType::Int((year / 1000) as i32)]);
        let year_1 = make_message(&addresses[14], vec![OscType::Int(((year % 1000) / 100) as i32)]);
        let year_2 = make_message(&addresses[15], vec![OscType::Int(((year % 100) / 10) as i32)]);
        let year_3 = make_message(&addresses[16], vec![OscType::Int((year % 10) as i32)]);


        send(second_f,&ip,port);
        send(second_i,&ip,port);
        
        send(hour24_f,&ip,port);
        send(minute_f,&ip,port);
        send(hour12_f,&ip,port);

        if sync_toggle[0] == true {
            send(minute_i,&ip,port);
        }
        if sync_toggle[1] == true {
            send(hour24_i,&ip,port);
            send(hour_is_pm,&ip,port);
            send(hour12_i,&ip,port);    
        }
        if sync_toggle[2] == true {
            send(day_i,&ip,port);
            send(dofw_i,&ip,port);
            send(month_i,&ip,port);
            send(year_i,&ip,port);
            send(year_0,&ip,port);
            send(year_1,&ip,port);
            send(year_2,&ip,port);
            send(year_3,&ip,port); 
        }
        if show_debug_log {
            print_flush(print_log(format!("{0}:{1} に値を送信 ({2})\t(分: {3:<5} | 時間: {4:<5} | 日付: {5:<5})",ip,port,dt.format("%Y-%m-%d %H:%M:%S.%f"),sync_toggle[0],sync_toggle[1],sync_toggle[2]), LogType::SEND));
        }
}

// 送信用
fn send(message: OscMessage,ip: &str,port: u16){
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let addr = SocketAddr::new(ip.parse().unwrap(), port);
        
    let packet = OscPacket::Message(message);
    let encoded_packet = rosc::encoder::encode(&packet).unwrap();

    socket.send_to(&encoded_packet, addr).unwrap();
}