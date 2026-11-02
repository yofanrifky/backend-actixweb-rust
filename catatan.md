# Dokumentasi Kode Backend Restoran

Dokumen ini menjelaskan seluruh berkas Rust di folder `src/`: tanggung jawab modul, struktur data, fungsi, alur request, validasi, query database, dan bentuk respons. Aplikasi ini menggunakan Actix Web sebagai server HTTP, PostgreSQL sebagai penyimpanan, SQLx untuk query asynchronous, Serde untuk JSON, serta `rust_decimal::Decimal` untuk nilai uang.

## Gambaran Alur Aplikasi

1. Proses mulai di `main.rs`, membaca konfigurasi lingkungan, membuat koneksi PostgreSQL, dan memanggil inisialisasi database.
2. Actix Web membuat aplikasi server. `AppState` berisi pool koneksi database yang dibagikan ke semua handler melalui `web::Data`.
3. `routes.rs` mendaftarkan endpoint di bawah awalan `/api`, lalu meneruskan request ke handler menu atau pesanan.
4. Handler membaca parameter path/query atau JSON, memvalidasi input, menjalankan query SQL, kemudian membentuk respons HTTP dan JSON.

## `src/main.rs`

Berkas ini adalah titik masuk executable dan merangkai proses startup.

### Deklarasi modul

- `mod db;`, `mod handlers;`, `mod models;`, `mod routes;`, dan `mod state;` menyertakan berkas modul lokal ke dalam crate. `handlers` adalah modul direktori yang submodulnya dideklarasikan di `handlers/mod.rs`.

### Import

- `actix_web::{web, App, HttpServer}` menyediakan tipe untuk menyusun aplikasi, membagikan data, dan menjalankan server HTTP.
- `sqlx::postgres::PgPoolOptions` membuat pool koneksi PostgreSQL.
- `state::AppState` adalah state aplikasi yang diberikan kepada handler.
- `std::{env, time::Duration}` digunakan untuk membaca variabel lingkungan dan mengatur timeout.

### `main`

`#[actix_web::main]` mengubah fungsi `async` menjadi titik masuk dengan runtime asynchronous Actix/Tokio. Tanda tangan `async fn main() -> std::io::Result<()>` membuat startup dapat menunggu operasi database dan server; hasil akhirnya adalah hasil I/O milik Rust.

Urutan kerja:

1. `dotenvy::dotenv().ok()` mencoba membaca berkas `.env`. Hasilnya sengaja diabaikan, sehingga ketiadaan atau kegagalan membaca `.env` tidak langsung menghentikan proses; variabel lingkungan proses tetap dapat digunakan.
2. `DATABASE_URL` wajib tersedia. Jika tidak, `expect` menghentikan proses dengan pesan yang meminta pengguna menyiapkan `.env`.
3. `HOST` opsional dan default-nya `127.0.0.1`.
4. `PORT` opsional dan default-nya `8080`. Nilainya di-parse menjadi `u16`; teks nonangka atau angka di luar rentang `u16` menyebabkan proses berhenti dengan pesan validasi.
5. `PgPoolOptions::new()` membuat konfigurasi pool. `max_connections(5)` membatasi koneksi bersamaan sampai lima, sedangkan `acquire_timeout(Duration::from_secs(10))` membatasi waktu menunggu koneksi menjadi 10 detik.
6. `.connect(&database_url).await` membuka pool. Kegagalan koneksi menyebabkan `expect` menghentikan startup dengan pesan untuk memeriksa URL dan server PostgreSQL.
7. `db::initialize_database(&pool).await` membuat/menyesuaikan tabel. Kegagalan juga menghentikan startup.
8. `HttpServer::new(move || { ... })` membangun aplikasi Actix. Closure `move` mengambil kepemilikan pool; setiap worker memperoleh `pool.clone()` (clone handle pool, bukan menyalin seluruh database), lalu membungkus `AppState` dengan `web::Data` dan memasang konfigurasi rute.
9. `.bind((host, port))?` mengikat server ke alamat tersebut dan meneruskan error I/O jika gagal. `.run().await` menjalankan server sampai berhenti.

## `src/db.rs`

Berkas ini menyiapkan skema database. Ia mengimpor `sqlx::PgPool`, yaitu handle pool koneksi PostgreSQL.

### `initialize_database(pool: &PgPool) -> Result<(), sqlx::Error>`

Fungsi asynchronous ini menerima referensi pool, mengembalikan `Ok(())` jika seluruh perintah berhasil, atau meneruskan `sqlx::Error` pertama yang terjadi melalui operator `?`.

Array `statements` berisi dua perintah `CREATE TABLE IF NOT EXISTS`:

- Tabel `menus`: `id SERIAL PRIMARY KEY` adalah ID integer yang dibuat PostgreSQL; `name TEXT NOT NULL` adalah nama menu wajib; `price NUMERIC(12, 2) NOT NULL` adalah harga presisi desimal dengan total hingga 12 digit dan 2 angka pecahan; `category TEXT NOT NULL` adalah kategori wajib.
- Tabel `orders`: `id SERIAL PRIMARY KEY`; `customer_name TEXT NOT NULL` untuk nama pelanggan; `item_name TEXT NOT NULL` untuk nama menu yang dipesan; `quantity INT NOT NULL` untuk jumlah; dan `status TEXT NOT NULL` untuk status pesanan.

Perulangan `for statement in statements` menjalankan tiap DDL menggunakan `sqlx::query(statement).execute(pool).await?`. `IF NOT EXISTS` membuat pembuatan tabel aman untuk startup berulang selama tabel sudah ada.

Setelah itu, perintah `ALTER TABLE menus ALTER COLUMN price TYPE NUMERIC(12, 2) USING price::NUMERIC(12, 2)` memastikan tipe harga menu sesuai presisi yang diharapkan dan mengonversi nilai lama menggunakan cast PostgreSQL. Perintah ini dijalankan setiap startup; jika data lama tidak dapat dikonversi, fungsi mengembalikan error.

## `src/models.rs`

Berkas ini mendefinisikan bentuk data yang dipakai SQLx dan/atau JSON. Import `Decimal` menangani uang tanpa representasi floating-point biner; `Deserialize` memungkinkan JSON masuk menjadi struct; `Serialize` mengubah struct menjadi JSON; `FromRow` memungkinkan SQLx memetakan kolom hasil query ke struct.

### `MenuItem`

`#[derive(Debug, Serialize, FromRow)]` membuat representasi debug, serialisasi JSON, dan pemetaan row SQL. Field publiknya: `id: i32` sebagai ID; `name: String` sebagai nama; `price: Decimal` sebagai harga; `category: String` sebagai kategori. Dipakai untuk hasil query satu menu, serta response create/update menu.

### `MenuInput`

`#[derive(Debug, Deserialize)]` menerima JSON untuk membuat atau mengganti menu secara penuh. `name: String`, `price: Decimal`, dan `category: String` semuanya wajib ada dalam JSON dan diverifikasi lagi oleh handler. Dipakai oleh `POST /api/menus` dan `PUT /api/menus/{id}`.

### `MenuPatchInput`

`#[derive(Debug, Deserialize)]` menerima perubahan sebagian. `name: Option<String>`, `price: Option<Decimal>`, dan `category: Option<String>` bersifat opsional; `None` berarti kolom tersebut tidak diminta untuk diubah. Handler menolak payload yang seluruh nilainya `None`.

### `Order`

`#[derive(Debug, Serialize, FromRow)]` merupakan model hasil query dan response JSON. Field: `id: i32`; `customer_name: String`; `item_name: String`; `quantity: i32`; `status: String`.

### `OrderInput`

`#[derive(Debug, Deserialize)]` menerima JSON lengkap untuk pembuatan atau penggantian pesanan. Field wajib: `customer_name: String`, `item_name: String`, `quantity: i32`, dan `status: String`. Validasi semantik dilakukan handler.

### `OrderPatchInput`

`#[derive(Debug, Deserialize)]` menerima perubahan sebagian pada pesanan. Keempat field (`customer_name`, `item_name`, `quantity`, `status`) bertipe `Option` masing-masing dengan tipe nilai yang sama seperti `OrderInput`; field `None` tidak mengubah kolom terkait.

## `src/state.rs`

Mengimpor `sqlx::PgPool` dan mendefinisikan `AppState`, state bersama aplikasi.

- `pub struct AppState { pub db: PgPool }`: satu-satunya field, `db`, adalah pool koneksi PostgreSQL yang dapat diakses handler. `pub` pada struct dan field memungkinkan modul lain membuat dan membaca state tersebut. Actix membagikannya memakai `web::Data<AppState>`.

## `src/routes.rs`

Import `actix_web::web` menyediakan API konfigurasi service, sedangkan `crate::handlers::{menus, orders}` mengambil fungsi endpoint.

### `configure(config: &mut web::ServiceConfig)`

Fungsi ini dipanggil dari `main.rs` melalui `.configure(routes::configure)`. Ia mendaftarkan satu scope induk `/api`, dengan dua scope anak:

- `/api/menus`: `GET` pada path dasar menuju `menus::get_menus`; `POST` menuju `menus::create_menu`; `GET /{id}` menuju `get_menu`; `PUT /{id}` menuju `update_menu`; `PATCH /{id}` menuju `patch_menu`; `DELETE /{id}` menuju `delete_menu`.
- `/api/orders`: `GET` pada path dasar menuju `orders::get_orders`; `POST` menuju `orders::create_order`; `GET /{id}` menuju `get_order`; `PUT /{id}` menuju `update_order`; `PATCH /{id}` menuju `patch_order`; `DELETE /{id}` menuju `delete_order`.

`web::get()`, `web::post()`, dan seterusnya menetapkan metode HTTP. `.to(...)` menghubungkan route ke fungsi handler. `{id}` adalah segmen dinamis yang dibaca sebagai integer oleh `web::Path<i32>`.

## `src/handlers/mod.rs`

Dua deklarasi `pub mod menus;` dan `pub mod orders;` menjadikan modul handler menu dan pesanan tersedia bagi modul lain, khususnya `routes.rs`. Implementasi masing-masing berada di `handlers/menus.rs` dan `handlers/orders.rs`.

## `src/handlers/menus.rs`

Import `web`, `HttpResponse`, dan `Responder` dari Actix menangani ekstraksi input dan respons. `Decimal` dipakai untuk harga; `HashMap<String, String>` membaca query string. Model yang dipakai ialah `MenuInput`, `MenuItem`, dan `MenuPatchInput`, sedangkan `AppState` menyediakan pool database.

### `get_menus(state, query) -> impl Responder`

Menangani `GET /api/menus`. `state` adalah state aplikasi Actix; `query` menampung query parameter sebagai pasangan string. Handler memeriksa parameter `field`:

- Tanpa `field`: menjalankan `SELECT id, name, price, category FROM menus ORDER BY id ASC`, memetakan hasil menjadi tuple `(i32, String, Decimal, String)`, lalu mengubah tiap row menjadi array JSON `[id, name, harga_format_rupiah, category]`. Urutannya ID menaik.
- `field=id`: hanya mengambil kolom ID dengan urutan ID menaik, lalu mengembalikan array JSON angka.
- `field=name`: hanya mengambil nama, dengan urutan yang sama, sebagai array JSON string.
- `field=price`: hanya mengambil harga desimal dan mengubah setiap nilai ke string Rupiah memakai `format_rupiah`, lalu mengembalikan array string.
- `field=category`: hanya mengambil kategori sebagai array JSON string.
- Nilai lain: mengembalikan `400 Bad Request` dengan JSON error yang menyebut pilihan `id`, `name`, `price`, atau `category`.

Query SQL untuk setiap field memilih daftar kolom eksplisit dan mengurutkan berdasarkan `id ASC`. Error query diteruskan ke `database_error` dengan pesan konteks.

### `get_menu(state, path) -> impl Responder`

Menangani `GET /api/menus/{id}`. Membaca ID dari `web::Path<i32>`, menjalankan query berparameter `WHERE id = $1` dan memetakan row opsional ke `MenuItem`. `$1` diikat melalui `.bind(...)`, bukan disisipkan sebagai teks SQL. Jika ditemukan, response `200 OK` berupa array `[id, name, harga_format_rupiah, category]`; jika tidak, `404 Not Found` dengan `{"error":"Menu tidak ditemukan"}`; jika query gagal, response internal server error.

### `create_menu(state, body) -> impl Responder`

Menangani `POST /api/menus`. JSON diekstrak sebagai `MenuInput`; sebelum query, `validate_menu` dijalankan. Query `INSERT ... RETURNING` menambahkan `name`, `price`, dan `category`, lalu meminta PostgreSQL mengembalikan row lengkap. Nilai string di-`trim()` untuk menghilangkan spasi di awal/akhir. Sukses menghasilkan `201 Created` dengan objek `MenuItem` yang diserialisasi sebagai JSON; kegagalan DB menjadi `500 Internal Server Error`.

### `update_menu(state, path, body) -> impl Responder`

Menangani `PUT /api/menus/{id}` sebagai penggantian penuh: ketiga nilai input wajib dan divalidasi seperti create. Query `UPDATE` menetapkan semua kolom yang dapat diubah dan memakai ID sebagai parameter terakhir. String di-trim. `RETURNING` mengembalikan menu yang telah diperbarui. Sukses `200 OK` dengan objek menu; ID tidak ditemukan menghasilkan `404`; error database menghasilkan `500`.

### `patch_menu(state, path, body) -> impl Responder`

Menangani `PATCH /api/menus/{id}` sebagai pengubahan parsial.

1. Jika `name`, `price`, dan `category` semuanya `None`, mengembalikan `400` karena minimal satu field harus disediakan.
2. Jika `name` atau `category` diberikan tetapi hanya berisi whitespace, mengembalikan `400`.
3. Jika harga yang diberikan kurang dari atau sama dengan nol, mengembalikan `400`.
4. Query `UPDATE` menggunakan `COALESCE($n, kolom)` untuk mempertahankan nilai lama saat parameter bernilai SQL `NULL`; setiap `Option` dibind sesuai urutan kolom. String opsional di-trim menggunakan `as_deref().map(str::trim)`.
5. `RETURNING` mengambil row baru. Sukses `200` dengan objek menu, ID tidak ditemukan `404`, dan kesalahan query `500`.

### `delete_menu(state, path) -> impl Responder`

Menangani `DELETE /api/menus/{id}`. Menjalankan `DELETE ... WHERE id = $1`, lalu memeriksa `rows_affected()`. Jika lebih dari nol, response `200 OK` dengan pesan `Menu berhasil dihapus`; jika tidak ada row terhapus, response `404` dengan error menu tidak ditemukan. Error SQL menjadi `500`.

### `validate_menu(menu: &MenuInput) -> Option<HttpResponse>`

Fungsi helper privat yang mengembalikan `Some(response 400)` jika nama atau kategori kosong setelah `trim()`, atau harga kurang dari/sama dengan nol. Pesan error masing-masing menjelaskan field wajib atau syarat harga lebih besar dari nol. Jika lolos, hasilnya `None`. Dipakai oleh create dan update penuh; patch memiliki validasi tersendiri karena field-nya opsional.

### `format_rupiah(price: Decimal) -> String`

Helper privat untuk menyajikan harga. `normalize().to_string()` menormalkan representasi desimal (misalnya menghilangkan nol pecahan yang tidak perlu), lalu `split_once('.')` memisahkan bagian integer dan pecahan. Bagian integer dibaca terbalik; setiap tiga digit, kecuali sebelum digit pertama, disisipi titik; hasilnya dibalik kembali. Jika tidak ada pecahan, fungsi menghasilkan `Rp` diikuti integer berkelompok. Jika ada pecahan, fungsi menambahkan koma dan digit pecahan. Contoh: `25000` menjadi `Rp25.000`, `1250.5` menjadi `Rp1.250,5`. Fungsi ini memformat string tampilan, bukan mengubah nilai yang disimpan di DB.

### `database_error(message: &str, error: sqlx::Error) -> HttpResponse`

Helper privat error database. Mencetak pesan konteks dan detail error SQLx ke standard error menggunakan `eprintln!`, tetapi hanya mengirim pesan konteks yang aman/sederhana kepada klien sebagai JSON `{"error": message}` dengan status `500 Internal Server Error`. Detail teknis tidak dikirim dalam response.

### Tes unit pada modul menu

Modul `#[cfg(test)]` hanya dikompilasi untuk tes. `formats_rupiah_with_thousand_separators` memastikan nilai `Decimal::new(2500000, 2)` menghasilkan `Rp25.000` dan `Decimal::new(125050, 2)` menghasilkan `Rp1.250,5`. Kedua kasus memeriksa pemisah ribuan dan pemformatan pecahan.

## `src/handlers/orders.rs`

Import Actix menangani ekstraksi request dan response; `HashMap` membaca query parameter; model yang dipakai `Order`, `OrderInput`, dan `OrderPatchInput`; `AppState` menyediakan pool.

### `get_orders(state, query) -> impl Responder`

Menangani `GET /api/orders`. Parameter query `field` menentukan data yang dipilih:

- Tanpa `field`: query mengambil seluruh kolom pesanan, mengurutkan `id DESC` (terbaru/ID terbesar lebih dahulu), dan mengubah setiap `Order` menjadi array JSON `[id, customer_name, item_name, quantity, status]`.
- `field=id`: hanya ID, urutan menurun.
- `field=customer_name`: hanya nama pelanggan, urutan menurun.
- `field=item_name`: hanya nama menu, urutan menurun.
- `field=quantity`: hanya jumlah integer, urutan menurun.
- `field=status`: hanya status, urutan menurun.
- Field lain: response `400 Bad Request` dengan daftar field yang valid.

Untuk pilihan field tunggal, hasil scalar dikemas sebagai JSON array. Error query dikirim ke helper `database_error`.

### `get_order(state, path) -> impl Responder`

Menangani `GET /api/orders/{id}`. Query mengambil row dengan ID yang dibind ke `$1`. Jika ditemukan, response `200` berupa array `[id, customer_name, item_name, quantity, status]`; jika tidak ditemukan, `404` dengan error `Pesanan tidak ditemukan`; jika query gagal, `500`.

### `create_order(state, body) -> impl Responder`

Menangani `POST /api/orders`. JSON dibaca sebagai `OrderInput`, kemudian diperiksa oleh `validate_order`. Query `INSERT` menyimpan nama pelanggan, nama item, kuantitas, dan status; tiga string di-`trim()`, sementara quantity dibind sebagai integer. `RETURNING` memetakan row yang baru dibuat menjadi `Order`. Sukses menghasilkan `201 Created` dengan objek JSON; kegagalan database `500`.

### `update_order(state, path, body) -> impl Responder`

Menangani `PUT /api/orders/{id}` sebagai penggantian penuh empat nilai. Input harus lolos `validate_order`. Query menetapkan seluruh kolom pesanan untuk ID yang diberikan dan mengembalikan row hasilnya. Sukses `200` dengan objek `Order`, ID tidak ditemukan `404`, dan error SQL `500`.

### `patch_order(state, path, body) -> impl Responder`

Menangani `PATCH /api/orders/{id}` sebagai perubahan sebagian.

1. Menolak payload jika keempat properti (`customer_name`, `item_name`, `quantity`, `status`) bernilai `None`.
2. Menolak nama pelanggan, nama item, atau status yang diberikan jika kosong setelah `trim()`.
3. Menolak quantity yang diberikan jika kurang dari atau sama dengan nol.
4. Query `UPDATE` memakai `COALESCE(parameter, kolom)` untuk menyimpan nilai lama pada field yang tidak dikirim. Parameter string opsional di-trim melalui `as_deref().map(str::trim)`; parameter lain dibind langsung.
5. `RETURNING` mengembalikan row terbarui. Sukses `200` dengan objek `Order`, ID tidak ditemukan `404`, dan error DB `500`.

### `delete_order(state, path) -> impl Responder`

Menangani `DELETE /api/orders/{id}`. Menghapus row berdasarkan parameter ID. Jika `rows_affected() > 0`, mengembalikan `200 OK` dan pesan `Pesanan berhasil dihapus`; jika tidak, `404` dan pesan pesanan tidak ditemukan. Kegagalan query menggunakan response `500`.

### `validate_order(order: &OrderInput) -> Option<HttpResponse>`

Helper privat validasi untuk create dan update penuh. Menolak nama pelanggan, nama menu, atau status kosong setelah `trim()` dengan response `400`; menolak quantity kurang dari/sama dengan nol dengan response `400`. Input valid menghasilkan `None`. Patch menggunakan pengecekan sendiri agar field opsional dapat diubah satu per satu.

### `database_error(message: &str, error: sqlx::Error) -> HttpResponse`

Helper privat yang mencetak detail error SQLx ke standard error dengan `eprintln!`, kemudian mengirim status `500` dan JSON hanya berisi pesan konteks. Ini mencegah detail internal database muncul di response API.

## Ringkasan Perilaku API

Semua path memakai awalan `/api`. `{id}` berarti ID integer. Endpoint daftar menerima `?field=...` untuk meminta satu kolom, dengan pilihan `menus`: `id`, `name`, `price`, `category`; pilihan `orders`: `id`, `customer_name`, `item_name`, `quantity`, `status`.

| Metode dan path | Fungsi | Perilaku sukses |
|---|---|---|
| `GET /api/menus` | `get_menus` | `200`, daftar menu berbentuk array data; dapat difilter dengan `field` |
| `POST /api/menus` | `create_menu` | `201`, objek menu |
| `GET /api/menus/{id}` | `get_menu` | `200`, data menu berbentuk array |
| `PUT /api/menus/{id}` | `update_menu` | `200`, objek menu yang diganti penuh |
| `PATCH /api/menus/{id}` | `patch_menu` | `200`, objek menu setelah perubahan sebagian |
| `DELETE /api/menus/{id}` | `delete_menu` | `200`, pesan menu dihapus |
| `GET /api/orders` | `get_orders` | `200`, daftar pesanan berbentuk array data; dapat difilter dengan `field` |
| `POST /api/orders` | `create_order` | `201`, objek pesanan |
| `GET /api/orders/{id}` | `get_order` | `200`, data pesanan berbentuk array |
| `PUT /api/orders/{id}` | `update_order` | `200`, objek pesanan yang diganti penuh |
| `PATCH /api/orders/{id}` | `patch_order` | `200`, objek pesanan setelah perubahan sebagian |
| `DELETE /api/orders/{id}` | `delete_order` | `200`, pesan pesanan dihapus |

Validasi input atau field filter yang tidak dikenal menghasilkan `400 Bad Request`; ID yang tidak ditemukan untuk operasi satu record menghasilkan `404 Not Found`; kesalahan query/database menghasilkan `500 Internal Server Error`. Bentuk JSON pada endpoint daftar/detail memakai array berurutan, sedangkan hasil create/update memakai objek hasil serialisasi model.

## Catatan Implementasi

- Semua query nilai menggunakan bind parameter SQLx (`$1`, `$2`, dan seterusnya), sehingga nilai request tidak digabungkan langsung ke teks SQL.
- Endpoint `PUT` memerlukan semua field input, sedangkan `PATCH` hanya mengubah field yang dikirim dan menolak body kosong.
- Tabel `orders` menyimpan `item_name` sebagai teks, bukan foreign key ke `menus`; integritas hubungan menu-pesanan tidak ditegakkan oleh skema yang dibuat di kode ini.
- Skema saat ini menetapkan kolom wajib sebagai `NOT NULL`; aturan nilai positif untuk harga dan quantity diperiksa oleh handler, bukan constraint `CHECK` di database.
