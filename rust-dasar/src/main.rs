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