extern crate rosc;
use std::{thread,time};
use serde::{Serialize, Deserialize};
use std::net::{UdpSocket, SocketAddr};
use rosc::{OscPacket, OscMessage, OscType};
use chrono::{Local, Timelike, Datelike};
use std::fs::File;
use std::{env,fs};
use std::io::{self, Read, Write};

static VERSION: f32 = 0.2;


#[derive(Serialize, Deserialize, Debug)]
struct Config {
    ip: String,
    port: u16,
    update_rate: u64,
    addresses: Vec<String>,
}

fn get_fallback_config() -> Config {
    let config = Config {
        ip: "127.0.0.1".to_string(),
        port: 9000,
        update_rate: 1000,
        addresses: vec![
            "/avatar/parameters/second_f".to_string(),
            "/avatar/parameters/second_i".to_string(),
            "/avatar/parameters/minute_f".to_string(),
            "/avatar/parameters/minute_i".to_string(),
            "/avatar/parameters/hour24_f".to_string(),
            "/avatar/parameters/hour24_i".to_string(),
            "/avatar/parameters/hour12_f".to_string(),
            "/avatar/parameters/hour12_i".to_string(),
            "/avatar/parameters/hour_isPM".to_string(),
            "/avatar/parameters/day".to_string(),
            "/avatar/parameters/dofw".to_string(),
            "/avatar/parameters/month".to_string(),
            "/avatar/parameters/year".to_string(),
            "/avatar/parameters/year_0".to_string(),
            "/avatar/parameters/year_1".to_string(),
            "/avatar/parameters/year_2".to_string(),
            "/avatar/parameters/year_3".to_string()
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
    file.write_all(json.as_bytes()).expect("Failed to write to file");
    
    Ok(true)
}

fn main() {
    //JSON修復用
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        if args[1] == "repair" {
            match repair_config_json() {
                Ok(_) => {
                    println!("修復しました");
                }
                Err(_error) => {
                    println!("Err: config.json の修復に失敗しました。config.jsonが存在する場合、削除してから実行してみてください。");
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

    let  config;
    match read_config_json("./config.json") {
        Ok(result) => {
            config = result;
        }
        Err(_error) => {
            println!("\nWarn: config.json の読み込みに失敗しました。代わりにデフォルトの設定を使用します。");
            println!("Tip: .\\osc_clock.exe repair でconfig.jsonを初期状態に戻します。\n");
            config = get_fallback_config();
        }
    }

    fn make_message(addr: &str,args: Vec<OscType>) -> OscMessage{
        let message = OscMessage {
            addr: addr.to_string(),
            args: args
        };
        return message;

    }

    // 接続先
    print!("{}:{} に送信します\n\n",config.ip,config.port);

    print!("Ctrl+C で終了\n");

    // モニター上部分
    // うまく表示されないため封印

    // print!("モニター\n\ni = {0:<10}f = {1:<10}b = {2:<10}{3:>10}\n\n","int","float","bool","Ctrl+Cで終了");
    /*print!("\r{0:<7}{1:<7}{2:<7}{3:<7}{4:<7}{5:<7}{6:<7}{7:<7}{8:<7}{9:<7}{10:<7}{11:<7}{12:<7}{13:<7}{14:<7}{15:<7}{16:<7}\n\n",
        "s(f",
        "s(i",
        "m(f",
        "m(i",
        "24h(f",
        "24h(i",
        "12h(f",
        "12h(i",
        "PM(b",
        "D(i",
        "DW(i",
        "M(i",
        "Y(i",
        "Y0(i",
        "Y1(i",
        "Y2(i",
        "Y3(i",
    );*/

    let mut dt = Local::now();

    // 比較用
    let current_second = dt.second();

    // .000 秒まで待つ

    while dt.second() == current_second {
        dt = Local::now();
    }

    loop {
        dt = Local::now();

        let second = dt.second();

        let second_f = make_message(&config.addresses[0].to_string(), vec![OscType::Float((second as f32/60.0) as f32)]);
        let second_i = make_message(&config.addresses[1].to_string(), vec![OscType::Int((second) as i32)]);


        // minute
        let minute = dt.minute();

        let minute_f = make_message(&config.addresses[2].to_string(), vec![OscType::Float((minute as f32/60.0) as f32)]);
        let minute_i = make_message(&config.addresses[3].to_string(), vec![OscType::Int((minute) as i32)]);


        // hour24
        let hour24 = dt.hour();

        let hour24_f = make_message(&config.addresses[4].to_string(), vec![OscType::Float((hour24 as f32/24.0) as f32)]);
        let hour24_i = make_message(&config.addresses[5].to_string(), vec![OscType::Int((hour24) as i32)]);


        //  hour12 & isPM
        let hour12 = dt.hour12();

        let hour12_f = make_message(&config.addresses[6].to_string(), vec![OscType::Float((hour12.1 as f32/12.0) as f32)]);
        let hour12_i = make_message(&config.addresses[7].to_string(), vec![OscType::Int((hour24) as i32)]);

        let hour_is_pm = make_message(&config.addresses[8].to_string(), vec![OscType::Bool((hour12.0) as bool)]);


        // day
        let day = dt.day();

        let day_i = make_message(&config.addresses[9].to_string(), vec![OscType::Int((day) as i32)]);


        // dofw
        let dofw = dt.weekday();

        let dofw_i = make_message(&config.addresses[10].to_string(), vec![OscType::Int((dofw) as i32)]);


        // month
        let month = dt.month();

        let month_i = make_message(&config.addresses[11].to_string(), vec![OscType::Int((month) as i32)]);


        // year
        let year = dt.year();

        let year_i = make_message(&config.addresses[12].to_string(), vec![OscType::Int((year) as i32)]);


        // splitted year
        let year_0 = make_message(&config.addresses[13].to_string(), vec![OscType::Int((year / 1000) as i32)]);
        let year_1 = make_message(&config.addresses[14].to_string(), vec![OscType::Int(((year % 1000) / 100) as i32)]);
        let year_2 = make_message(&config.addresses[15].to_string(), vec![OscType::Int(((year % 100) / 10) as i32)]);
        let year_3 = make_message(&config.addresses[16].to_string(), vec![OscType::Int((year % 10) as i32)]);


        // モニター用
        // なんかうまく表示されないから封印

        /*print!("\r{0:<7.3}{1:<7}{2:<7.4}{3:<7}{4:<7.4}{5:<7}{6:<7.4}{7:<7}{8:<7}{9:<7}{10:<7}{11:<7}{12:<7}{13:<7}{14:<7}{15:<7}{16:<7}",
            (second as f32 /60.0), // 0
            second,
            (minute as f32 /60.0), // 2
            minute,
            (hour24 as f32 /24.0), // 4
            hour24,
            (hour12.1 as f32 /12.0), // 6
            hour12.1,
            hour12.0,
            day,
            dofw as u32,
            month,
            year,
            (year / 1000),
            ((year % 1000) / 100),
            ((year % 100) / 10),
            (year % 10)
        );*/

        send(second_f,&config.ip,config.port);
        send(second_i,&config.ip,config.port);
        send(minute_f,&config.ip,config.port);
        send(minute_i,&config.ip,config.port);
        send(hour24_f,&config.ip,config.port);
        send(hour24_i,&config.ip,config.port);
        send(hour_is_pm,&config.ip,config.port);
        send(hour12_f,&config.ip,config.port);
        send(hour12_i,&config.ip,config.port);
        send(day_i,&config.ip,config.port);
        send(dofw_i,&config.ip,config.port);
        send(month_i,&config.ip,config.port);
        send(year_i,&config.ip,config.port);
        send(year_0,&config.ip,config.port);
        send(year_1,&config.ip,config.port);
        send(year_2,&config.ip,config.port);
        send(year_3,&config.ip,config.port);

        // 1秒毎に送信
        thread::sleep(time::Duration::from_millis(config.update_rate));
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