use ::std::io;
use std::io::Write;

#[derive(Debug)]
struct Todo {
    id: u8,
    task: String,
    is_completed: bool,
}

fn main() {
    let mut todos: Vec<Todo> = Vec::new();
    let mut id: u8 = 0;

    println!("===== Selamat datang di To Do App CLI =====");
    println!("Ketik 'add <task>' untuk tambah tugas baru");
    println!("Ketik 'del <task_id>' untuk hapus tugas");
    println!("Ketik 'list' untuk lihat daftar tugas");
    println!("Ketik 'exit' untuk keluar");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        // get user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().to_lowercase();
        let parts: Vec<&str> = input.splitn(2, ' ').collect();

        match parts.get(0) {
            Some(&"add") => {
                if let Some(tasks) = parts.get(1) {
                    let tasks: Vec<&str> = tasks.split(" ").collect();
                    if tasks.len() > 1 {
                        for task in tasks{
                            id += 1;
                            let new_task = Todo {
                                id: id,
                                task: task.to_string(),
                                is_completed: false,
                            };
                            todos.push(new_task);
                            println!("Berhasil menambahkan task '{task}'")
                        }
                    }else {
                        id += 1;
                        let task = tasks[0];
                        let new_task = Todo {
                            id: id,
                            task: task.to_string(),
                            is_completed: false,
                        };
                        todos.push(new_task);
                        println!("Berhasil menambahkan task '{task}'")

                    }


                } else {
                    println!("Tolongkan masukkan task yang ingin ditambahkan")
                }
            }

            Some(&"list") => {
                if !todos.is_empty() {
                    println!("===> Daftar Tugas <===");
                    for (
                        i,
                        Todo {
                            id,
                            task,
                            is_completed,
                        },
                    ) in todos.iter().enumerate()
                    {
                        let status = if *is_completed {
                            "Selesai"
                        } else {
                            "Belum Selesai"
                        };
                        println!("{}. [{id}] {task} [{status}]", i + 1);
                    }
                } else {
                    println!("Tugas Kosong. Tambah tugas dengan format 'add <task>'")
                }
            }

            Some(&"del") => {
                if let Some(id) = parts.get(1) {
                    match id.parse::<u8>() {
                        Ok(id) => {
                            if let Some(i) = todos.iter().position(|todo| todo.id == id) {
                                let del = todos.remove(i);
                                println!("Task '{}' telah berhasil dihapus", del.task);
                            } else {
                                println!("ID Tidak di temukan");
                            }
                        }
                        Err(_) => println!("ID tidak valid"),
                    }
                } else {
                    println!("gagal menghapus");
                    println!("{parts:?}");
                    println!("{todos:?}");
                }
            }

            Some(&"exit") => {
                println!("===== Sampai jumpa <(>_<)> =====");
                break;
            }

            Some(_) => println!("Perintah tidak valid"),
            None => print!("Tolong masukkan printah"),
        }
    }
}
