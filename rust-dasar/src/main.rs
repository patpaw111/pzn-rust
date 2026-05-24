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

    println!("{}", a && b); // and operator syara ke2 true menghasilkan true selain itu false
    println!("{}", a || b); // or operator salah satu true maka hasilnya true, gk ada  maka false
    println!("{}", !b); // not operator jika tidak sama / jika false maka true / jika true maka false
}