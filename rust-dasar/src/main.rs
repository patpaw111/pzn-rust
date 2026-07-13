fn main() {
    println!("Hello, world!");
    print!("Hello, world!");
}

#[test]
fn test_print() {
    println!("tes print");
}

#[test]
fn test_variable() {
    // imutable variable
    let my_name = "Faqih";
    let semester = 6;
    println!("Hallo, saya {}, saya semester {}", my_name, semester);

    // mutable variable
    let mut uname = "miawpaw";
    println!("username: {}", uname);

    uname = "saipudin666666666666";
    println!("username: {}", uname);

    // shadowing dalam rust
    // kenapa dikasih underscore di dpn var tidak terpakai untuk silence/supress warning dari compiler
    let _idu = 14412;
    let _idu = "200321";
}

#[test]
fn number() {
    // ini akan menyesuaikan brp bit dengan system operasi saat di compile
    let _contoh_uzise: usize = 34;

    let idu: i8 = 123;
    println!("id usser: {}", idu);

    // contoh konfersi tipe data number
    let mut a = idu as i32;
    println!("konfersi {}", a);
    a = 234413;
    println!("konfersi {}", a);
}

#[test]
fn numeric_operator() {
    let a = 223;
    let b = 32;

    let mut _c = a + b;

    let mut _d;
    _d = 44;

    // contoh augmented assignment
    _c += 10;
    println!("{}", _c);
}

#[test]
fn comparasion_operator() {
    let a = 10;
    let b = 3;
    println!("{}", a < b);
}

#[test]
fn boolean_operator() {
    let a = true;
    let b = false;

    println!("{}", a && b); // and operator syarat ke2 true menghasilkan true selain itu false
    println!("{}", a || b); // or operator salah satu true maka hasilnya true, gk ada  maka false
    println!("{}", !b); // not operator jika tidak sama / jika false maka true / jika true maka false
}

// mengenal tipe data compound yg isinya bisa banyak tipe data lainnya
#[test]
fn tuple() {
    let data: (i32, f32, bool) = (332, 3.14, true); // tuple bagusnya digunakan untuk tipe data yg berbeda
    println!("{:?}", data);

    // cara mengakses data dari tuple  dengan cara .nomor-indeks dari data tersebut, urutan indeks dimulai dari 0
    println!("{:?}", data.0);

    // ini dinamakan destructuring tuple dmna data dari tuple di atas di bongkar dan di masukan ke dalam variable
    let (a, b, _) = data; // data yg tidak gunakan bisa di suprres dengan _ agar tidak warning
    println!("{} {}", a, b);

    // contoh mutable tuple
    let mut jumlah_siswa_tiap_kelas: (i32, i32, bool) = (33, 29, true);

    // dengan tuple yg mutable bisa di ubah nilai dri setiap indexnya
    jumlah_siswa_tiap_kelas.0 = 30;
    println!("{:?}", jumlah_siswa_tiap_kelas);
}

// contoh unit = tuple yg kosong
fn _unit() {
    println!("Hello");
}

#[test]
fn test_unit() {
    let result = _unit(); // bisa diliat bahwa hasil dri fn unit itu tuple kosong saya juga masih bingung
    println!("{:?}", result);
}

#[test]
fn array() {
    // membuat array dengann implisit/tidak di sebutkan jenis dan jumlah datannya
    let array1 = [1, 2, 3, 4];
    println!("{:?}", array1);

    // membuat array dengan cara explisit/tipe data dan jumlahnya disebutkan
    let array2: [i8; 6] = [1, 2, 3, 4, 5, 6];
    println!("{:?}", array2);

    let _a = array2[1]; // cara mengakses nilai dari array

    let [_a, _b, _c, _] = array1; // contoh destructuring pada array

    // mutable array
    let mut _array3 = [123, 543, 31];
    _array3[2] = 432; // dengan ini bisa mengubah isi dari arraynya

    // cara menghitung panjang array/jumlah datanya
    let _lenght = array1.len(); // bisa di lihat tipe panjang  data itu usize dmna menyesuaikan komputernya 32/64 bit
    println!("panjang array {:?}", _array3.len()); // denga fungsi .len() milih array mampu menghitung panjang di array
}

// array 2 dimensi / metrix
#[test]
fn dimensional_array() {
    let _metrix3d = [
        //ini  array 3 dimensi
        // ini array 2 dimensi
        [
            [1, 3], // ini array 1 dimensi
            [4, 4],
        ],
        // ini array 2 dimensi
        [[5, 6], [3, 1]],
    ];

    println!(": {:?}", _metrix3d);
    // Mengambil angka 4 (Blok ke-0, Baris ke-1, Kolom ke-0) itu urutan dengan index
    println!("Angka: {}", _metrix3d[0][1][0]);
    // ngambil isi array block ke-0, baris ke-0
    println!("Angka: {:?}", _metrix3d[0][1]);
}

#[test]
fn constanta() {
    // membuat variebel constan = tidak dapat di ubah
    // deklarasi dengan const tidak dengan let
    // nama variabel harus huruf besar & pemisah kata menggunakan _
    // tipe data harus explisit/disebutkan
    const TES: i32 = 12231;
    println!("nilai conts: {}", TES);
}

// stak = tempat menaruh data yg fix
// heap = tempat menaruh data yg dinamis dan akan mengembalikan pointer
#[test]
fn stack_heap() {
    _function_a();
    _function_b();
}

fn _function_a() {
    let a = 10;
    let b = String::from("Faqih");
    println!("{} {}", a, b);
}

fn _function_b() {
    let a = 10;
    let b = String::from("Faqih");
    println!("{} {}", a, b);
}

#[test]
fn string() {
    let name: &str = " ini string slice &str "; //  tipe data string slice &str
    let trim_name: &str = name.trim(); // digunakan untuk menghapus karakter kosong spasi  pada awal & akhir
    println!("{}", trim_name);

    let mut nama: String = String::from("faqih"); // walaupun dasarnya bisa di ubah tpi harus memakai mut
    println!("{}", nama);
    nama.push(' '); // ini buat char cuma bisa 1 karakter
    nama.push_str("abdullah"); // ini buat string
    println!("{}", nama);
    let faqih = nama.replace("faqih", "imam");
    println!("{}", faqih);
}

// ownership
#[test]
fn ownership_rules() {
    // a tidak bbisa di akses di sini krn blm di buat
    let a = 10; // bisa di akses mulai dari sini

    {
        //  b tidak bisa di  akses dari sini krn blm di deklarasi
        let b = 20;
        println!("{}", b);
    } // scope b selesai, b dihapus, b tidak bisa di akses lagi 

    println!("{}", a);
} // scope a selesai, a dihapus, a tidak bisa di akses lagi

// data copy di fixed size variabel/data yg disimpan di stack
#[test]
fn data_copy() {
    let a = 10;
    let b = a; // b akan copy data a, bukan memiliki/mengambil hak data

    println!("{}", b);
}

#[test]
fn ownership_transfer() {
    let a = String::from("faqih");

    // ownership data dari a dipindahkan ke b
    let b = a;
    // variabel a tidak lagi valid dari sini

    println!("{}", b);
    // println!("{}", a);
}

#[test]
fn data_heap_clone() {
    // intinya sama aja kaya data copy di fixed size variable
    let a = String::from("radit");

    let b = a.clone(); // akan membuat clone data dari a tpi di masukannya ke heap memory

    println!("{}", a);
    println!("{}", b);

    {
        let c = b.clone();
        println!("{}", c);
        let _d = &a;
    }

    // let d = &c; gk bisa borrow ownership  klo dri dalam ke luar
}

#[test]
fn if_expresion() {
    let nilai_mtk = 67;

    if nilai_mtk >= 75 {
        println!("anda lulus matkul mtk")
    } else if nilai_mtk == 75 {
        println!("kkm")
    } else {
        println!("anda tidak lulus matkul  mtk")
    }

    // di dalem variable kita bisa memakai if untuk mengisi hasilnya
    let _hasil = if nilai_mtk >= 75 { true } else { false };
}

// loop perulangan yg akan terus mengulang, kita harus menyuruhnya berhenti
#[test]
fn loop_expresion() {
    let mut nilai = 1;
    loop {
        if nilai == 10 {
            break;
        } else if nilai == 5 {
            nilai += 1;
            continue;
        } else {
            println!("nilai {}", nilai);
            nilai += 1;
        }
    }
}

#[test]
fn loop_return_value() {
    // return value pada loop
    let mut a = 0;
    let counter = loop {
        a += 1;
        if a > 10 {
            break a * 2; // return value yg akan mengisi counter
        }
    };

    println!("{} {}", a, counter);
}

// loop label berguna untuk melabeli loop, bisa jga untuk menghentikan loop spesifik saat nested
#[test]
fn loop_label() {
    let mut number = 1;
    // membuat label cukup dengan  'nama_label: loop{}
    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer; //  jika kita break aja tanpa menyebutkan labelnya, loop yg berhenti yg di dlm
            }

            println!("{} x {} = {}", number, i, number * i);
            i += 1;
            if i > 10 {
                break;
            }
        }

        number += 1;
    }
}

// #[test]
// fn loop_segitiga() {
//     let mut number = 1;
//     'outer: loop {
//         let mut baris: String = String::from("");
//         let mut i = 1;
//         loop {
//             if number > 6 {
//                 break 'outer;
//             }

//             loop {
//                 if i == i{
//                     break
//                 }
//                 baris.push_str("*");
//             }

//             println!("{}", baris);
//             if  i == i{
//                 i += 1
//             }
//         }

//         number += 1;
//     }
// }

// while loop sama kaya biasanya
#[test]
fn while_loop() {
    let mut penentu = 1;
    while penentu <= 10 {
        println!("{}", penentu);
        penentu += 1;
    }
}

#[test]
fn array_iteration_while_loop() {
    // iterasi array/pengulangan array untuk membaca isinya
    let pustaka = ["A", "B", "C", "D"];
    let mut pengulangan: usize = 0;
    while pengulangan < pustaka.len() {
        println!("{}", pustaka[pengulangan]);
        pengulangan += 1;
    }
}

#[test]
fn array_iteration_for_loop() {
    // lebih simpel
    let pustaka = ["A", "B", "C", "D"];

    for index in pustaka {
        // langsung masuk value dia klo  pke for loop
        println!("{}", index);
    }
}

// variabel tipe range dalam rust
#[test]
fn tipe_data_range() {
    let pustaka = ["A", "B", "C", "D"];

    let penentu = 0..4; // angka terakhir itu kaya gk disebutin

    for i in penentu {
        println!("{}", pustaka[i]);
    }

    for i in 0..4 {
        println!("{}", pustaka[i]);
    }
}

#[test]
fn range() {
    let range = 1..6;
    println!("{}", range.start); // startnya includ bakal di ambil
    println!("{}", range.end); // endnya excluds/pengecualian(bahasa mudahnya) gk di ambil/pake
}

// range inclusive bedanya angka trakhirnya di ambil juga
#[test]
fn range_inclusive() {
    let angka = 0..=3; // jika range melebihi indeks array maka akan membuat panic dan program akan terhenti ketika range membaca indeks array yg tidak ada

    let huruf: [&str; 4] = ["A", "B", "C", "D"];

    for i in angka {
        println!("huruf: {}", huruf[i]);
    }

    for i in huruf {
        println!("huruf: {}", i);
    }
}

// function
fn greating(first_name: &str, last_name: &str){ // parameter tipe datanya harus di sebutkan/definisi
    println!("Hallo {} {}", first_name, last_name);
}

#[test]
fn test_greating() {
    greating("faqih", "abdullah");
}

// parameter pada function
fn factorial_math(n: i32) -> i32 { // nilai return value bisa di sebutkan dengan -> tipe_data
    if n < 1 {
        return 0;
    }

    let mut result = 1;
    for i in 1..=n {
        result *= i
    }
    result
}

#[test]
fn test_factorial_math() {
    let angka = factorial_math(5);
    println!("{}", angka)
}

// rescursive fn masih kurang jelas tpi intinya ini bisa memanggil diri sendiri sampe selesai
fn print_text(value: String, times: u32) {
    if times == 0 {
        return;
    } else {
        println!("value:{} time:{}", value,  times);
    }

    print_text(value, times-1);
}

#[test]
fn test_print_text() {
    print_text(String::from("nama"), 100);
}

fn rescursive_factorial_math(n:i32) -> i32 {
    if n <= 1 {
        return 1;
    }

    n * rescursive_factorial_math(n - 1)
}

#[test]
fn test_rescursive_factorial() {
    let result = rescursive_factorial_math(5);
    println!("{}", result);
}

// ownership dalam function -- start

// perpindahan ownership dari variabel ke fn parameter
fn hi_number(n:i32) { // parameternya tipe integer termaksud stack memory
    println!("hallo {}", n)
}

fn hi(word:String) {  // parameternya tipe string termaksud heap memory
    println!("hallo {}", word)
}

#[test]
fn test_hi_number() {
    let angka  = 10;
    hi_number(angka); // nilai dalam angka akan di copy ke parameter fn itu krn stack memory
    println!("{}", angka); // angka masih bisa digunakan karena cuma di copy ke parameter fn
}

#[test]
fn test_hi() {
    let nama  =  String::from("faqih"); // heap memory dia itu hanya boleh 1 owner valuenya
    // hi(nama);
    // jika variabel nama kita pakai, maka ownership value akan di transfer ke parameter fn  tersebut, fn selesai maka variabel ataupun value semua akan di hapus dri memory
    hi(nama.clone()); // berbeda jika tidak pke metode .clone() dimana dia copy data dri heap memory
    println!("{}", nama); // maka variabel nama tetep bisa di pakai  
}

// return value ownership
fn full_name(f_name:String, l_name:String) -> String {
    format!("{} {}",f_name, l_name )  // ini adalah macro yg berfungsi copy ke 2 data string dan menghasilkan 1 data baru di memory heap
    // di sini fn akan kembalikan data string yg baru dri format!() dan delete 2 parameter dri memory
}

#[test]
fn test_name() {
    let first_name = String::from("faqih");
    let last_name = String::from("abdullah");

    // owner value dari variabel di atas akan pindah ke parameter
    // lalu di gabungkan dengan format!() dan di kembalikan oleh fn
    // kedumian di pindahkan lagi ownershipnya ke variabel my_name yg memanggil fn isinya
    let my_name = full_name(first_name, last_name);

    // println!("{}", first_name); // tidak bisa di pakai krn owner ship pindah
    // println!("{}", last_name); // tidak bisa di pakai krn owner ship pindah
    println!("{}", my_name);
}

// mengembalikan ownership denga tuple
// contohnya kita pakai kode yg di atas tadi
fn komponen_komputer(cpu:String, ram:String) -> (String, String, String) {
    let komponen_pc = format!("{} {}", cpu, ram);
    (cpu, ram, komponen_pc)
}

#[test]
fn pc_saya() {
    let cpu =  String::from("AMD Ryzen 8600G");
    let ram = String::from("32GB DDR 5");

    // di sini ownership akan dibongkar(destrucsion tuple) ke variabel di yg sesuai di dlm list tuple
    let (cpu, ram, my_pc) = komponen_komputer(cpu, ram); // fn akan mengembalikan return berupa tuple dan di destruction ke dlm variabel sesuai urutan
    // saat let itu kita shadowing variabel yg sudah ada bukan memakainya lagi/membuat baru dengan menuliskannya

    println!("{}", cpu);
    println!("{}", ram);
    println!("{}", my_pc);
}
// ownership dalam function -- end

// refrence dan borrowing -- start
// menggunakan refrence untuk menunjuk tempat datanya tanpa mengambil ownershipnya
fn sebut_kata(kata1: &String,  kata2: &String) -> String {
    // dengan memberikan & bermaksud refrence(mengarahkan lokasi) bawah parameter cuma mengarahkan ke tujuan data
    // tidak meminjam ownership dari variabel di pakainya
    format!("{} {}", kata1, kata2)
}

#[test]
fn test_panggil_kata() {
    let kata_pertama = String::from("heloo");
    let kata_kedua = String::from("world");

    // fungsi &variabel yg di pke untuk param berguna mengarahkan ke lokasi data variabel tersebut tanpa mengambil ownership
    let gabungkan = sebut_kata(&kata_pertama, &kata_kedua);
    println!("{}", kata_pertama); // dengan begitu variabel bisa digunakan lagi
    println!("{}", kata_kedua);
    println!("{}", gabungkan);
}

// mengganti value dari borrowing (tidak boleh, tapi jika di kasih &mut compiler bolehin)
fn minjem_motor(motor: &mut String) { // jika di kasih kata depan mut compiler memperbolehkannya
    motor.push_str(" modif") //compiler akan menolak krn refrence tidak boleh di ubah walau nilainya mutable
}

#[test]
fn test_pinjemin() {
    let mut merek = String::from("beat");
    minjem_motor(&mut merek); // saat menggunakan refrence klo data mau di ubah bisa pke &mut nama_variable
    println!("{}", merek)
}
// refrence dan borrowing -- end

// slice -- start
// refrence dalam slice
#[test]
fn slice() {
    let angka = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", angka);
    
    let slice1: &[i32] = &angka[..];
    println!("{:?}", slice1);

    let slice2: &[i32] = &angka[5..10];
    println!("{:?}", slice2);

    let slice3: &[i32] = &angka[6..];
    println!("{:?}", slice3);
    // println!("{:?}", )
}
// slice -- end

// string slice -- start
// jika String kita slice akan menghasilkan &str, dia itu refrence doang dan akan masuk ke fixed size
#[test]
fn test_string_slice() {
    let nama_saya = String::from("faqih abdullah");
    let nama_depan: &str = &nama_saya[0..5];
    let nama_belakang: &str = &nama_saya[6..];
    println!("{}", nama_depan);
    println!("{}", nama_belakang);
}
// string slice -- end

// struct -- start
// hampir sama kaya tuple, klo struct urutannya pke nama kaya class jatuhnya ya
struct Person {
    nama: String,
    umur: u8
}

impl Person {
    fn new(nama: &str, umur: u8) -> Person {
        Person{nama: nama.to_string(), umur}
    }
}

fn informasi_orang(nama: &String, umur: u8) -> String {
    format!("nama dia {}, umur dia {}", nama, umur)
}

#[test]
fn test_informasi_orang() {
    // ini adalah contoh instace yg di buat dari Person
    let orang1: Person = Person {
        nama: String::from("faqih abdullah"),
        umur: 21,
    };
    let sapa = informasi_orang(&orang1.nama, orang1.umur);
    println!("{}", orang1.nama);
    println!("{}", sapa);

    // contoh init shorthand dalam struct
    let nama = String::from("taiho"); // syarat nama variabel harus sama dengan nama  field
    let umur = 24;
    let orang2: Person = Person {
        nama, // ownership akan pindah kesini
        umur,
    };
    println!("{} {}", orang2.nama, orang2.umur);
}

// tuple struck intinya struck gk pke nama field aksesnya sama kaya tuple
struct NilaiXY (f64, f64); // tuple struk make urutan kaya indeks 0..seterusnya

impl NilaiXY {
    // ASSOCIATED FUNCTION
    fn new(x: f64, y: f64) -> NilaiXY {
        // tipe returnnya nama struct/enum/trait nya sendiri
        NilaiXY(x, y) // ini bkn manguil fn  lagi tapi, ini tuple struct makanya pke () bkn {}
    }
}

#[test]
fn test_nilaixy() {
    let kordinat_2d = NilaiXY (-51.3142, 313.222);

    let kordinat_player = NilaiXY::new(18.3125, 434.656);
    println!("{}", kordinat_player.0); // cara akses model tuple
    println!("{}", kordinat_player.1);

    // cara aksesnya
    println!("x:{} y:{}", kordinat_2d.0, kordinat_2d.1) // memakai nama_variabel.indeks/urutannya
}

// struct tanpa field
struct _Kosong;

#[test]
fn test_kosong() {
    let _isi = _Kosong;
}
// struct -- end

// methode -- start
// methode itu sama aja kaya fn, tpi dia menempel di struct, trait, dan enum
// pada methode parameter pertama selalu self = field yg ada di dlm struct tersebut
// parameter self biasanya dibuat dengan bentuk refrence
struct Mobil {
    merek: String,
    harga: u64,
}

// untuk membuat method pakai impl=implementasi
// lalu sebutkan nama struct/enum/trait
impl Mobil {
    // lalu buat fn di dalam scope ini
    fn mobil_wow(&self) { // pke refrence biar gk pindah ownershipnya
        println!("ini merek {}, harganya {}", self.merek, self.harga)
    }

    // ASSOCIATED FUNCTION
    fn new(merek: &str, harga: u64) -> Mobil {
        Mobil {
            merek: merek.to_string(), // convert ke string biar pas pke fn gk ribet
            harga: harga
        }
    }
}

#[test]
fn test_mobil() {
    let mobil1 = Mobil {
        merek: String::from("mbw"),
        harga: 100000000
    };

    // cara pemanggilan methode yg di buat
    mobil1.mobil_wow();
    println!("{}", mobil1.merek);

    let mobil2 = Mobil::new("toyota", 10000);
    mobil2.mobil_wow();
    println!("{}", mobil2.merek);
}
// methode -- end

// enum -- start
// enum itu kaya tipe data yg  punya opsi pilihan, contohnya

// tipe data buatan kita sendiri (seperti struct dan enum) tidak secara otomatis tahu bagaimana cara menampilkan wujudnya ke layar

// contoh enum sederhana
#[derive(Debug)] // Tambahkan baris ini agar LevelMember bisa di-print pakai {:?}
enum LevelMember {
    Bronze,
    Silver,
    Gold
}

// contoh enum bisa simpan data/seperti tuple tiap field-nya
#[derive(Debug)] // Tambahkan baris ini agar LevelMember bisa di-print pakai {:?}
enum Payment {
    CreditCard(String),
    BankTransfer(String, String),
    EWallet(String, String)
}

// methode dalam enum
impl LevelMember {
    fn greating_member(&self) -> String {
        format!("Selamat Bergabung")
    }
}

impl Payment {
    // membuat methode, mencoba matching patern untuk menentukan pilihan dari enum
    fn check_member(&self, total_bayar: u64, level_member: &LevelMember) {

        let mut discount;
        // jika ingin memakai 2 enum berbeda
        match (self, level_member) {
            (Payment::CreditCard(nama_bank), LevelMember::Bronze) => {
                println!("membayar dengan {}", nama_bank);
                discount = 10;
            }
            (Payment::CreditCard(nama_bank), LevelMember::Silver) => {
                println!("membayar dengan {}", nama_bank);
                discount = 15;
            }
            (Payment::CreditCard(nama_bank), LevelMember::Gold) => {
                println!("membayar dengan {}", nama_bank);
                discount = 30;
            }
            (Payment::BankTransfer(nama_bank, norek), LevelMember::Bronze) => {
                println!("membayar dengan {}, nomor rekening: {}", nama_bank, norek);
                discount = 5;
            }
            (Payment::BankTransfer(nama_bank, norek), LevelMember::Silver) => {
                println!("membayar dengan {}, nomor rekening: {}", nama_bank, norek);
                discount = 10;
            }
            (Payment::BankTransfer(nama_bank, norek), LevelMember::Gold) => {
                println!("membayar dengan {}, nomor rekening: {}", nama_bank, norek);
                discount = 15;
            }
            (Payment::EWallet(nama_bank, norek), LevelMember::Bronze) => {
                println!("membayar dengan {}, nomor rekening: {}", nama_bank, norek);
                discount = 5;
            }
            (Payment::EWallet(nama_bank, norek), LevelMember::Silver) => {
                println!("membayar dengan {}, nomor rekening: {}", nama_bank, norek);
                discount = 10;
            }
            (Payment::EWallet(nama_bank, norek), LevelMember::Gold) => {
                println!("membayar dengan {}, nomor rekening: {}", nama_bank, norek);
                discount = 15;
            }
        }

        let hasil = total_bayar - ((total_bayar*discount)/100);
        println!("Member membayar:{}", total_bayar);
        println!("{} discount {}% = {}", total_bayar, discount, hasil);
    }
}
#[test]
fn test_enum() {
    // jika kita ingin memakai enum
    let _anggota1 = LevelMember::Bronze; // NamaEnum::NamaField
    let _anggota2 = LevelMember::Silver;
    let _anggota3 = LevelMember::Gold;

    // contoh memakai enum dengan field tuple
    let _payment1 = Payment::CreditCard(String::from("BCA"));
    let _payment2 = Payment::BankTransfer(String::from("Bank Mandiri"), String::from("rek:223122231"));
    let _payment3 = Payment::EWallet(String::from("Dana"), String::from("no:083124123"));

    println!("{:?} {:?}", _anggota1, _payment1);
    println!("{:?} {:?}", _anggota2, _payment2);
    println!("{:?} {:?}", _anggota3, _payment3);

    // satu-satunya cara untuk membongkar dan mengambil isi data dari dalam enum adalah menggunakan Pattern Matching (Pencocokan Pola).
}
// enum -- end

// patern matching -- start
// bisa digunakan untuk melakukan pengecekan value, variable, dan banyak hal

// patern matching itu cara kita mengakses enum, krn di dalam enum ada banyak sekali opsi
// patern matching untuk enum
#[test]
fn test_enum_pattern_matching() {
    let level_anggota1 = LevelMember::Bronze;
    let payment_anggota1  = Payment::EWallet(String::from("Dana"), String::from("0831235343"));

    // cara menggunakan pada variabel
    // match nama_variabel {}
    match level_anggota1 {
        // disini semua opsi variant(tipe data yg kita buat) harus ada outputnya
        // penulisannya NamaEnum::NamaField/Opsi-nya => {}
        LevelMember::Bronze => {
            println!("hallo anggota bronze")
        }
        LevelMember::Silver => {
            println!("hallo anggota silver")
        }
        LevelMember::Gold => {
            println!("hallo anggota gold")
        }
    }

    payment_anggota1.check_member(230000, &level_anggota1);
}

#[test]
fn test_pattern_matching_value() {
    // pattern matching pada value
    let nama = "faqih";
    match nama {
        "abdullah" => {
            println!("hallo abdullah")
        }
        other => { // other bekerja untuk menampung value yg tidak ada
            println!("hallo {}", other)
        }
    }

    // multiple matching
    match nama {
        "abdullah" | "imam" => {  // disini bisa menggunakan pipe simbol |, sebagai atau
            println!("hallo abdullah")
        }
        other => { // other bekerja untuk menampung value yg tidak ada
            println!("hallo {}", other)
        }
    }

    // range value dalam matching
    // bisa di lakukan dengan tipe data range (seperti pada materi slice)
    let nilai = 100;
    match nilai {
        85..=100 => {println!("bagus sekali")} // memakai inlcusive range
        75..=84 => {println!("bagus")}
        40..=74 => {println!("jelek")}
        0..=39 => {println!("DO")}
        // Tipe data i32 bisa menampung angka dari minus 2,1 Miliar sampai plus 2,1 Miliar
        other => {println!("gk sesuai {}", other)} // syaratnya wajib ada variabel untuk menampung nilai dri i32
    }
}

// destruc match pattern
// kita bisa membongkar struct field
#[test]
fn test_struct_pattern() {
    let axis = NilaiXY::new(0.0, 14.53);

    // destruc pada struct model tuple
    match axis {
        NilaiXY(0.0, y) => {
            println!("y:{}", y)
        }
        NilaiXY(x, 0.0) => {
            println!("x:{}", x)
        }
        NilaiXY(x, y) => {
            println!("x:{} y:{}", x, y)
        }
    }

    // destruc pada struct model named field
    // let orang1 = Person::new(String::from("faqih A"), 22);
    let orang1 = Person::new("faqih A", 22);
    // kenapa cuma 1 opsi krn tipenya cuma punya 1 wujud
    // beda dengan enum yg banyak opsinya
    match orang1 {
        Person { nama, ..} => { // .. artinya tidak di pke
            println!("nama: {}", nama)
        }
    }

    // contoh ignoring dengan _ 
    // dipakai jika data tidak dibutuhkan
    match axis {
        NilaiXY(_, y) => {
            println!("y:{}", y)
        }
    }
    let nilai = 10022;
    match nilai {
        85..=100 => {println!("bagus sekali")} // memakai inlcusive range
        75..=84 => {println!("bagus")}
        40..=74 => {println!("jelek")}
        0..=39 => {println!("DO")}
        // Tipe data i32 bisa menampung angka dari minus 2,1 Miliar sampai plus 2,1 Miliar
        _ => {println!("invalid data")} // syaratnya wajib ada variabel untuk menampung nilai dri i32
    }
}

// match expresion bisa menghasilkan value
// sama kaya if while bisa menghasilkan value
#[test]
fn test_match_expresion() {
    let angka = 4;
    let hasil = match angka {
        1 => {"1"}
        2 => {"2"}
        3 => {"3"}
        _ => {"invalid"}
    };
    println!("{}", hasil);
}
// patern matching -- end