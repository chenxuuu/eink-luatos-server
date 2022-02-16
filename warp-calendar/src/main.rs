use std::convert::Infallible;

use calendar;
use log::info;
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Deserialize, Serialize, Debug)]
struct LuatRequest {
    //设备唯一识别字符串
    mac: String,
    //电量 0-100
    battery: u8,
    //位置城市ID：https://yikeapi.com/help/tianqicity
    //如上海：101020100
    location: String,
    //appid
    appid: String, 
    //appsecret
    appsecret: String,
}

async fn eink_server(d: LuatRequest) -> Result<impl warp::Reply, Infallible> {
    Ok(calendar::get_img_vec(d.battery, d.location,d.appid,d.appsecret, false).await)
}
async fn eink_server_png(d: LuatRequest) -> Result<impl warp::Reply, Infallible> {
    Ok(calendar::get_img_vec(d.battery, d.location,d.appid,d.appsecret, true).await)
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    //http://127.0.0.1:23366/luatos-calendar/v1?mac=aaaaa&battery=10&location=12312&appid=xxx&appsecret=xxx
    let calendar_request = warp::get()
        .and(warp::path!("luatos-calendar" / "v1"))
        .and(warp::query::<LuatRequest>())
        .and_then(eink_server);
    //返回图片的接口
    let calendar_request_png = warp::get()
        .and(warp::path!("luatos-calendar" / "v1" / "png"))
        .and(warp::query::<LuatRequest>())
        .and_then(eink_server_png);

    let port = std::env::args().nth(1).unwrap_or(String::from("23366"));
    let port = port.parse::<u16>().expect(&format!("error port number {}!",port));
    info!("server start at port {} !", port);
    warp::serve(calendar_request.or(calendar_request_png))
        .run(([0,0,0,0], port))
        .await
}
