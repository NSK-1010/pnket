use anyhow::Result;
use std::fs::File;
use std::io;

mod tzst;

#[tokio::main]
pub async fn download(url: &str, output: &str, tzst_bool: bool) -> Result<()> {
    let filename: &str = if output.is_empty() {
        if url.ends_with("/") {
            &url[..url.rfind('/').unwrap()].split("/").last().unwrap()
        } else {
            url.split("/").last().unwrap()
        }
    } else {
        output
    };
    
    let client = reqwest::Client::new();
    let get = client.get(url)
        .send()
        .await?;
    let bytes = get
        .bytes()
        .await?;
    
    let mut out = File::create(filename)?;
    io::copy(&mut bytes.as_ref(), &mut out)?;

    if tzst_bool {
        if rmext(filename) == filename {
            if let Err(err) = tzst::tzst(filename, &format!("{}{}", filename, ".d")) {
                println!("{:?}", err);
            }
        } else {
            if let Err(err) = tzst::tzst(filename, rmext(filename)) {
                println!("{:?}", err);
            }
        }
    }

    Ok(())
}

fn rmext(filename: &str) -> &str {
    let mut filename = filename;
    while let Some(i) = filename.rfind('.') {
        if filename.ends_with(".pkg") {
            break;
        } else {
            filename = &filename[..i];
        }
    }
    return filename;
}