# `src/auth/jwt.rs` — Layanan Token

File ini membuat dan memeriksa JWT (*JSON Web Token*). Anggap token sebagai tiket sementara: setelah login berhasil, klien membawa tiket ini pada request berikutnya agar server dapat mengenali ID dan role akun.

Navigasi: [peta autentikasi](README.md) · [handler login](../handlers/auth.md) · [middleware](../auth-middleware.md).

File sumber: [jwt.rs](../../src/auth/jwt.rs).

## Isi token

| Klaim | Isi | Kegunaan |
|---|---|---|
| `sub` | ID pengguna yang ditulis sebagai teks. | Menunjukkan akun pemilik token. |
| `role` | `admin`, `kasir`, atau `pembeli`. | Membawa role saat token diterbitkan. |
| `iat` | Waktu token diterbitkan, dalam detik sejak 1 Januari 1970 UTC. | Catatan waktu penerbitan. |
| `exp` | Batas waktu token berlaku. | Token ditolak setelah waktu ini. |

> Isi JWT dapat dibaca oleh orang yang memegang token. Tanda tangan mencegah perubahan diam-diam; JWT ini tidak mengenkripsi isinya. Jangan masukkan password atau data rahasia ke klaim.

## Tipe yang didefinisikan

| Tipe | Penjelasan |
|---|---|
| `JwtService` | Menyimpan secret yang dipakai untuk menandatangani dan memeriksa token. |
| `AuthenticatedUser` | Hasil pemeriksaan token: `user_id` dan `role`. Middleware menyimpan nilai ini untuk dipakai handler. |
| `Claims` | Bentuk isi token. Tipe ini privat karena hanya diperlukan di dalam modul JWT. |

Secret disimpan sebagai `Arc<[u8]>`, yaitu data byte yang dapat dibagikan antar salinan service tanpa menyalin seluruh secret. `#[derive(Clone)]` memungkinkan handle layanan ini disalin saat state aplikasi dipakai beberapa worker.

## Fungsi yang tersedia

| Fungsi | Apa yang dilakukan | Hasil |
|---|---|---|
| `JwtService::new(secret)` | Menolak secret yang kurang dari 32 byte lalu membuat layanan. | `Err` jika terlalu pendek; selain itu `JwtService`. |
| `issue(user_id, role)` | Membuat klaim waktu, memilih HS256, lalu menandatangani token dengan secret. | String token atau error dari pustaka JWT. |
| `verify(token)` | Memeriksa signature, algoritma, waktu berlaku, lalu membaca ID positif dan role. | `AuthenticatedUser` atau gagal. |
| `expiration_seconds()` | Mengembalikan lama berlakunya token. | `3600` detik (satu jam). |

### Alur `issue`

1. Membaca waktu saat ini sebagai detik sejak awal kalender Unix.
2. Membuat `iat` dan `exp`; waktu kedaluwarsa satu jam setelah penerbitan.
3. Memilih algoritma tanda tangan HS256.
4. Mengubah klaim menjadi token bertanda tangan menggunakan secret.

### Alur `verify`

1. Meminta pustaka JWT hanya menerima algoritma HS256.
2. Memeriksa tanda tangan dan aturan validasi token, termasuk waktu kedaluwarsa.
3. Mengubah `sub` dari teks menjadi `i32` dan menolak ID nol atau negatif.
4. Mengembalikan ID serta role sebagai `AuthenticatedUser`.

Handler login memanggil `issue`; middleware memanggil `verify`. Middleware juga memeriksa bahwa role token masih sama dengan database. Karena itu, keberhasilan `verify` saja belum memberi izin ke setiap endpoint.

## Arti beberapa bentuk Rust

- `Result<T, E>` berarti operasi dapat berhasil dengan `T` atau gagal dengan `E`.
- `?` meneruskan error ke pemanggil fungsi dan menghentikan langkah berikutnya.
- `Arc` adalah pointer bersama agar beberapa bagian aplikasi dapat memakai data yang sama dengan aman.
- `SystemTime::now()` membaca waktu sistem; `UNIX_EPOCH` adalah titik awal perhitungan detik.
- `parse::<i32>()` mencoba mengubah teks menjadi bilangan bulat bertipe `i32`.

## Tes yang tercantum di file

Bagian `#[cfg(test)]` berisi pemeriksaan untuk token yang diterbitkan, token yang diubah, dan secret terlalu pendek. Bagian tersebut hanya dikompilasi untuk pengujian; dokumentasi ini tidak menjalankan tes.

---

Kembali ke [peta autentikasi](README.md) atau [indeks catatan](../README.md).
