use std::{collections::HashMap, sync::Arc};
use std::fs::File;
use std::io::Read;
use reqwest::{cookie::Jar, Url, ClientBuilder};
use serde::{Deserialize};

#[derive(Deserialize)]
struct ResponseName {
    deviceid : String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //从文件写入 Cookie
    let mut cookie = File::open("cookies.txt")?;
    let mut cookie_contents = String::new();
    cookie.read_to_string(&mut cookie_contents)?;
    let baha = BahaRequest::new(cookie_contents)?;
    let resp = baha.request("https://ani.gamer.com.tw/ajax/getdeviceid.php".to_string()).await?;
    println!("{}", resp);
    // let resp = ClientBuilder::new();
    // let url = "https://ani.gamer.com.tw".parse::<Url>()?;
    // let jar = Jar::default();
    // jar.add_cookie_str(cookie, &url);
    // //开始请求服务器
    // let jar = Arc::new(jar);
    // let resp = resp
    // .cookie_provider(jar)
    // .build()?
    // .get("https://ani.gamer.com.tw/ajax/getdeviceid.php").send().await?
    // .json::<ResponseName>().await?;
    // println!("deviceid: {}", resp.deviceid);
    Ok(())
}

struct BahaRequest {
    cookie : Arc<Jar>
}
//TODO 1. 可修改代理
//TODO 2. 热更新 Cookies

impl BahaRequest {
    fn new(cookie: String) -> Result<BahaRequest,Box<dyn std::error::Error>> {
        let url = "https://ani.gamer.com.tw".parse::<Url>()?;
        let jar = Jar::default();
        jar.add_cookie_str(cookie.as_str(), &url);
        let jar = Arc::new(jar);
        Ok(BahaRequest { cookie: jar  })
    }

    async fn request(&self, url : String) -> Result<String,Box<dyn std::error::Error>> {
        let resp = ClientBuilder::new();
        let resp = resp
        .cookie_provider(self.cookie.clone())
        .build()?
        .get("https://ani.gamer.com.tw/ajax/getdeviceid.php").send().await?
        .text().await?;
        Ok(resp)
    }
}