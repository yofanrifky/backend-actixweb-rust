# `src/auth/mod.rs`

File ini mengumumkan tiga bagian kecil yang dipakai untuk autentikasi. Bayangkan folder `auth` sebagai kotak perkakas: berkas ini memberi nama pada alat-alat yang tersedia, sementara pekerjaan masing-masing alat ada di file lain.

File sumber: [mod.rs](../../src/auth/mod.rs).

## Deklarasi modul

```rust
pub mod jwt;
pub mod password;
pub mod role;
```

| Deklarasi | Implementasi | Dipakai untuk |
|---|---|---|
| `pub mod jwt;` | `jwt.rs` | Menerbitkan dan memeriksa bearer token. |
| `pub mod password;` | `password.rs` | Membuat hash password dan memeriksa login. |
| `pub mod role;` | `role.rs` | Mengenali role `admin`, `kasir`, dan `pembeli`. |

`mod nama;` memberi tahu Rust bahwa isi modul berada di file `nama.rs` dalam folder yang sama. Kata `pub` membuat modul itu bisa dipakai modul lain. Karena itu, middleware dan handler dapat memanggil jalur seperti `crate::auth::jwt` atau `crate::auth::role`.

File ini tidak berisi pemeriksaan token atau password. Ikuti [peta autentikasi](README.md) untuk melihat hubungan antarfile, atau buka penjelasan [JWT](jwt.md), [password](password.md), dan [role](role.md).

---

Kembali ke [peta catatan kode](../README.md).
