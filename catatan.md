# Peta Catatan Kode Backend Restoran

> Catatan lengkap disusun per topik di folder [`catatan/`](catatan/README.md). Halaman ini menjadi pintu masuk untuk menemukan penjelasan yang cocok dengan file sumber.

## Gambaran alur

```text
main.rs menyalakan server
    ↓
routes.rs memilih endpoint → middleware memeriksa login dan izin
    ↓
handler membaca input → memeriksa aturan → menjalankan query
    ↓
PostgreSQL menyimpan atau mengirim data → handler membuat response JSON
```

Mulai dari [panduan belajar](catatan/README.md) untuk pengenalan dan latihan, atau gunakan peta di bawah jika sedang membaca file tertentu. Istilah dan contoh ditulis dengan bahasa sederhana; tiap halaman juga menyediakan tabel atau alur untuk membantu melihat hubungan antarbagian.

## Aplikasi dan database

| File sumber | Peran | Catatan |
|---|---|---|
| `.env` | Alamat database, alamat server, dan secret JWT lokal. | [Konfigurasi](catatan/env.md) |
| `Cargo.toml` | Daftar pustaka yang dipakai program. | [Dependency Cargo](catatan/cargo.md) |
| `src/main.rs` | Membaca konfigurasi, menyambungkan database, lalu menyalakan server. | [Startup](catatan/main.md) |
| `src/state.rs` | Menyimpan pool database dan layanan JWT untuk dipakai handler. | [State aplikasi](catatan/state.md) |
| `src/db.rs` | Membuat dan menyesuaikan tabel database. | [Skema database](catatan/db.md) |
| `src/models.rs` | Menentukan bentuk data menu dan pesanan di Rust/JSON. | [Model data](catatan/models.md) |

## Routing dan akses

| File sumber | Peran | Catatan |
|---|---|---|
| `src/routes.rs` | Menghubungkan metode dan URL dengan handler. | [Route API](catatan/routes.md) |
| `src/auth_middleware.rs` | Memeriksa token dan izin sebelum request masuk ke handler. | [Middleware autentikasi](catatan/auth-middleware.md) |
| `src/auth/mod.rs` | Mengumumkan modul layanan token, password, dan role. | [Peta modul autentikasi](catatan/auth/mod.md) |
| `src/auth/jwt.rs` | Membuat dan memeriksa token JWT. | [Layanan JWT](catatan/auth/jwt.md) |
| `src/auth/password.rs` | Mengamankan password dan memeriksa password saat login. | [Layanan password](catatan/auth/password.md) |
| `src/auth/role.rs` | Menentukan tiga role yang dikenal aplikasi. | [Tipe role](catatan/auth/role.md) |

## Handler API

| File sumber | Peran | Catatan |
|---|---|---|
| `src/handlers/mod.rs` | Mengumumkan file-file handler agar dapat dipakai router. | [Peta modul handler](catatan/handlers/mod.md) |
| `src/handlers/auth.rs` | Register, login, logout, profil akun, dan perubahan role. | [Handler autentikasi](catatan/handlers/auth.md) |
| `src/handlers/format.rs` | Mengubah angka Decimal menjadi teks Rupiah. | [Format harga](catatan/handlers/format.md) |
| `src/handlers/menus.rs` | Membaca dan mengelola menu serta stok. | [Handler menu](catatan/handlers/menus.md) |
| `src/handlers/orders.rs` | Membuat dan mengelola pesanan, termasuk mengurangi stok. | [Handler pesanan](catatan/handlers/orders.md) |
| `src/handlers/payments.rs` | Membuat permintaan pembayaran, mengonfirmasi pembayaran, dan membuat struk. | [Handler pembayaran](catatan/handlers/payments.md) |

## Praktik

- [Coba request API dengan `curl`](catatan/akses_curl.md)
- [Belajar dengan tutor langkah demi langkah](catatan/prompt-belajar-kode.md)
- [Roadmap belajar enam bulan](catatan/roadmap-belajar-6-bulan.md)
