
use std::io::{self, Write};
use crate::cache::Cache;

pub fn handle_input(cache: &Cache) {
    loop {
        print_prompt();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();


        let parts: Vec<&str> = input.splitn(4, ' ').collect();
        if parts.is_empty() {
            continue;
        }


        match parts[0] {
            "set" if parts.len() == 4 => {
                let cluster = parts[1].to_string();
                let key = parts[2].to_string();
                let value = parts[3].as_bytes().to_vec();
                cache.set(cluster.clone(), key.clone(), value);
                println!("Set [{}] {} = {}", cluster, key, parts[3]);
            }
            "get" if parts.len() == 3 => {
                let cluster = parts[1];
                let key = parts[2];
                match cache.get(cluster, key) {
                    Some(value) => println!("{} = {:?}", key, String::from_utf8_lossy(&value)),
                    None => println!("{} not found in cluster [{}]", key, cluster),
                }
            }
            "delete" if parts.len() == 3 => {
                let cluster = parts[1];
                let key = parts[2];
                cache.delete(cluster, key);
                println!("Deleted {} from cluster [{}]", key, cluster);
            }
            "clear_cluster" if parts.len() == 2 => {
                let cluster = parts[1];
                cache.clear_cluster(cluster);
                println!("Cleared cluster [{}]", cluster);
            }
            "clear_all" => {
                cache.clear_all();
                println!("Cleared all clusters");
            }
            "get_clusters" =>{
                let clusters = cache.get_all_clusters();
                let port = cache.get_default_port();
                print!("clusters on port {:?} are:{:?}",port,clusters);
            }
            "port" =>{
                let port = cache.get_default_port();
                print!("port is :{:?}",port);
            }
            "help" =>{
                println!("for set use => set [cluster name] [key] [value]");
                println!("for get use => get [cluster name] [key]");
                println!("for delete use => delete [cluster name] [key]");
                println!("for clear cluster => clear_cluster [cluster name]");
                println!("for clear all => clear_all");
                println!("for get clusters name => get_clusters");
                println!("see port that app has running =>  port");
                println!("for kill process => exit");
            }
            "exit" => break,
            _ => println!("Invalid command. Use pars_cache help to see help"),
        }
    }
}

fn print_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}
