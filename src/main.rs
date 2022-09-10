use std::fmt::format;
use std::{collections::HashMap, sync::Arc};
use std::fs::File;
use std::io::{Read, Error};
use reqwest::{cookie::Jar, Url, ClientBuilder};
use serde::{Deserialize};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use serde_json::{ Value};
use std::{thread, time};
use async_recursion::async_recursion;
use regex::Regex;


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

    async fn request_with_nocookie(&self, url : String) -> Result<String,Box<dyn std::error::Error>> {
        let resp = ClientBuilder::new();
        let resp = resp
        .build()?
        .get(url).header("origin", "https://ani.gamer.com.tw").send().await?
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

    async fn get_playlist(&self, sn : &str) -> Result<String,Box<dyn std::error::Error>> {
        let url = format!("https://ani.gamer.com.tw/ajax/m3u8.php?sn={}&device={}",sn,self.deviceid);
        let resp = self.request(url).await?;
        Ok(resp)
    }

    async fn parse_playlist(&self, playlist: String) ->  Result<(),Box<dyn std::error::Error>> {
        let playlist: Value = serde_json::from_str(playlist.as_str()).unwrap();
        let src = match playlist.get("src") {
            Some(src) => src.as_str(),
            None => {
                return Err(GenError::from(BahaError {
                    message : "从 playlist 中找不到 src".to_string(),
                    error_kind : BahaErrorKind::parse_playlist
                }));
            }
        };
        let src = src.unwrap();
        let resp = self.request_with_nocookie(src.to_string()).await?;
        let url_prefix = regex::Regex::new(r"playlist.+").unwrap();
        let url_prefix = url_prefix.replace(src, "").to_string();
        Ok(())
    }
}

type GenError = Box<dyn std::error::Error>;

#[derive(Debug,Clone)]
pub enum BahaErrorKind {
    parse_playlist
}

#[derive(Debug, Clone)]
 pub struct BahaError {
        pub message: String,
        pub error_kind: BahaErrorKind
}

impl std::fmt::Display for BahaErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.to_string())
    }
}

impl std::fmt::Display for BahaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"发生错误：{}，在{}",self.message,self.error_kind)
    }
}

impl std::error::Error for BahaError {
    fn description(&self) -> &str {
        &self.message
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

    #[test]
    fn regex_test() {
        let r = regex::Regex::new(r"playlist.+").unwrap();
        let r = r.replace(r"https:\/\/bahamut.akamaized.net\/113080078d2f53e440bd36ea19a9c20845815d67\/playlist_basic.m3u8?hdnts=exp%3D1662821730%7Edata%3D354e342d4094e53%3A31124%3A0%3A1%3A6868682e%7Eacl%3D%2F113080078d2f53e440bd36ea19a9c20845815d67%2Fplaylist_basic.m3u8%21%2F113080078d2f53e440bd36ea19a9c20845815d67%2F360p%2F%2A%21%2F113080078d2f53e440bd36ea19a9c20845815d67%2F480p%2F%2A%21%2F113080078d2f53e440bd36ea19a9c20845815d67%2F540p%2F%2A%21%2F113080078d2f53e440bd36ea19a9c20845815d67%2F576p%2F%2A%21%2F113080078d2f53e440bd36ea19a9c20845815d67%2F720p%2F%2A%7Ehmac%3D54c09241c37176f6c30bcdb76e5ec2ccbf2a76450b0cbd2f07fb1dc881346e86","");
        assert_eq!(r,r"https:\/\/bahamut.akamaized.net\/113080078d2f53e440bd36ea19a9c20845815d67\/");
        let r = regex::Regex::new(r"(?P<resolution>=\d+x\d+)\n(?P<m3u8>.*chunklist.+)").unwrap();
        let test_str = r"
#EXTM3U
#EXT-X-VERSION:3
#EXT-X-STREAM-INF:BANDWIDTH=400000,RESOLUTION=640x360
360p/hdntl=exp=1662904531~acl=%2f113080078d2f53e440bd36ea19a9c20845815d67%2fplaylist_basic.m3u8!%2f113080078d2f53e440bd36ea19a9c20845815d67%2f360p%2f*!%2f113080078d2f53e440bd36ea19a9c20845815d67%2f480p%2f*!%2f113080078d2f53e440bd36ea19a9c20845815d67%2f540p%2f*!%2f113080078d2f53e440bd36ea19a9c20845815d67%2f576p%2f*!%2f113080078d2f53e440bd36ea19a9c20845815d67%2f720p%2f*~data=hdntl,354e342d4094e53%3a31124%3a0%3a1%3a6868682e~hmac=20dbb88435afe2f9d7cbf58ac41af8d2214b16e7aed0a6625e528acb8198b8ed/chunklist_b400000.m3u8
#EXT-X-STREAM-INF:BANDWIDTH=800000,RESOLUTION=960x540
540p/hdntl=exp=1662904531~acl=%2f113080078d2f53e440bd36ea19a9c20845815d67%2fplaylist_basic.m3u8!%2f113080078d2f53e440bd36ea19a9c20845815d67%2f360p%2f*!%2f113080078d2f53e440bd36ea19a9c20845815d67%2f480p%2f*!%2f113080078d2f53e440bd36ea19a9c20845815d67%2f540p%2f*!%2f113080078d2f53e440bd36ea19a9c20845815d67%2f576p%2f*!%2f113080078d2f53e440bd36ea19a9c20845815d67%2f720p%2f*~data=hdntl,354e342d4094e53%3a31124%3a0%3a1%3a6868682e~hmac=20dbb88435afe2f9d7cbf58ac41af8d2214b16e7aed0a6625e528acb8198b8ed/chunklist_b800000.m3u8
#EXT-X-STREAM-INF:BANDWIDTH=1200000,RESOLUTION=1280x720
720p/hdntl=exp=1662904531~acl=%2f113080078d2f53e440bd36ea19a9c20845815d67%2fplaylist_basic.m3u8!%2f113080078d2f53e440bd36ea19a9c20845815d67%2f360p%2f*!%2f113080078d2f53e440bd36ea19a9c20845815d67%2f480p%2f*!%2f113080078d2f53e440bd36ea19a9c20845815d67%2f540p%2f*!%2f113080078d2f53e440bd36ea19a9c20845815d67%2f576p%2f*!%2f113080078d2f53e440bd36ea19a9c20845815d67%2f720p%2f*~data=hdntl,354e342d4094e53%3a31124%3a0%3a1%3a6868682e~hmac=20dbb88435afe2f9d7cbf58ac41af8d2214b16e7aed0a6625e528acb8198b8ed/chunklist_b1200000.m3u8
        ";
        let result = r.captures_iter(test_str);
        for i in result {
            println!("{},{}",&i["resolution"], &i["m3u8"]);
            let resolution = &i["resolution"];
            let resolution_vertical = &resolution[resolution.find('x').unwrap() + 1..resolution.len()];
            println!("{}",resolution_vertical);
        }
    }

}
    