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
}