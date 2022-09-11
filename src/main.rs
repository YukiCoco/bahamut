#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //从文件写入 Cookie
    // let mut cookie = File::open("cookies.txt")?;
    // let mut cookie_contents = String::new();
    // let user_agent = "Mozilla/5.0 (X11; CrOS x86_64 14989.58.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36".to_string();
    // cookie.read_to_string(&mut cookie_contents)?;
    // let baha = BahaRequest::new(cookie_contents, user_agent)?;
    // let resp = baha
    //     .request("https://ani.gamer.com.tw/ajax/getdeviceid.php".to_string())
    //     .await?;
    // baha.get_deviceid().await?;
    // println!("{}", resp);
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

