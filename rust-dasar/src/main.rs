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
    let _idu  = 14412;
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
    let a =  223;
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
    let mut jumlah_siswa_tiap_kelas: (i32, i32, bool)  = (33, 29, true);

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
    let _metrix3d = [ //ini  array 3 dimensi
        // ini array 2 dimensi
        [
            [1, 3], // ini array 1 dimensi
            [4, 4],
        ],
        // ini array 2 dimensi
        [
            [5, 6],
            [3, 1]
        ]
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

    {  //  b tidak bisa di  akses dari sini krn blm di deklarasi
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
fn data_heap_clone() { // intinya sama aja kaya data copy di fixed size variable
    let a = String::from("radit");

    let b = a.clone(); // akan membuat clone data dari a tpi di masukannya ke heap memory

    println!("{}", a);
    println!("{}", b);

    {
        let c = b.clone();
        println!("{}", c);
    }

    // let d = &c; gk bisa borrow ownership  klo dri dalam ke luar
}