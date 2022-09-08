use std::fmt::format;
use std::{collections::HashMap, sync::Arc};
use std::fs::File;
use std::io::Read;
use reqwest::{cookie::Jar, Url, ClientBuilder};
use serde::{Deserialize};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use serde_json::{ Value};
use std::{thread, time};
use async_recursion::async_recursion;


#[derive(Deserialize)]
struct ResponseName {
    deviceid : String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //从文件写入 Cookie
    let mut cookie = File::open("cookies.txt")?;
    let mut cookie_contents = String::new();
    let user_agent = "Mozilla/5.0 (X11; CrOS x86_64 14989.58.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36".to_string();
    cookie.read_to_string(&mut cookie_contents)?;
    let baha = BahaRequest::new(cookie_contents,user_agent)?;
    let resp = baha.request("https://ani.gamer.com.tw/ajax/getdeviceid.php".to_string()).await?;
    baha.get_deviceid().await?;
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
    cookie : Arc<Jar>,
    deviceid : String,
    user_agent : String
}
//TODO 1. 可修改代理
//TODO 2. 热更新 Cookies

impl BahaRequest {
    fn new(cookie: String, user_agent: String) -> Result<BahaRequest,Box<dyn std::error::Error>> {
        let url = "https://ani.gamer.com.tw".parse::<Url>()?;
        let jar = Jar::default();
        jar.add_cookie_str(cookie.as_str(), &url);
        let jar = Arc::new(jar);
        Ok(BahaRequest { cookie: jar,deviceid: "".to_string(),user_agent: user_agent })
    }

    async fn request(&self, url : String) -> Result<String,Box<dyn std::error::Error>> {
        let resp = ClientBuilder::new();
        let resp = resp
        .cookie_provider(self.cookie.clone())
        .build()?
        .get(url).send().await?
        .text().await?;
        Ok(resp)
    }

    async fn get_deviceid(&self) -> Result<String,Box<dyn std::error::Error>> {
        let resp = self.request("https://ani.gamer.com.tw/ajax/getdeviceid.php".to_string()).await?;
        Ok(resp)
    }

    fn rand_str() -> String {
        let mut rng = thread_rng();
        let chars: String = (0..12).map(|_| rng.sample(Alphanumeric) as char).collect();
        chars
    }

    async fn gain_access(&self, sn : &str) -> Result<String,Box<dyn std::error::Error>> {
        let url = format!("https://ani.gamer.com.tw/ajax/token.php?adID=0&sn={}&device={}&hash={}",sn, self.deviceid, Self::rand_str());
        let resp = self.request(url).await?;
        Ok(resp)
    }

    async fn unlock(&self, sn : &str) -> Result<String,Box<dyn std::error::Error>> {
        let url = format!("https://ani.gamer.com.tw/ajax/unlock.php?sn={}&ttl=0",sn);
        let resp = self.request(url).await?;
        Ok(resp)
    }

    async fn check_lock(&self, sn : &str) -> Result<String,Box<dyn std::error::Error>> {
        let url = format!("https://ani.gamer.com.tw/ajax/checklock.php?device={}&sn={}",self.deviceid,sn);
        let resp = self.request(url).await?;
        Ok(resp)
    }

    async fn start_ad(&self, sn : &str) -> Result<String,Box<dyn std::error::Error>> {
        let url = format!("https://ani.gamer.com.tw/ajax/videoCastcishu.php?sn={}&s=194699",sn);
        let resp = self.request(url).await?;
        Ok(resp)
    }

    async fn skip_ad(&self, sn : &str) -> Result<String,Box<dyn std::error::Error>> {
        let url = format!("https://ani.gamer.com.tw/ajax/videoCastcishu.php?sn={}&s=194699&ad=end",sn);
        let resp = self.request(url).await?;
        Ok(resp)
    }

    async fn video_start(&self, sn : &str) -> Result<String,Box<dyn std::error::Error>> {
        let url = format!("https://ani.gamer.com.tw/ajax/videoStart.php?sn={}",sn);
        let resp = self.request(url).await?;
        Ok(resp)
    }

    #[async_recursion]
    async fn check_no_ad(&self, sn : &str) -> Result<String,Box<dyn std::error::Error>> {
        let url = format!("https://ani.gamer.com.tw/ajax/token.php?adID=0&sn={}&device={}&hash={}",sn, self.deviceid, Self::rand_str());
        let resp = self.request(url).await?;
        let resp_json: Value = serde_json::from_str(resp.as_str())?;
        match resp_json.get("time") {
            Some(t) => {
                //if(t )
                let t = t.as_i64().unwrap();
                if t != 1 { //广告还没去除
                    thread::sleep(time::Duration::from_secs(2));
                    self.skip_ad(sn).await?;
                    self.video_start(sn).await?;
                    self.check_no_ad(sn).await?;
                } else {
                    //通过广告检查
                }
            },
            _ => ()
        }
        Ok(resp)
    }

    async fn parse_playlist(&self, sn : &str) -> Result<String,Box<dyn std::error::Error>> {
        let url = format!("https://ani.gamer.com.tw/ajax/videoStart.php?sn={}",sn);
        let resp = self.request(url).await?;
        Ok(resp)
    }


}

#[cfg(test)]
    mod tests {
        use crate::BahaRequest;

    #[test]
        fn rand_test() {
            println!("{}",BahaRequest::rand_str());
            assert_eq!(BahaRequest::rand_str().len(),12);
        }
}
    