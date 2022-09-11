pub mod baha {
    use async_recursion::async_recursion;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use reqwest::{cookie::Jar, ClientBuilder, Url};
    use serde_json::Value;
    use std::{collections::HashMap, sync::Arc};
    use std::{thread, time};

    pub struct BahaRequest {
        cookie: Arc<Jar>,
        deviceid: String,
        user_agent: String,
    }
    //TODO 1. 可修改代理
    //TODO 2. 热更新 Cookies

    impl BahaRequest {
        pub fn new(
            cookie: String,
            user_agent: String,
        ) -> Result<BahaRequest, Box<dyn std::error::Error>> {
            let url = "https://ani.gamer.com.tw".parse::<Url>()?;
            let jar = Jar::default();
            jar.add_cookie_str(cookie.as_str(), &url);
            let jar = Arc::new(jar);
            Ok(BahaRequest {
                cookie: jar,
                deviceid: "".to_string(),
                user_agent: user_agent,
            })
        }

        async fn request(&self, url: String) -> Result<String, Box<dyn std::error::Error>> {
            let resp = ClientBuilder::new();
            let resp = resp
                .cookie_provider(self.cookie.clone())
                .build()?
                .get(url)
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

        pub async fn get_deviceid(&self) -> Result<String, Box<dyn std::error::Error>> {
            let resp = self
                .request("https://ani.gamer.com.tw/ajax/getdeviceid.php".to_string())
                .await?;
            Ok(resp)
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

        pub async fn parse_playlist(&self, playlist: String) -> Result<(), Box<dyn std::error::Error>> {
            let playlist: Value = serde_json::from_str(playlist.as_str()).unwrap();
            let src = match playlist.get("src") {
                Some(src) => src.as_str(),
                None => {
                    return Err(GenError::from(BahaError {
                        message: "从 playlist 中找不到 src".to_string(),
                        error_kind: BahaErrorKind::parse_playlist,
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
            Ok(())
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
    }

    impl std::fmt::Display for BahaErrorKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.to_string())
        }
    }

    impl std::fmt::Display for BahaError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "发生错误：{}，在{}", self.message, self.error_kind)
        }
    }

    impl std::error::Error for BahaError {
        fn description(&self) -> &str {
            &self.message
        }
    }
}
