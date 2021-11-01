#![allow(non_snake_case)]
use crate::{Context, Response};
use hyper::StatusCode;
use serde::Deserialize;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs;
use std::path::Path;

pub async fn test_handler(ctx: Context) -> String {
    format!("test called, state_thing was: {}", ctx.state.state_thing)
}

#[derive(Deserialize)]
struct SendRequest {
    name: String,
    active: bool,
}

pub async fn send_handler(mut ctx: Context) -> Response {
    let body: SendRequest = match ctx.body_json().await {
        Ok(v) => v,
        Err(e) => {
            return hyper::Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(format!("could not parse JSON: {}", e).into())
                .unwrap()
        }
    };

    Response::new(
        format!(
            "send called with name: {} and active: {}",
            body.name, body.active
        )
        .into(),
    )
}

pub async fn param_handler(ctx: Context) -> String {
    let param = match ctx.params.find("some_param") {
        Some(v) => v,
        None => "empty",
    };
    let days =  writefl(param);

    
    format!("param called, param was: {}",days)
}


fn writefl(str: &str) ->i32{
    //  let data:String =  "this is a fucking str1 ->".to_owned()+str+"\n";
    let mut return_days:i32 = 30;
      let data:String  =  str.to_owned()+"->30\n";
      if Path::new("foo.txt").exists() {
          let mut file1  =  OpenOptions::new().create(true).write(true).append(true).open("foo.txt").unwrap();
          if !check_if_exists(&str){
  
              file1.write_all(data.as_bytes()).expect("err");
          }else{
              /*let mut contents = readfl();
              let index = contents.find(&str);
              println!("Found ");*/
           return_days =   seek_and_changefl3(&str,"foo.txt");
          }
      }else{
      //    readfl().map_err(|err| println!("{:?}", err)).ok();
      let mut file1  =  OpenOptions::new().create(true).write(true).append(true).open("foo.txt").unwrap();
      file1.write_all(data.as_bytes()).expect("err");
      }
      /*let mut file  =  *///OpenOptions::new().create(true).write(true).append(true).open("foo.txt").unwrap();
     // Ok(())
     return return_days;
  }
  
  fn check_if_exists(search_str:&str)-> bool {
      let contents = fs::read_to_string("foo.txt").expect("Unable to open file");
      for  line in contents.lines() {
      if line.contains(search_str) {
          return true;
       }
      }
    false
  }
  
  fn readfl() -> String{
      let  file = File::open("foo.txt").expect("Unable to open file");
      let mut contents = String::new();
      let mut buf_reader =  BufReader::new(file);
      buf_reader.read_to_string(&mut contents).expect("Unable to open file");
     /* let splitter: Vec<&str>  =  contents.split("->").collect();
      let id =  splitter[0];
      let days = splitter[1];
      println!("contents: {} {}", id, days);
      Ok(())*/
      return contents;
  }
  
  
  fn seek_and_changefl3(searched_item: &str, file_name: &str) -> i32{
      let contents = fs::read_to_string(file_name).expect("err");
      let mut new:String = String::new();
      let mut buff:String = String::new();
      let mut return_days = 30;
  
    for  line in  contents.lines() {
     // println!(" {} ",line);
      if line.contains(searched_item){
          let days_remaining:String = line.chars().skip(line.len()-2).take(2).collect();
          let mut my_int = days_remaining.parse::<i32>().unwrap();
          my_int -=1;
          println!(" {} ",days_remaining);
          buff.push_str(line.replace(&days_remaining, &my_int.to_string()).as_str());
          buff.push_str("\n");
          return_days = my_int;
      }else{
          buff.push_str(line);
          buff.push_str("\n")
      }
    }
    new.push_str(&buff);
     // dbg!(&contents, &new);
  
     let mut file = OpenOptions::new().write(true).truncate(true).open(file_name).expect("err");
      file.write(new.as_bytes()).expect("err");
  
  //  Ok(())
    return return_days;
  }
  