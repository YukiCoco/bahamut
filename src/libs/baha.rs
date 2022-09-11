pub mod baha {
    use async_recursion::async_recursion;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use reqwest::header::HeaderMap;
    use reqwest::{cookie::Jar, ClientBuilder, Url};
    use serde_json::Value;
    use std::{collections::HashMap, sync::Arc};
    use std::{thread, time};

    pub struct BahaRequest {
        cookie: String,
        deviceid: String,
        user_agent: String,
        headers: HeaderMap,
    }
    //TODO 1. 可修改代理
    //TODO 2. 热更新 Cookies

    impl BahaRequest {
        fn gen_header() -> HeaderMap {
            let mut headers = HeaderMap::new();
            headers.append("origin", "https://ani.gamer.com.tw".parse().unwrap());
            headers.append(
                "accept-language",
                "zh-TW,zh;q=0.9,en-US;q=0.8,en;q=0.6".parse().unwrap(),
            );
            headers.append("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8".parse().unwrap());
            //headers.append("accept-encoding", "gzip, deflate".parse().unwrap());
            headers.append("cache-control", "max-age=0".parse().unwrap());
            headers
        }

        pub fn new(
            cookie: String,
            user_agent: String,
        ) -> Result<BahaRequest, Box<dyn std::error::Error>> {
            // let url = "https://ani.gamer.com.tw".parse::<Url>()?;
            // let jar = Jar::default();
            // for i in cookie.split(';').into_iter() {
            //     let i = i.trim();
            //     let (key, value) = i.split_once('=').unwrap();
            // }
            // jar.add_cookie_str(cookie.as_str(), &url);
            // let jar = Arc::new(jar);
            Ok(BahaRequest {
                cookie: cookie,
                deviceid: "".to_string(),
                user_agent: user_agent,
                headers: BahaRequest::gen_header(),
            })
        }

        async fn request(&self, url: String) -> Result<String, Box<dyn std::error::Error>> {
            let resp = ClientBuilder::new();
            let resp = resp
                .default_headers(self.headers.clone())
                //.cookie_provider(self.cookie.clone())
                .build()?
                .get(url)
                .header("Cookie", self.cookie.clone())
                .send()
                .await?
                .text()
                .await?;
            Ok(resp)
        }

        async fn request_with_nocookie(
            &self,
            url: String,
        ) -> Result<String, Box<dyn std::error::Error>> {
            let resp = ClientBuilder::new();
            let resp = resp
                .build()?
                .get(url)
                .header("origin", "https://ani.gamer.com.tw")
                .send()
                .await?
                .text()
                .await?;
            Ok(resp)
        }

        pub async fn get_deviceid(&mut self) -> Result<String, Box<dyn std::error::Error>> {
            let resp = self
                .request("https://ani.gamer.com.tw/ajax/getdeviceid.php".to_string())
                .await?;
            let resp : Value = serde_json::from_str(resp.as_str()).unwrap();
            let deviceid = resp.get("deviceid").unwrap().as_str().unwrap().to_string();
            self.deviceid = deviceid.clone(); //TODO: 写完优化
            Ok(deviceid)
        }

        fn rand_str() -> String {
            let mut rng = thread_rng();
            let chars: String = (0..12).map(|_| rng.sample(Alphanumeric) as char).collect();
            chars
        }

        pub async fn gain_access(&self, sn: &str) -> Result<String, Box<dyn std::error::Error>> {
            let url = format!(
                "https://ani.gamer.com.tw/ajax/token.php?adID=0&sn={}&device={}&hash={}",
                sn,
                self.deviceid,
                Self::rand_str()
            );
            let resp = self.request(url).await?;
            Ok(resp)
        }

        pub async fn unlock(&self, sn: &str) -> Result<String, Box<dyn std::error::Error>> {
            let url = format!("https://ani.gamer.com.tw/ajax/unlock.php?sn={}&ttl=0", sn);
            let resp = self.request(url).await?;
            Ok(resp)
        }

        pub async fn check_lock(&self, sn: &str) -> Result<String, Box<dyn std::error::Error>> {
            let url = format!(
                "https://ani.gamer.com.tw/ajax/checklock.php?device={}&sn={}",
                self.deviceid, sn
            );
            let resp = self.request(url).await?;
            Ok(resp)
        }

        pub async fn start_ad(&self, sn: &str) -> Result<String, Box<dyn std::error::Error>> {
            let url = format!(
                "https://ani.gamer.com.tw/ajax/videoCastcishu.php?sn={}&s=194699",
                sn
            );
            let resp = self.request(url).await?;
            Ok(resp)
        }

        pub async fn skip_ad(&self, sn: &str) -> Result<String, Box<dyn std::error::Error>> {
            let url = format!(
                "https://ani.gamer.com.tw/ajax/videoCastcishu.php?sn={}&s=194699&ad=end",
                sn
            );
            let resp = self.request(url).await?;
            Ok(resp)
        }

        pub async fn video_start(&self, sn: &str) -> Result<String, Box<dyn std::error::Error>> {
            let url = format!("https://ani.gamer.com.tw/ajax/videoStart.php?sn={}", sn);
            let resp = self.request(url).await?;
            Ok(resp)
        }

        #[async_recursion]
        pub async fn check_no_ad(&self, sn: &str) -> Result<String, Box<dyn std::error::Error>> {
            let url = format!(
                "https://ani.gamer.com.tw/ajax/token.php?adID=0&sn={}&device={}&hash={}",
                sn,
                self.deviceid,
                Self::rand_str()
            );
            let resp = self.request(url).await?;
            let resp_json: Value = serde_json::from_str(resp.as_str())?;
            match resp_json.get("time") {
                Some(t) => {
                    //if(t )
                    let t = t.as_i64().unwrap();
                    if t != 1 {
                        //广告还没去除
                        thread::sleep(time::Duration::from_secs(2));
                        self.skip_ad(sn).await?;
                        self.video_start(sn).await?;
                        self.check_no_ad(sn).await?;
                    } else {
                        //通过广告检查
                    }
                }
                _ => (),
            }
            Ok(resp)
        }

        pub async fn get_playlist(&self, sn: &str) -> Result<String, Box<dyn std::error::Error>> {
            let url = format!(
                "https://ani.gamer.com.tw/ajax/m3u8.php?sn={}&device={}",
                sn, self.deviceid
            );
            let resp = self.request(url).await?;
            Ok(resp)
        }

        pub async fn parse_playlist(
            &self,
            playlist: String,
        ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
            let playlist: Value = serde_json::from_str(playlist.as_str()).unwrap();
            let src = match playlist.get("src") {
                Some(src) => src.as_str(),
                None => {
                    return Err(GenError::from(BahaError {
                        message: "从 playlist 中找不到 src".to_string(),
                        error_kind: BahaErrorKind::parse_playlist,
                        detail: playlist.to_string(),
                    }));
                }
            };
            let src = src.unwrap();
            let resp = self.request_with_nocookie(src.to_string()).await?;
            let url_prefix = regex::Regex::new(r"playlist.+").unwrap();
            let url_prefix = url_prefix.replace(src, "").to_string();
            let mut source: HashMap<String, String> = HashMap::new();
            let r =
                regex::Regex::new(r"(?P<resolution>=\d+x\d+)\n(?P<m3u8>.*chunklist.+)").unwrap();
            let result = r.captures_iter(resp.as_str());
            for i in result {
                // println!("{},{}",&i["resolution"], &i["m3u8"]);
                let resolution = &i["resolution"];
                let resolution_vertical =
                    &resolution[resolution.find('x').unwrap() + 1..resolution.len()]; //获取高度作为清晰度 比如 720P
                let create_url = |prefix: &String, m3u8: &str| (*prefix).clone() + m3u8; //m3u8 的长度
                                                                                         //let url_prefix = &url_prefix;
                source.insert(
                    resolution_vertical.to_string(),
                    create_url(&url_prefix, &i["m3u8"]),
                );
            }
            Ok(source)
        }
    }

    type GenError = Box<dyn std::error::Error>;

    #[derive(Debug, Clone)]
    pub enum BahaErrorKind {
        parse_playlist,
    }

    #[derive(Debug, Clone)]
    pub struct BahaError {
        pub message: String,
        pub error_kind: BahaErrorKind,
        pub detail: String,
    }

    impl std::fmt::Display for BahaErrorKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.to_string())
        }
    }

    impl std::fmt::Display for BahaError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "发生错误：{}，在{}", self.message, self.detail)
        }
    }

    impl std::error::Error for BahaError {
        fn description(&self) -> &str {
            &self.message
        }
    }
}
