use std::io::prelude::*;
use std::fs;
use std::net::{ TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use final_project_server::GroupeTaches;

fn main() {
    let ecouteur = TcpListener::bind("127.0.0.1:7878").unwrap();
    let groupe = GroupeTaches::new(4);

    for flux in ecouteur.incoming().take(2) {
        let flux = flux.unwrap();

        groupe.executer(|| {
            gestion_connexion(flux);
        });
    }

    println!("Arret complet");
}

fn gestion_connexion(mut flux: TcpStream) {
    let mut tampon = [0; 1024];
    flux.read(&mut tampon).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let pause = b"GET /pause HTTP/1.1\r\n";

    let (ligne_statut, file_to_read) = if tampon.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if tampon.starts_with(pause) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contenu = fs::read_to_string(file_to_read).unwrap();
    
    let reponse = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        ligne_statut,
        contenu.len(),
        contenu
    );
    flux.write(reponse.as_bytes()).unwrap();
    flux.flush().unwrap();
}