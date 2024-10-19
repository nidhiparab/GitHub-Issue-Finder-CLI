use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    html_url: String,
}

#[derive(Deserialize, Debug)]
struct Issue{
    title: String,
    html_url: String,
    user: User
}

#[derive(Deserialize, Debug)]
struct Resp{
    items: Vec<Issue>
}

impl Resp{
    fn print_repos(&self,idx:usize)->usize{
        //print 5 results
        let size = self.items.len();
        if idx+5<=size{
        for repidx in idx..idx+5{
            println!("***************************** \n");
            println!("{}) {}\n\tUser:- \x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\\n\tLink:- {} \n", repidx+1,self.items[repidx].title,
            self.items[repidx].user.html_url,self.items[repidx].user.login, self.items[repidx].html_url);
        }
        return idx+5;
    }else{
        for repidx in idx..size{
            println!("***************************** \n");
            println!("{}) {}\n\tUser:- \x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\\n\tLink:- {} \n", repidx+1,self.items[repidx].title,
            self.items[repidx].user.html_url,self.items[repidx].user.login, self.items[repidx].html_url);
        }
        return size;
    }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let req_url = "https://api.github.com/search/issues?q=label:good-first-issue+state:open";

    let response = client
    .get(req_url)
    .header("User-Agent", "your-rust-cli")
    .send()
    .await?;
    
    let issues: Resp = response.json().await?;
    let mut stop=String::from("");
    let mut idx = 0;
    while idx!=issues.items.len() {
        idx = issues.print_repos(idx);
        println!("More Results ?\n Y Or N");
        stop.clear(); 
        std::io::stdin().read_line(&mut stop).unwrap();
        stop = stop.trim().to_string();

        if stop.eq_ignore_ascii_case("N") {
            break;
        }
    }
    
    Ok(())
}
