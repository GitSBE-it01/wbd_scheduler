# WBD Scheduler

program utk running scheduler WBD.
Di buat dengan menggunakan bahasa program rust.
Alasan :

- karena running di php sering error utk scheduler pick_now dan db_jig
- Rust program yang lebih efesien dan mudah utk membuat .exe files
- Program ini tentunya running di pc lokal bukan di server

Note:

- tidak bisa connect menggunakan ODBC (karena tidak ada koneksi ODBC di local PC)
- ganti password, username, ip koneksi database bisa di cek di .env file. utk setup baru bisa buat variable baru dengan setting : "mysql://username:password@ipaddress:port/nama_database"

## Perintah di command line :

Develop env :
command ini akan berulang jika ada perubahan di folder dir
`cargo watch -q -x "run"`

Running dir examples:
seperti develop testing tetapi melihat folder examples saja. digunakan utk belajar menggunakan rust
`cargo watch -q -w examples/ -x "run --example nama_file -- --nocapture"`

Build .exe files (applikasi):
utk membuat execute file
`cargo build`

running testing:
utk running di directory tests utk running testing yg di buat di situ
`cargo test --test nama_file`

##

for running unit test
cargo test function_name --lib -- --nocapture
