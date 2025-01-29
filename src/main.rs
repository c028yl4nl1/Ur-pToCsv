use bloom::{BloomFilter, ASMS};
use std::{
    collections::HashSet,
    env::args,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::exit,
    str::FromStr,
};
use url_parse::core::Parser;
use url_parse::url::Url;
use walkdir::WalkDir;

const FILENAME_SAVE: &str = "bigcombo.csv";
const NUM_PARTITIONS: usize = 3; // Dividir inicialmente em 300 filtros menores
const ENTRIES_PER_PARTITION: u32 = i32::MAX as u32; // Máximo de entradas por filtro

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut filepath = PathBuf::new();

    if let Some(valor) = args().nth(1) {
        let path = PathBuf::from_str(&valor)?;
        if !path.exists() {
            eprintln!("Pasta não existe");
            exit(1);
        }
        filepath = path;
    } else {
        eprintln!("Preciso da pasta");
        exit(1);
    }

    let mut file = salvefile();
    let _ = file.write("host,port,path,user,pass\n".as_bytes());

    // Configuração inicial dos filtros de Bloom
    let p = 0.01;
    let mut bloom_filters = Vec::new();
    let mut filter_counts = Vec::new(); // Para contar o número de entradas em cada filtro

    // Criar os filtros de Bloom iniciais
    for _ in 0..NUM_PARTITIONS {
        let filter = BloomFilter::with_rate(p, ENTRIES_PER_PARTITION);
        bloom_filters.push(filter);
        filter_counts.push(0); // Inicializar o contador de entradas para cada filtro
    }
    let mut count_Foda: isize = 0;
    let mut printA: isize = 100000;

    loop {
        for file_entry in WalkDir::new(&filepath) {
            if file_entry.is_err() {
                continue;
            }

            let path = file_entry.unwrap().into_path();
            
            if path.is_file() {
                println!("{:?}", path);
                let random_number: u32 = rand::thread_rng().gen_range(0..10000);
            let filename = format!("megaleak_{}.txt", random_number);
            salve_file(ascii_date(), &filename);
                if let Ok(hash) = remove_repetidas_open_file(&path) {
                    for config in hash {
                        let string = format!(
                            "\"{}\",{},\"{}\",\"{}\",\"{}\"\n",
                            config.host,
                            config.port, // A porta não estará entre aspas
                            config.path,
                            config.user,
                            config.pass
                        );
                        println!("{}", string);

                        let buffer = format!(
                            "{}:{}/{}|{}|{}\n",
                            config.host,
                            config.port, // A porta não estará entre aspas
                            config.path,
                            config.user,
                            config.pass
                        );
                        salve_file(buffer, &filename);

                        // Verificar se o valor já existe em algum dos filtros
                        let exists_in_any =
                            bloom_filters.iter().any(|filter| filter.contains(&string));

                        if !exists_in_any {
                            // Tentar inserir nos filtros disponíveis
                            let mut inserted = false;

                            for (index, filter) in bloom_filters.iter_mut().enumerate() {
                                if filter_counts[index] < ENTRIES_PER_PARTITION {
                                    filter.insert(&string);
                                    filter_counts[index] += 1; // Incrementar o contador de entradas
                                    inserted = true;
                                    break;
                                }
                            }

                            // Se não foi inserido, significa que todos os filtros estão saturados, então ignoramos a entrada
                            if !inserted {
                                eprintln!(
                                    "Todos os filtros estão saturados. Entrada ignorada: {}",
                                    string
                                );
                            } else {
                                // Salvar no arquivo
                                let _ = file.write(string.as_bytes());
                                count_Foda += 1;
                                if count_Foda == printA {
                                    println!("Total agora: {}", count_Foda);
                                    printA += printA;
                                }
                            }
                        }
                    }
                }
            }
            let _ = deletefilename(&path);
        }
    }
}

fn remove_repetidas_open_file(
    pathfile: &PathBuf,
) -> Result<HashSet<login<String>>, Box<dyn std::error::Error>> {
    let mut hashset = HashSet::new();
    let open_file = fs::read_to_string(pathfile)?;
    let proibido = [r#"""#, ",", "'"];

    for line in open_file.lines() {
        if let Some(login) = parse(line) {
            println!("{:?}", login);
            hashset.insert(login);
        }
    }

    Ok(hashset)
}

fn deletefilename(filename: &PathBuf) {
    fs::remove_file(filename).ok();
}

fn salvefile() -> File {
    fs::OpenOptions::new()
        .append(true)
        .create(true)
        .write(true)
        .open(FILENAME_SAVE)
        .expect("Erro ao abrir arquivo para salvar")
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct login<T: AsRef<str>> {
    host: T,
    port: u32,
    path: T,
    user: T,
    pass: T,
}

fn parse(line: &str) -> Option<login<String>> {
    if line.len() > 1000 || !line.contains("https:") || !line.contains("http") {
        return None;
    }
    //println!("{}", line);
    // let line = "https://silvercop.app/sdsddsdds/dsdsds/sdsds/dsds:caiofabiano215@gmail.com:caio0209";
    let split: Vec<&str> = line.split(":").collect();
    let len = split.len();
    let mut entry = String::new();

    let mut mail = String::new();
    let mut pass = String::new();
    if len >= 4 {
        let (line, pass_mail) = split.split_at(len - 2);
        entry.push_str(&line.join(":"));

        mail = pass_mail[0].to_string();
        pass = pass_mail[1].to_string();

      if pass.contains("[NOT_SAVED]")
    || pass.contains('"')
    || pass.contains('\'')  // Verificar aspas simples
    || pass.contains("''")   // Duas aspas simples consecutivas
    || mail.contains('"')
    || mail.contains('\'')   // Verificar aspas simples em "mail"
    || mail.contains("''")    // Duas aspas simples consecutivas em "mail"

{
    return None;
}
    } else {
        return None;
    }
    //  println!("{} , {}", entry, len);
    let result = Parser::new(None).parse(&entry);
    if result.is_ok() {
        let none_string = "".to_string();
        let url = result.unwrap();
        if let Some(ref subdomain) = url.subdomain {
            if let None = url.host_str() {
                return None;
            }
            let host = format!("{}.{}", subdomain, url.host_str().expect("Error  kk"));
            let port = url.port.unwrap_or(80);
            let path = url.path.unwrap_or(Vec::new()).join("/");

            let login = login {
                host: host,
                port: port,
                user: mail,
                path: path,
                pass: pass,
            };

            return Some(login);
        } else {
            if let None = url.host_str() {
                return None;
            }
            let host = format!("{}", url.host_str().expect("Error ao encontrar o host"));
            let port = url.port.unwrap_or(80);
            let path = url.path.unwrap_or(Vec::new()).join("/");

            let login = login {
                host: host,
                port: port,
                user: mail,
                path: path,
                pass: pass,
            };

            return Some(login);
        }
    } else {
    }
    None
}

use rand::Rng;
fn salve_file(buffer: String, filename: &str) {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .write(true)
        .open(filename)
        .unwrap();
    file.write(format!("{}", buffer).as_bytes());
}

use chrono::prelude::*;
fn ascii_date() -> String {
    let now: DateTime<Utc> = Utc::now(); // Obtém a data e hora atual em UTC
    let date_str = now.format("%Y-%m-%d").to_string();

    let ascii = r#"

    ___  ___                 _                _    
    |  \/  |                | |              | |   
    | .  . | ___  __ _  __ _| |     ___  __ _| | __
    | |\/| |/ _ \/ _` |/ _` | |    / _ \/ _` | |/ /
    | |  | |  __/ (_| | (_| | |___|  __/ (_| |   < 
    \_|  |_/\___|\__, |\__,_\_____/\___|\__,_|_|\_\
                  __/ |                            
                 |___/                             
    
    @forclogs 
    @ossintools
    @OssintAndCheckBot < bot search login + 2TB 
    @ToolsHackerSell -  Tools 

    Free logs : https://t.me/+mPzyUmUzlQVhNWNh

    format: host:port/path|user|pass

    "#;

    format!("{}\n\n{}\n\n", ascii, date_str)
}
