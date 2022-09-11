use bahamut::libs::baha::baha;

use async_recursion::async_recursion;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use regex::Regex;
use reqwest::{cookie::Jar, ClientBuilder, Url};
use serde::Deserialize;
use serde_json::Value;
use std::fmt::format;
use std::fs::File;
use std::io::{Error, Read};
use std::{collections::HashMap, sync::Arc};
use std::{path, thread, time};
#[test]
fn rand_test() {
    // println!("{}", baha::BahaRequest::rand_str());
    // assert_eq!(BahaRequest::rand_str().len(), 12);
}

#[test]
fn regex_test() {
    let r = regex::Regex::new(r"playlist.+").unwrap();
    let r = r.replace(r"https:\/\/bahamut.akamaized.net\/113080078d2f53e440bd36ea19a9c20845815d67\/playlist_basic.m3u8?hdnts=exp%3D1662821730%7Edata%3D354e342d4094e53%3A31124%3A0%3A1%3A6868682e%7Eacl%3D%2F113080078d2f53e440bd36ea19a9c20845815d67%2Fplaylist_basic.m3u8%21%2F113080078d2f53e440bd36ea19a9c20845815d67%2F360p%2F%2A%21%2F113080078d2f53e440bd36ea19a9c20845815d67%2F480p%2F%2A%21%2F113080078d2f53e440bd36ea19a9c20845815d67%2F540p%2F%2A%21%2F113080078d2f53e440bd36ea19a9c20845815d67%2F576p%2F%2A%21%2F113080078d2f53e440bd36ea19a9c20845815d67%2F720p%2F%2A%7Ehmac%3D54c09241c37176f6c30bcdb76e5ec2ccbf2a76450b0cbd2f07fb1dc881346e86","");
    assert_eq!(
        r,
        r"https:\/\/bahamut.akamaized.net\/113080078d2f53e440bd36ea19a9c20845815d67\/"
    );
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
        println!("{},{}", &i["resolution"], &i["m3u8"]);
        let resolution = &i["resolution"];
        let resolution_vertical = &resolution[resolution.find('x').unwrap() + 1..resolution.len()];
        println!("{}", resolution_vertical);
    }
}

#[tokio::test]
async fn ready() {
    //从文件写入 Cookie
    let mut cookie = File::open("cookie.txt").unwrap();
    let mut cookie_contents = String::new();
    let user_agent = "Mozilla/5.0 (X11; CrOS x86_64 14989.58.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36".to_string();
    cookie.read_to_string(&mut cookie_contents).unwrap();
    let baha = baha::BahaRequest::new(cookie_contents, user_agent).unwrap();
    baha.get_playlist("31142").await.unwrap();
}
