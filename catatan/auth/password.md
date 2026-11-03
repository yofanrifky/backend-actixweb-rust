# `src/auth/password.rs` — Pengelolaan Password

File ini membuat hash password baru dan memeriksa password yang dimasukkan saat login. Ibarat menyimpan cap khusus untuk membandingkan password: database menyimpan hasil hash, bukan password asli.

Navigasi: [peta autentikasi](README.md) · [handler register/login](../handlers/auth.md).

File sumber: [password.rs](../../src/auth/password.rs).

## Tipe hasil verifikasi

`PasswordVerification` menyampaikan dua hal kepada handler login:

| Field | Arti |
|---|---|
| `valid` | Apakah password yang dimasukkan cocok dengan data akun? |
| `upgraded_hash` | Jika password lama berhasil diperiksa, hash baru yang perlu disimpan; jika tidak perlu, nilainya kosong. |

## Fungsi

### `hash_password(password)`

1. Membuat salt acak memakai `OsRng`. Salt adalah nilai acak yang membuat hash untuk password sama tetap berbeda.
2. Menggunakan `Argon2::default()` untuk membuat hash dari byte password dan salt.
3. Mengembalikan hash sebagai teks. Jika proses gagal, mengembalikan error.

Salt ikut tersimpan di dalam teks hash sehingga tidak perlu disimpan pada kolom terpisah.

### `verify_and_upgrade(password, stored_password)`

Fungsi ini menangani dua bentuk data akun:

| Isi database | Cara pemeriksaan | Jika cocok |
|---|---|---|
| Hash yang diawali `$argon2` | Membaca hash, lalu Argon2 memeriksa password terhadap salt/parameter di hash. | `valid = true`; tidak membuat hash tambahan. |
| Password plaintext lama | Membandingkan byte dengan pemeriksaan waktu konstan. | Membuat hash Argon2 baru untuk mengganti data lama. |

Jika hash Argon2 tidak dapat dibaca atau password tidak cocok, fungsi melaporkan `valid = false`. Untuk akun lama, `upgraded_hash` hanya berisi nilai saat password plaintext cocok. Handler login lalu menyimpan hash baru setelah login berhasil.

Pemeriksaan waktu konstan (`ConstantTimeEq`) mengurangi kebocoran informasi dari lama waktu perbandingan pada jalur data lama. Password baru tetap memakai Argon2, yang sengaja membutuhkan komputasi agar tebakan massal lebih mahal.

## Arti sintaks

- `SaltString::generate(&mut OsRng)` membuat salt menggunakan sumber acak sistem operasi.
- `password.as_bytes()` mengubah teks password menjadi byte untuk fungsi kriptografi.
- `Option<String>` berarti hash baru mungkin ada (`Some`) atau tidak (`None`).
- `.then(...).transpose()?` hanya membuat hash bila perbandingan lama berhasil; jika hash gagal, error diteruskan.
- `web::block` dipakai handler karena hashing membutuhkan komputasi dan sebaiknya tidak menahan worker async.

## Tes yang tercantum di file

Bagian test memeriksa bahwa password plaintext lama dapat diperiksa lalu diubah ke hash Argon2, serta password yang salah ditolak. Dokumentasi ini tidak menjalankan tes.

---

Kembali ke [peta autentikasi](README.md) atau [indeks catatan](../README.md).
