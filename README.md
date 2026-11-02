# Restoran API

REST API restoran menggunakan Rust, Actix Web, dan PostgreSQL. Backend ini tidak menyertakan frontend. Login mengembalikan JWT bearer bertanda tangan HMAC-SHA256; endpoint menu dan pesanan memerlukan token tersebut di header `Authorization`.

## Struktur Proyek

```text
backend/
|-- postman/Restoran-API.postman_collection.json
|-- src/
|   |-- handlers/
|   |   |-- auth.rs        # Register, login, logout, dan profil
|   |   |-- menus.rs       # Operasi menu
|   |   |-- orders.rs      # Operasi pesanan
|   |   |-- format.rs      # Helper format harga
|   |   `-- mod.rs         # Daftar modul handler
|   |-- auth/             # JWT, role, dan password hashing
|   |-- auth_middleware.rs # Autentikasi dan otorisasi role
|   |-- db.rs              # Persiapan tabel database
|   |-- main.rs            # Titik mulai aplikasi
|   |-- models.rs          # Bentuk data menu dan pesanan
|   |-- routes.rs          # Pendaftaran endpoint
|   `-- state.rs           # Pool database untuk handler
|-- catatan/               # Panduan belajar per file
|-- .env                   # Konfigurasi lokal, jangan commit
`-- Cargo.toml             # Dependency Rust
```

## Persiapan dan Menjalankan

1. Pastikan Rust/Cargo dan PostgreSQL sudah terpasang. Buat database kosong yang akan dipakai aplikasi.
2. Buat file `.env` pada folder backend. Isi `DATABASE_URL`, misalnya `postgres://USER:PASSWORD@HOST:PORT/NAMA_DATABASE`.
3. Tambahkan `JWT_SECRET` dari sumber acak minimal 32 byte. Di Linux/macOS, buat nilai heksadesimal dengan `openssl rand -hex 32`. Jangan commit `.env` atau membagikan nilai rahasianya.
4. Jalankan aplikasi dari folder backend:

```sh
cargo run
```

Server default berjalan di `http://127.0.0.1:8080`. Saat startup, aplikasi menyiapkan tabel dan migrasi kolom `role`, `user_id`, serta `stock`. Akun baru selalu mendapat role `pembeli`; role tidak dapat ditentukan lewat endpoint register.

`cargo check` hanya memeriksa kompilasi; perintah itu tidak menguji koneksi PostgreSQL. `cargo run` memerlukan database aktif dan nilai `.env` yang benar.

## Alur Login

1. Daftarkan akun melalui `POST /api/auth/register` dengan JSON `username` dan `password`.
2. Login melalui `POST /api/auth/login` menggunakan kredensial tadi.
3. Salin `access_token` dari response login dan kirim pada request selanjutnya sebagai `Authorization: Bearer <token>`.
4. Akses endpoint menu, pesanan, atau `GET /api/auth/me`.
5. Logout melalui `POST /api/auth/logout`, lalu hapus token dari penyimpanan klien. Token stateless yang sudah diterbitkan tetap berlaku sampai kedaluwarsa (1 jam).

Password baru di-hash memakai Argon2id. Password plaintext dari akun lama akan di-upgrade menjadi hash saat login berhasil. JWT memakai algoritma HS256, secret minimal 32 byte, dan masa berlaku 1 jam. Endpoint register tetap terbuka untuk siapa pun yang dapat mengakses server.

## Role dan Hak Akses

| Role | Katalog/stok | Pembelian |
|---|---|---|
| `admin` | Membaca dan mengubah semua menu/stok | Melihat, mengubah, dan menghapus semua pesanan; dapat membeli |
| `kasir` | Membaca katalog, membuat/mengubah/menghapus barang dan stok | Tidak dapat mengakses endpoint pesanan |
| `pembeli` | Membaca katalog untuk memilih barang | Membuat pesanan dan melihat detail pesanan miliknya saja; tidak bisa melihat daftar pembelian semua pengguna |

JWT memuat role, tetapi middleware juga membandingkannya dengan role terbaru di database pada setiap request. Perubahan role langsung membuat token lama ditolak; pengguna harus login kembali. Untuk menentukan admin pertama, jalankan SQL melalui akun database tepercaya setelah mendaftarkan akun:

```sql
UPDATE "user" SET role = 'admin' WHERE username = 'username-admin-anda';
```

Setelah itu login ulang. Admin dapat mengatur role melalui `PUT /api/auth/users/{id}/role` dengan body `{"role":"admin"}`, `{"role":"kasir"}`, atau `{"role":"pembeli"}`. Jangan pernah menerima role dari input register publik.

## Endpoint

| Metode | URL | Kegunaan | Login |
|---|---|---|---|
| POST | `/api/auth/register` | Membuat akun role `pembeli` | Tidak |
| POST | `/api/auth/login` | Memeriksa password dan menerbitkan JWT | Tidak |
| POST | `/api/auth/logout` | Memberi tahu klien untuk membuang token | Tidak wajib |
| GET | `/api/auth/me` | Melihat akun dan role aktif | Semua role |
| PUT | `/api/auth/users/{id}/role` | Mengubah role akun | Admin |
| GET | `/api/menus` | Daftar katalog/stok atau satu kolom | Semua role |
| GET | `/api/menus/{id}` | Detail menu dan stok | Semua role |
| POST | `/api/menus` | Membuat menu | Admin/kasir |
| PUT | `/api/menus/{id}` | Mengganti seluruh field termasuk stok | Admin/kasir |
| PATCH | `/api/menus/{id}` | Mengganti sebagian field termasuk stok | Admin/kasir |
| DELETE | `/api/menus/{id}` | Menghapus menu | Admin/kasir |
| GET | `/api/orders` | Daftar semua (admin) atau milik sendiri (pembeli) | Admin/pembeli |
| GET | `/api/orders/{id}` | Detail pesanan; pembeli hanya miliknya | Admin/pembeli |
| POST | `/api/orders` | Membeli menu dan mengurangi stok | Admin/pembeli |
| PUT | `/api/orders/{id}` | Mengganti seluruh field pesanan | Admin |
| PATCH | `/api/orders/{id}` | Mengganti sebagian field pesanan | Admin |
| DELETE | `/api/orders/{id}` | Menghapus pesanan | Admin |
| POST | `/api/orders/{id}/payment` | Membuat permintaan pembayaran `pending` | Admin/pemilik |
| GET | `/api/payments` | Daftar seluruh pembayaran untuk verifikasi | Admin |
| POST | `/api/payments/{id}/confirm` | Konfirmasi pembayaran setelah verifikasi manual | Admin |
| GET | `/api/orders/{id}/receipt` | Mendapat struk untuk pembayaran lunas | Admin/pemilik |

Request terlindungi tanpa bearer token, dengan token rusak, atau token kedaluwarsa akan ditolak dengan `401 Unauthorized`.

## Contoh Data

Body untuk membuat menu:

```json
{
  "name": "Nasi Goreng",
  "price": 25000,
  "category": "Makanan",
  "stock": 20
}
```

Harga disimpan dalam tipe PostgreSQL `NUMERIC(12,2)`, bukan floating-point. Angka `25000` akan ditampilkan sebagai `Rp25.000` oleh endpoint daftar/detail tertentu.

Body untuk membuat pesanan:

```json
{
  "customer_name": "Budi",
  "menu_id": 1,
  "quantity": 2
}
```

Status awal pesanan ditetapkan server sebagai `pending`. Pembelian hanya berhasil bila stok cukup; pengurangan stok dan pencatatan pesanan berada dalam satu transaksi. Pembayaran saat ini manual: buyer mengirim metode `cash`/`bank_transfer` dan status selalu `pending`; admin hanya boleh mengonfirmasi setelah memverifikasi pembayaran di luar sistem. Struk baru tersedia setelah konfirmasi. Ini bukan payment gateway dan belum memproses uang. Untuk pembayaran online, integrasikan provider dan verifikasi signature callback/webhook sebelum menandai pembayaran lunas.

`PUT` memerlukan semua field input. `PATCH` hanya memerlukan field yang ingin diubah, contohnya:

```json
{
  "status": "diproses"
}
```

Pada `GET /api/menus`, parameter `field` yang didukung adalah `id`, `name`, `price`, `category`, dan `stock`. Pada `GET /api/orders`, field-nya `id`, `customer_name`, `item_name`, `quantity`, dan `status`; endpoint daftar pesanan hanya untuk admin.

## Postman

Impor `postman/Restoran-API.postman_collection.json`, jalankan server, lalu register dan login. Login otomatis menyimpan bearer token. Role register adalah `pembeli`; untuk mencoba admin/kasir, gunakan endpoint admin setelah admin pertama dipromosikan lewat SQL. Pembelian, permintaan pembayaran, konfirmasi admin, dan struk tersedia dengan verifikasi pembayaran manual.

Panduan menguji API dari terminal menggunakan `curl` ada di [catatan/akses_curl.md](catatan/akses_curl.md). Panduan pemula yang membahas implementasi per file ada di [catatan/README.md](catatan/README.md).# backend-actixweb-rust
