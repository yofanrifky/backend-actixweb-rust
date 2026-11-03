# `src/auth/role.rs` — Tipe Role

File ini menetapkan role yang dikenal backend. Role seperti kartu akses: tipe yang sama dipakai oleh token, middleware, handler, dan database agar aturan akses tidak bergantung pada teks bebas.

Navigasi: [peta autentikasi](README.md) · [middleware](../auth-middleware.md) · [handler autentikasi](../handlers/auth.md).

File sumber: [role.rs](../../src/auth/role.rs).

## Nilai role

| Varian Rust | Teks yang dikirim/disimpan |
|---|---|
| `Role::Admin` | `admin` |
| `Role::Kasir` | `kasir` |
| `Role::Pembeli` | `pembeli` |

`Role` adalah `enum`, yaitu tipe yang hanya dapat berisi salah satu pilihan yang tercantum. Dengan begitu, kode yang memeriksa izin dapat membandingkan nilai yang jelas, misalnya `role == Role::Admin`.

## Fungsi dan atribut

- `#[serde(rename_all = "lowercase")]` meminta Serde membaca dan menulis nama varian sebagai teks huruf kecil, misalnya `pembeli`.
- `as_str(self)` mengubah varian menjadi teks database yang cocok. `&'static str` berarti teksnya tetap tersedia selama program berjalan.
- `from_database(value)` melakukan kebalikannya: hanya tiga teks yang dikenal yang berubah menjadi `Some(Role)`; nilai lain, seperti `owner`, menghasilkan `None`.
- `Clone`, `Copy`, `Debug`, `Eq`, `PartialEq`, `Serialize`, dan `Deserialize` mengaktifkan penyalinan, pencetakan debug, perbandingan, dan konversi JSON untuk tipe ini.

Nilai yang tidak dikenal sengaja tidak diterka sebagai salah satu role. Handler dapat menolak input itu, dan middleware dapat memperlakukannya sebagai masalah data akun.

## Hubungan dengan database dan middleware

Register menetapkan role awal `pembeli`. Handler perubahan role hanya menerima nilai yang dikenali. Middleware membandingkan role dari JWT dengan role terbaru di tabel pengguna, lalu memutuskan apakah request dapat diteruskan. Lihat [peta route](../routes.md) untuk ringkasan izin setiap kelompok endpoint.

## Tes yang tercantum di file

Test pada file memeriksa tiga nilai yang diterima dan satu nilai tak dikenal. Dokumentasi ini hanya menjelaskan maksud pemeriksaan tersebut.

---

Kembali ke [peta autentikasi](README.md) atau [indeks catatan](../README.md).
