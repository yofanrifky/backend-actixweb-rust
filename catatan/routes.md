# `src/routes.rs`

File ini mendaftarkan pasangan metode HTTP dan path, lalu menghubungkannya ke handler.

Navigasi: [indeks](README.md) · [middleware](auth-middleware.md) · [handler autentikasi](handlers/auth.md) · [handler pembayaran](handlers/payments.md) · [contoh `curl`](akses_curl.md).

> **Inti bab:** route mencocokkan metode HTTP dan URL, lalu mengarahkan request ke handler. Scope route juga menentukan middleware login.

File sumber: [routes.rs](../src/routes.rs).

`actix_web::web` menyediakan API route/scope. `from_fn` memasang middleware role pada scope. Modul handler yang digunakan adalah `auth`, `menus`, `orders`, dan `payments`.

## Fungsi `configure`

![Peta route API yang membagi endpoint menjadi auth, menus, dan orders serta menandai kebutuhan login.](visual/route-map.svg)

`pub fn configure(config: &mut web::ServiceConfig)` menerima konfigurasi service yang dapat diubah. Fungsi ini dipasang dari `main.rs` lewat `.configure(routes::configure)`.

| Metode | Path | Handler | Kegunaan |
|---|---|---|---|
| GET | `/api/menus` | `menus::get_menus` | Daftar menu; dapat meminta satu kolom dengan `?field=...`. |
| POST | `/api/menus` | `menus::create_menu` | Admin/kasir membuat menu dan stok. |
| GET | `/api/menus/{id}` | `menus::get_menu` | Detail satu menu. |
| PUT | `/api/menus/{id}` | `menus::update_menu` | Admin/kasir mengganti semua field menu/stok. |
| PATCH | `/api/menus/{id}` | `menus::patch_menu` | Admin/kasir mengganti sebagian field menu/stok. |
| DELETE | `/api/menus/{id}` | `menus::delete_menu` | Admin/kasir menghapus menu. |
| GET | `/api/orders` | `orders::get_orders` | Admin: semua; pembeli: pesanan miliknya. |
| POST | `/api/orders` | `orders::create_order` | Membeli menu dan mengurangi stok. |
| GET | `/api/orders/{id}` | `orders::get_order` | Detail; pembeli hanya dapat melihat miliknya. |
| PUT | `/api/orders/{id}` | `orders::update_order` | Mengganti seluruh field pesanan. |
| PATCH | `/api/orders/{id}` | `orders::patch_order` | Mengganti sebagian field pesanan. |
| DELETE | `/api/orders/{id}` | `orders::delete_order` | Menghapus pesanan. |
| GET | `/api/payments` | `payments::list_payments` | Daftar pembayaran admin. |
| GET | `/api/orders/{id}/receipt` | `payments::get_receipt` | Struk setelah pembayaran tercatat lunas. |
| POST | `/api/auth/register` | `auth::register` | Membuat akun; tidak perlu login. |
| POST | `/api/auth/login` | `auth::login` | Memeriksa akun dan menerbitkan JWT bearer. |
| POST | `/api/auth/logout` | `auth::logout` | Memberi tahu klien untuk membuang token. |
| GET | `/api/auth/me` | `auth::me` | Melihat akun yang sedang login; perlu login. |
| PUT | `/api/auth/users/{id}/role` | `auth::update_user_role` | Admin mengubah role akun. |

Policy route: semua role membaca katalog, admin/kasir mengubah katalog dan stok, pembeli membuat pembelian serta membuka detail miliknya, admin mengelola seluruh pesanan. Kasir tidak dapat melihat endpoint pembelian. Register dan login terbuka; register selalu menetapkan role pembeli. Logout tidak mencabut JWT stateless.

### Pembagian akses

| Route | Perlu bearer token? | Alasannya |
|---|:---:|---|
| `/api/auth/register`, `/api/auth/login` | Tidak | Pengguna harus bisa membuat akun dan memperoleh token. |
| `/api/auth/logout` | Tidak | Menginstruksikan klien menghapus token lokal. |
| `/api/auth/me` | Bearer JWT | Membaca identitas dan role token terverifikasi. |
| `/api/auth/users/{id}/role` | Admin | Mengubah role ke salah satu nilai yang diizinkan. |
| `/api/menus/*` | Bearer JWT | Semua role membaca; hanya admin/kasir mengubah menu dan stok. |
| `/api/orders/*` | Bearer JWT + role | Admin semua; buyer membuat dan melihat miliknya; kasir ditolak. |
| `/api/payments` | Admin | Melihat daftar payment yang tercatat. |
| `/api/orders/{id}/receipt` | Buyer pemilik/admin | Melihat struk pembayaran. Payment dicatat saat order dibuat. |

Tanda `*` pada tabel hanya ringkasan seluruh route di bawah prefix; bukan wildcard yang dikirim sebagai URL literal.

`web::scope("/api")` memberi prefix API; scope `/menus` dan `/orders` menambahkan prefix berikutnya. `.route("", ...)` mendaftarkan path dasar scope. `/{id}` adalah segmen dinamis, diekstrak handler sebagai `web::Path<i32>`. `web::get()`, `post()`, `put()`, `patch()`, dan `delete()` menetapkan metode; `.to(...)` menentukan fungsi tujuan.

File ini hanya melakukan routing. Validasi dan query ada di handler.

## Urutan Memakai Bearer JWT

1. Kirim `POST /api/auth/register` dengan JSON `username` dan `password` untuk membuat akun.
2. Kirim `POST /api/auth/login`; server mengembalikan bearer JWT dan role.
3. Kirim `Authorization: Bearer <token>` pada request. Role register adalah `pembeli`.
4. Admin pertama dapat dipromosikan lewat SQL tepercaya; setelah itu admin menetapkan role melalui `PUT /api/auth/users/{id}/role`.
5. Pembeli dapat membuat order, meminta pembayaran, dan mengambil struk setelah admin mengonfirmasi pembayaran manual. Kasir hanya mengelola katalog/stok.

Token tidak valid atau role berubah menghasilkan `401 Unauthorized`; token valid tanpa izin menghasilkan `403 Forbidden`. Pembayaran belum tersambung ke payment gateway.

<details>
<summary>🔎 Pendalaman: membaca konfigurasi route di Rust</summary>

Pembahasan ini menjelaskan signature, builder, dan cara scope menggabungkan path. Kamu bisa membukanya saat mulai menelusuri implementasi `routes.rs`.

## Memahami Sintaks Fungsi

Pada `pub fn configure(config: &mut web::ServiceConfig)`, `pub` berarti fungsi dapat dipanggil dari modul lain; `fn` mendeklarasikan fungsi biasa (tidak asynchronous); `config` adalah nama parameter; `&mut` berarti fungsi menerima pinjaman yang boleh diubah; `web::ServiceConfig` adalah tipe konfigurasi Actix. Fungsi tidak menulis tipe return, sehingga return-nya unit `()` secara implisit.

`crate::handlers` berarti mulai pencarian dari root crate saat ini, lalu masuk ke modul `handlers`. Bentuk `use crate::handlers::{auth, menus, orders, payments};` mengimpor submodul handler yang dipakai router sekaligus.

## Builder dan Rangkaian Pemanggilan

Pola berikut adalah method chaining:

```rust
config.service(
	web::scope("/api")
		.service(web::scope("/menus").route("", web::get().to(menus::get_menus)))
)
```

- `config.service(...)` menambahkan satu service ke konfigurasi aplikasi.
- `web::scope("/api")` membuat scope dengan prefix path.
- Pemanggilan `.service(...)` yang berikutnya memasukkan scope anak.
- `.route(path, guard)` menambahkan satu pasangan path dan aturan metode/handler.
- `web::get()` membuat konfigurasi route khusus metode GET; `.to(...)` mengubah fungsi handler menjadi tujuan route yang dapat dipanggil Actix.

Builder mengembalikan objek konfigurasi berikutnya, sehingga beberapa pemanggilan dapat dirangkai. Tanda titik di awal baris hanyalah pemformatan; method tersebut tetap bagian dari ekspresi sebelumnya.

## Scope dan Penggabungan Path

Scope bersarang menggabungkan prefix. Scope `/api` yang berisi scope `/menus`, yang berisi route `/{id}`, menghasilkan `/api/menus/{id}`. Route path `""` berarti tidak menambah segmen setelah scope, jadi menjadi `/api/menus` atau `/api/orders`. Scope `/auth` menghasilkan path seperti `/api/auth/login`.

`{id}` adalah placeholder segment pada path Actix, bukan format interpolasi Rust. Nilainya diekstrak kemudian oleh handler melalui extractor `web::Path<i32>`. Karena tipe target integer, URL seperti `/api/menus/abc` tidak dapat diekstrak sebagai `i32` dan akan ditolak oleh framework sebelum handler mendapatkan ID.

## Metode HTTP Bukan Validasi Isi

Pendaftaran `web::post()` hanya memilih metode HTTP. JSON body, validasi bisnis, query, dan response dikerjakan handler. Demikian pula route GET dengan query `?field=...` tidak didaftarkan terpisah; satu handler membaca query tersebut dan memilih kolom.

## Arti Prefix `web::`

`web` adalah modul Actix yang diimpor dari `actix_web`. Notasi `web::scope`, `web::get`, dan lainnya berarti mengakses item bernama itu di modul `web`, bukan fungsi lokal dalam file ini.

</details>

---

Kembali ke [indeks catatan](README.md) atau lanjut ke [middleware autentikasi](auth-middleware.md).
