# `src/handlers/mod.rs`

Navigasi: [indeks](../README.md) · [autentikasi](auth.md) · [menu](menus.md) · [pesanan](orders.md).

> **Inti bab:** deklarasi `pub mod` menghubungkan nama modul handler dengan file `.rs` yang berisi implementasinya.

File sumber: [mod.rs](../../src/handlers/mod.rs).

File ini mendaftarkan modul handler yang tersedia. Isinya deklarasi modul, bukan implementasi endpoint:

![Peta letak modul utama: router mengarahkan request ke handler, handler memakai model dan state.](../visual/arsitektur.svg)

```rust
pub mod menus;
pub mod orders;
pub mod format;
pub mod auth;
```

Setiap deklarasi `pub mod nama;` memberi tahu compiler bahwa implementasi modul berada di file `nama.rs` pada folder yang sama, dan membuat modul itu dapat diakses dari luar `handlers`. `format.rs` berisi fungsi format harga yang dipakai handler lain; `auth.rs` berisi endpoint autentikasi.

File ini tidak memuat logika endpoint. Ia mengatur struktur modul. Baca [handler autentikasi](auth.md), [format harga](format.md), [handler menu](menus.md), dan [handler pesanan](orders.md) untuk implementasinya.

---

Kembali ke [indeks catatan](../README.md).
