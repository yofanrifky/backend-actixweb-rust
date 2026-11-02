# Roadmap Belajar Software Engineering dan UI/UX — 6 Bulan

**Periode rencana:** 30 Oktober 2026–30 April 2027 (26 minggu).  
**Proyek latihan utama:** backend API restoran yang ada di repository ini, lalu sebuah antarmuka sederhana yang menggunakannya.

Roadmap ini mengasumsikan waktu belajar di luar sekolah/PKL sekitar 30–60 menit saat tersedia, bukan kewajiban belajar setiap hari. Kalau sekolah, PKL, kesehatan, atau istirahat sedang membutuhkan waktu, geser target mingguan; jangan mengganti waktu tidur atau pemulihan. Enam fase pertama masing-masing empat minggu. Minggu 25–26 menjadi ruang integrasi, perbaikan, dan evaluasi akhir.

## Cara belajar dan evaluasi

Setiap sesi berfokus pada satu konsep kecil: arti dan manfaatnya, contoh singkat, praktik mandiri, lalu satu pertanyaan terbuka untuk memeriksa pemahaman. Jawaban dinilai dari ketepatan konsep. Materi berikutnya menyesuaikan bukti yang terlihat; menuntaskan latihan saja belum menaikkan skor.

Pakai hasil mingguan sebagai bukti, misalnya diagram alur yang bisa dijelaskan, program yang berjalan, keputusan desain beserta alasannya, temuan bug yang berhasil diperbaiki, atau hasil proyek yang ditinjau. Catat juga bagian yang masih membingungkan dan bantuan yang diperlukan. Jangan isi skor jika belum ada bukti.

### Skala pemahaman

| Skor | Arti |
|---:|---|
| 1 | Belum memahami konsep. |
| 2 | Mulai mengenali konsep. |
| 3 | Memahami konsep dasar. |
| 4 | Mampu menerapkan dengan bantuan. |
| 5 | Mampu menerapkan secara mandiri. |
| 6 | Mampu menganalisis dan menjelaskan trade-off. |
| 7 | Mampu menerapkan secara matang dalam proyek nyata. |

Skor diberikan per topik setelah memeriksa pemahaman, penjelasan ulang, penerapan, analisis/debugging, kualitas hasil, dan kemandirian yang relevan. Skor saat ini **belum dinilai** karena belum ada hasil asesmen dalam sesi ini.

### Grafik dan catatan kemajuan

Grafik baru dapat memuat titik setelah tersedia bukti asesmen. Skala berikut adalah kanvas kosong, bukan nilai awal:

```text
7 |
6 |
5 |
4 |
3 |
2 |
1 |
   +------------------------------------------------
     Baseline    M1    M2    M3    M4    M5    Akhir
```

| Tanggal/cek | Topik | Skor 1–7 atau belum dinilai | Bukti yang ditinjau | Langkah berikutnya |
|---|---|---:|---|---|
| Baseline | Alur request dan kemampuan awal | Belum dinilai | Menunggu praktik pertama | — |

Setelah ada beberapa titik pada topik yang sama, isi grafik dengan skor dari catatan ini. Jangan menebak atau mengisi nilai untuk bulan yang belum dievaluasi.

## Target bulanan

| Fase | Fokus utama | Hasil yang dapat diperiksa |
|---|---|---|
| Bulan 1 · Minggu 1–4 | Computational thinking dan fondasi Rust | Dapat mengurai masalah kecil, membaca alur request, dan menulis program kecil dengan kondisi/fungsi. |
| Bulan 2 · Minggu 5–8 | Struktur data Rust dan dasar software engineering | Dapat menjelaskan model, alur error, serta batas tanggung jawab modul dalam proyek. |
| Bulan 3 · Minggu 9–12 | HTTP, PostgreSQL, dan pola backend | Dapat menelusuri atau membuat satu alur API dari input sampai response dan query. |
| Bulan 4 · Minggu 13–16 | Konsistensi data, autentikasi, keamanan | Dapat menemukan risiko penting dan menjelaskan cara memperbaiki serta memeriksanya. |
| Bulan 5 · Minggu 17–20 | UI/UX dan dasar frontend | Memiliki wireframe dan antarmuka sederhana dengan state yang jelas dan aksesibel. |
| Bulan 6 · Minggu 21–24 | Integrasi produk | Antarmuka lokal terhubung ke alur menu/pesanan API dan menangani kondisi penting. |
| Penutup · Minggu 25–26 | Penyempurnaan dan evaluasi | Demo lokal, dokumentasi, evaluasi kemampuan, dan grafik berdasarkan bukti. |

## Rencana per minggu

Rentang berikut memakai siklus Jumat–Kamis, dimulai 30 Oktober 2026. Hari terakhir, 30 April 2027, menjadi batas penutupan dan refleksi akhir.

| Minggu | Tanggal |
|---:|---|
| 1 | 30 Okt–5 Nov |
| 2 | 6–12 Nov |
| 3 | 13–19 Nov |
| 4 | 20–26 Nov |
| 5 | 27 Nov–3 Des |
| 6 | 4–10 Des |
| 7 | 11–17 Des |
| 8 | 18–24 Des |
| 9 | 25–31 Des |
| 10 | 1–7 Jan |
| 11 | 8–14 Jan |
| 12 | 15–21 Jan |
| 13 | 22–28 Jan |
| 14 | 29 Jan–4 Feb |
| 15 | 5–11 Feb |
| 16 | 12–18 Feb |
| 17 | 19–25 Feb |
| 18 | 26 Feb–4 Mar |
| 19 | 5–11 Mar |
| 20 | 12–18 Mar |
| 21 | 19–25 Mar |
| 22 | 26 Mar–1 Apr |
| 23 | 2–8 Apr |
| 24 | 9–15 Apr |
| 25 | 16–22 Apr |
| 26 | 23–29 Apr |

### Bulan 1 — Berpikir komputasional dan fondasi Rust

| Minggu | Konsep dan praktik | Hasil yang diperiksa |
|---:|---|---|
| 1 | **Pemetaan route.** Telusuri satu request pada API restoran dan pecah alurnya menjadi bagian-bagian. Ini menjadi asesmen awal, bukan tes hafalan. | Diagram singkat satu request dari route sampai hasil; jelaskan bagian yang belum dipahami. |
| 2 | **Dekomposisi dan pseudocode.** Uraikan kebutuhan membuat menu menjadi input, aturan, langkah, dan hasil. Selipkan matematika yang relevan: harga harus lebih dari nol. | Pseudocode dan daftar kasus valid/tidak valid untuk satu operasi. |
| 3 | **Nilai, tipe data, dan ekspresi.** Baca atau tulis program kecil yang membedakan teks, bilangan bulat, desimal, dan nilai benar/salah. | Contoh kecil yang menerima data menu dan memeriksa harga positif. |
| 4 | **Kondisi dan fungsi.** Pisahkan satu keputusan dari program kecil agar tiap bagian punya tugas jelas. | Program/praktik ringkas serta penjelasan mengapa kondisi menghasilkan keputusan tersebut. Evaluasi Bulan 1. |

### Bulan 2 — Rust dan dasar rekayasa perangkat lunak

| Minggu | Konsep dan praktik | Hasil yang diperiksa |
|---:|---|---|
| 5 | **Struct dan enum.** Hubungkan bentuk data dengan model menu/pesanan yang sudah ada. | Gambar atau contoh model data; jelaskan alasan memilih tipe untuk beberapa field. |
| 6 | **Kepemilikan dan peminjaman (ownership/borrowing).** Pelajari lewat contoh kecil, lalu hubungkan ke nilai yang diteruskan antar fungsi. | Prediksi bagian kode yang bisa memakai nilai dan jelaskan alasannya. |
| 7 | **Option dan Result.** Bedakan data yang mungkin tidak ada dengan operasi yang bisa gagal. | Contoh PATCH atau query yang menunjukkan kemungkinan kosong dan gagal; jelaskan penanganannya. |
| 8 | **Modul dan batas tanggung jawab.** Ikuti pembagian route, handler, model, dan database; pertimbangkan keterbacaan/perubahan. | Peta tanggung jawab beberapa file dan satu usulan kecil untuk memudahkan pemeliharaan. Evaluasi Bulan 2. |

### Bulan 3 — HTTP, SQL, dan alur backend

| Minggu | Konsep dan praktik | Hasil yang diperiksa |
|---:|---|---|
| 9 | **HTTP request/response.** Bedakan method, path, body, status, dan response JSON menggunakan satu endpoint. | Contoh request dan response yang diberi label serta alasan statusnya. |
| 10 | **Tabel, kunci, dan relasi.** Gambarkan hubungan menu, pengguna, dan pesanan; kaitkan dengan kebutuhan aplikasi. | Sketsa tabel dan penjelasan aturan data yang ingin dijaga. |
| 11 | **CRUD dan parameter SQL.** Ikuti operasi baca/tulis serta alasan nilai query dikirim sebagai parameter. | Telusuran satu operasi dari handler ke query dan hasilnya; tunjukkan bagaimana input mencapai SQL. |
| 12 | **Validasi dan error API.** Petakan input keliru ke respons yang mudah dipahami client. | Tabel kasus validasi, status HTTP, dan pesan; evaluasi Bulan 3. |

### Bulan 4 — Keutuhan data dan keamanan

| Minggu | Konsep dan praktik | Hasil yang diperiksa |
|---:|---|---|
| 13 | **Constraint database.** Pelajari bagaimana aturan seperti NOT NULL, UNIQUE, dan foreign key mencegah data tidak konsisten. | Rancangan aturan untuk satu bagian data restoran dan alasan tiap aturan. |
| 14 | **Transaksi dan konsistensi.** Analisis operasi yang terdiri dari lebih dari satu perubahan data. | Urutan langkah untuk satu proses pesanan; tandai apa yang perlu berhasil atau dibatalkan bersama. |
| 15 | **Autentikasi dan penyimpanan password.** Tinjau alur login yang ada dan identifikasi risiko penyimpanan password teks biasa yang dicatat di README. | Catatan risiko dan rancangan perubahan yang aman; jangan memakai password sungguhan untuk latihan. |
| 16 | **Session, middleware, dan pemeriksaan regresi.** Telusuri batas akses endpoint serta susun pemeriksaan untuk alur berhasil/gagal. | Checklist skenario login, akses terlindungi, logout, dan kegagalan. Evaluasi Bulan 4. |

### Bulan 5 — UI/UX dan fondasi frontend

| Minggu | Konsep dan praktik | Hasil yang diperiksa |
|---:|---|---|
| 17 | **Pengguna dan tugas.** Tentukan siapa yang mengelola menu/pesanan dan tindakan utama yang perlu diselesaikan. | Satu user flow untuk tugas utama beserta kebutuhan informasinya. |
| 18 | **Wireframe dan hierarki visual.** Rancang layar daftar menu/pesanan sebelum memilih warna atau detail visual. | Wireframe sederhana dengan alasan urutan informasi dan aksi. |
| 19 | **HTML semantik dan aksesibilitas dasar.** Buat struktur halaman yang dapat dipahami tanpa bergantung pada tampilan visual saja. | Halaman statis yang memakai struktur dan label sesuai fungsi. |
| 20 | **CSS responsif dan keadaan antarmuka.** Tampilkan keadaan normal, kosong, memuat, dan gagal dengan jelas. | Prototype halaman pada lebar layar kecil dan besar; evaluasi Bulan 5. |

### Bulan 6 — Integrasi frontend dan backend

| Minggu | Konsep dan praktik | Hasil yang diperiksa |
|---:|---|---|
| 21 | **JavaScript dan pemanggilan API.** Hubungkan satu aksi antarmuka ke endpoint baca, lalu tampilkan hasilnya. | Daftar menu lokal dengan keadaan memuat/kosong/gagal yang dapat dijelaskan. |
| 22 | **Form dan validasi client/server.** Bandingkan pemeriksaan di antarmuka dengan aturan backend. | Form tambah atau ubah menu dengan pesan validasi yang tidak menyamarkan error server. |
| 23 | **Session dan alur pengguna.** Rancang bagaimana login, cookie, dan request terlindungi bekerja bersama di lingkungan lokal. | Alur login sampai membuka halaman terlindungi; catat hambatan integrasi. |
| 24 | **Alur pesanan dan tinjauan UX.** Sambungkan kebutuhan utama pesanan; minta satu orang mencoba prototype jika memungkinkan, tanpa membagikan kredensial atau data sensitif. | Catatan tugas yang berhasil/gagal dan perubahan desain berdasarkan temuan. Evaluasi Bulan 6. |

### Dua minggu penutup

| Minggu | Fokus | Hasil yang diperiksa |
|---:|---|---|
| 25 | Perbaiki satu atau dua masalah terpenting, rapikan README, serta ulangi pemeriksaan untuk alur utama dan kasus gagal. | Demo lokal yang dapat diikuti dari dokumentasi; daftar bug yang diperbaiki dan yang masih diketahui. |
| 26 | Presentasikan cara kerja proyek, jelaskan keputusan teknis/desain, lalu lakukan evaluasi akhir terhadap topik yang punya bukti. | Demo dan penjelasan tanpa melihat catatan untuk konsep yang diuji; isi skor, grafik, refleksi, dan rencana lanjutan. Target akhir roadmap: 30 April 2027. |

## Kebiasaan kecil yang menyertai materi utama

- **Bahasa Inggris teknis:** kumpulkan sampai lima istilah yang benar-benar muncul pada materi minggu itu; tulis arti dan satu contoh konteksnya. Tidak perlu menerjemahkan dokumentasi panjang sekaligus.
- **Matematika:** latih konsep yang langsung dipakai, seperti perbandingan, bilangan positif, desimal uang, jumlah barang, dan logika kondisi. Naikkan kompleksitas hanya jika contoh dasarnya sudah jelas.
- **Evaluasi mingguan:** catat apa yang dapat dijelaskan dan diterapkan, apa yang masih membingungkan, hasil praktik, kendala, serta apakah target berikutnya perlu digeser.
- **Evaluasi fase:** pada akhir minggu 4, 8, 12, 16, 20, 24, dan 26, tinjau hasil dan skor. Jika satu fondasi belum kuat, pecah target berikutnya menjadi bagian lebih kecil.
- **Batas proyek:** password baru di-hash memakai Argon2id dan autentikasi memakai JWT HS256, tetapi register masih terbuka dan logout tidak mencabut token server-side. Tinjau risiko deployment serta tambahkan pembatasan dan pencabutan token sesuai kebutuhan.

## Format check-in mingguan

Salin dan isi saat ingin meninjau progres. Lama belajar tidak dipakai sebagai pengganti bukti kemampuan.

```text
Minggu/topik:
Bukti praktik atau artefak:
Hal yang bisa saya jelaskan sendiri:
Bagian yang masih membingungkan:
Bug/kendala dan cara saya menanganinya:
Skor sementara (hanya jika bukti cukup):
Kondisi/kepuasan terhadap proses:
Target minggu depan atau penyesuaian:
```
