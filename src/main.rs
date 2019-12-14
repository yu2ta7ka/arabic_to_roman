use std::io;

struct Roman {
    roman: [(u16, String); 18],
}

impl Roman {
    /// 対応表の初期化
    /// 参照：https://ja.wikipedia.org/wiki/%E3%83%AD%E3%83%BC%E3%83%9E%E6%95%B0%E5%AD%97
    pub fn new() -> Roman {
        Roman {
            roman: [
                (1000, "M".to_string()),
                (900, "CM".to_string()),
                (500, "D".to_string()),
                (400, "CD".to_string()),
                (100, "C".to_string()),
                (90, "XC".to_string()),
                (50, "L".to_string()),
                (40, "XL".to_string()),
                (10, "X".to_string()),
                (9, "IX".to_string()),
                (8, "VIII".to_string()),
                (7, "VII".to_string()),
                (6, "VI".to_string()),
                (5, "V".to_string()),
                (4, "IV".to_string()),
                (3, "III".to_string()),
                (2, "II".to_string()),
                (1, "I".to_string()),
            ],
        }
    }

    /// アラビア数字をローマ数字へ変換する
    pub fn arabic_to_roman(&self, input: u16) -> String {
        let mut n = input;
        let mut r_string = String::new();
        for index in self.roman.iter() {
            let d = n / index.0;
            let r_ch = match d {
                0 => "".to_string(),
                _ => {
                    n = n % index.0;
                    Roman::write_roman(d, &index.1)
                }
            };
            r_string += &r_ch;
        }
        r_string
    }

    /// 該当するローマ数字の文字を追加する
    fn write_roman(d: u16, ch_roman: &String) -> String {
        let mut r = String::new();
        for _i in 0..d {
            r += &ch_roman;
        }
        r
    }
}

fn main() {
    println!("Arabic to Roman: Please input arabic number");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: u16 = input.trim().parse().expect("please type a number(<2000)");

    let r = Roman::new();
    println!("roman={:?}", r.arabic_to_roman(input));
}

#[test]
fn arabic_to_roman_works() {
    const SIZE: usize = 1000;
    let r = Roman::new();
    let roman: [String; SIZE] = [
        "I".to_string(),
        "II".to_string(),
        "III".to_string(),
        "IV".to_string(),
        "V".to_string(),
        "VI".to_string(),
        "VII".to_string(),
        "VIII".to_string(),
        "IX".to_string(),
        "X".to_string(),
        "XI".to_string(),
        "XII".to_string(),
        "XIII".to_string(),
        "XIV".to_string(),
        "XV".to_string(),
        "XVI".to_string(),
        "XVII".to_string(),
        "XVIII".to_string(),
        "XIX".to_string(),
        "XX".to_string(),
        "XXI".to_string(),
        "XXII".to_string(),
        "XXIII".to_string(),
        "XXIV".to_string(),
        "XXV".to_string(),
        "XXVI".to_string(),
        "XXVII".to_string(),
        "XXVIII".to_string(),
        "XXIX".to_string(),
        "XXX".to_string(),
        "XXXI".to_string(),
        "XXXII".to_string(),
        "XXXIII".to_string(),
        "XXXIV".to_string(),
        "XXXV".to_string(),
        "XXXVI".to_string(),
        "XXXVII".to_string(),
        "XXXVIII".to_string(),
        "XXXIX".to_string(),
        "XL".to_string(),
        "XLI".to_string(),
        "XLII".to_string(),
        "XLIII".to_string(),
        "XLIV".to_string(),
        "XLV".to_string(),
        "XLVI".to_string(),
        "XLVII".to_string(),
        "XLVIII".to_string(),
        "XLIX".to_string(),
        "L".to_string(),
        "LI".to_string(),
        "LII".to_string(),
        "LIII".to_string(),
        "LIV".to_string(),
        "LV".to_string(),
        "LVI".to_string(),
        "LVII".to_string(),
        "LVIII".to_string(),
        "LIX".to_string(),
        "LX".to_string(),
        "LXI".to_string(),
        "LXII".to_string(),
        "LXIII".to_string(),
        "LXIV".to_string(),
        "LXV".to_string(),
        "LXVI".to_string(),
        "LXVII".to_string(),
        "LXVIII".to_string(),
        "LXIX".to_string(),
        "LXX".to_string(),
        "LXXI".to_string(),
        "LXXII".to_string(),
        "LXXIII".to_string(),
        "LXXIV".to_string(),
        "LXXV".to_string(),
        "LXXVI".to_string(),
        "LXXVII".to_string(),
        "LXXVIII".to_string(),
        "LXXIX".to_string(),
        "LXXX".to_string(),
        "LXXXI".to_string(),
        "LXXXII".to_string(),
        "LXXXIII".to_string(),
        "LXXXIV".to_string(),
        "LXXXV".to_string(),
        "LXXXVI".to_string(),
        "LXXXVII".to_string(),
        "LXXXVIII".to_string(),
        "LXXXIX".to_string(),
        "XC".to_string(),
        "XCI".to_string(),
        "XCII".to_string(),
        "XCIII".to_string(),
        "XCIV".to_string(),
        "XCV".to_string(),
        "XCVI".to_string(),
        "XCVII".to_string(),
        "XCVIII".to_string(),
        "XCIX".to_string(),
        "C".to_string(),
        "CI".to_string(),
        "CII".to_string(),
        "CIII".to_string(),
        "CIV".to_string(),
        "CV".to_string(),
        "CVI".to_string(),
        "CVII".to_string(),
        "CVIII".to_string(),
        "CIX".to_string(),
        "CX".to_string(),
        "CXI".to_string(),
        "CXII".to_string(),
        "CXIII".to_string(),
        "CXIV".to_string(),
        "CXV".to_string(),
        "CXVI".to_string(),
        "CXVII".to_string(),
        "CXVIII".to_string(),
        "CXIX".to_string(),
        "CXX".to_string(),
        "CXXI".to_string(),
        "CXXII".to_string(),
        "CXXIII".to_string(),
        "CXXIV".to_string(),
        "CXXV".to_string(),
        "CXXVI".to_string(),
        "CXXVII".to_string(),
        "CXXVIII".to_string(),
        "CXXIX".to_string(),
        "CXXX".to_string(),
        "CXXXI".to_string(),
        "CXXXII".to_string(),
        "CXXXIII".to_string(),
        "CXXXIV".to_string(),
        "CXXXV".to_string(),
        "CXXXVI".to_string(),
        "CXXXVII".to_string(),
        "CXXXVIII".to_string(),
        "CXXXIX".to_string(),
        "CXL".to_string(),
        "CXLI".to_string(),
        "CXLII".to_string(),
        "CXLIII".to_string(),
        "CXLIV".to_string(),
        "CXLV".to_string(),
        "CXLVI".to_string(),
        "CXLVII".to_string(),
        "CXLVIII".to_string(),
        "CXLIX".to_string(),
        "CL".to_string(),
        "CLI".to_string(),
        "CLII".to_string(),
        "CLIII".to_string(),
        "CLIV".to_string(),
        "CLV".to_string(),
        "CLVI".to_string(),
        "CLVII".to_string(),
        "CLVIII".to_string(),
        "CLIX".to_string(),
        "CLX".to_string(),
        "CLXI".to_string(),
        "CLXII".to_string(),
        "CLXIII".to_string(),
        "CLXIV".to_string(),
        "CLXV".to_string(),
        "CLXVI".to_string(),
        "CLXVII".to_string(),
        "CLXVIII".to_string(),
        "CLXIX".to_string(),
        "CLXX".to_string(),
        "CLXXI".to_string(),
        "CLXXII".to_string(),
        "CLXXIII".to_string(),
        "CLXXIV".to_string(),
        "CLXXV".to_string(),
        "CLXXVI".to_string(),
        "CLXXVII".to_string(),
        "CLXXVIII".to_string(),
        "CLXXIX".to_string(),
        "CLXXX".to_string(),
        "CLXXXI".to_string(),
        "CLXXXII".to_string(),
        "CLXXXIII".to_string(),
        "CLXXXIV".to_string(),
        "CLXXXV".to_string(),
        "CLXXXVI".to_string(),
        "CLXXXVII".to_string(),
        "CLXXXVIII".to_string(),
        "CLXXXIX".to_string(),
        "CXC".to_string(),
        "CXCI".to_string(),
        "CXCII".to_string(),
        "CXCIII".to_string(),
        "CXCIV".to_string(),
        "CXCV".to_string(),
        "CXCVI".to_string(),
        "CXCVII".to_string(),
        "CXCVIII".to_string(),
        "CXCIX".to_string(),
        "CC".to_string(),
        "CCI".to_string(),
        "CCII".to_string(),
        "CCIII".to_string(),
        "CCIV".to_string(),
        "CCV".to_string(),
        "CCVI".to_string(),
        "CCVII".to_string(),
        "CCVIII".to_string(),
        "CCIX".to_string(),
        "CCX".to_string(),
        "CCXI".to_string(),
        "CCXII".to_string(),
        "CCXIII".to_string(),
        "CCXIV".to_string(),
        "CCXV".to_string(),
        "CCXVI".to_string(),
        "CCXVII".to_string(),
        "CCXVIII".to_string(),
        "CCXIX".to_string(),
        "CCXX".to_string(),
        "CCXXI".to_string(),
        "CCXXII".to_string(),
        "CCXXIII".to_string(),
        "CCXXIV".to_string(),
        "CCXXV".to_string(),
        "CCXXVI".to_string(),
        "CCXXVII".to_string(),
        "CCXXVIII".to_string(),
        "CCXXIX".to_string(),
        "CCXXX".to_string(),
        "CCXXXI".to_string(),
        "CCXXXII".to_string(),
        "CCXXXIII".to_string(),
        "CCXXXIV".to_string(),
        "CCXXXV".to_string(),
        "CCXXXVI".to_string(),
        "CCXXXVII".to_string(),
        "CCXXXVIII".to_string(),
        "CCXXXIX".to_string(),
        "CCXL".to_string(),
        "CCXLI".to_string(),
        "CCXLII".to_string(),
        "CCXLIII".to_string(),
        "CCXLIV".to_string(),
        "CCXLV".to_string(),
        "CCXLVI".to_string(),
        "CCXLVII".to_string(),
        "CCXLVIII".to_string(),
        "CCXLIX".to_string(),
        "CCL".to_string(),
        "CCLI".to_string(),
        "CCLII".to_string(),
        "CCLIII".to_string(),
        "CCLIV".to_string(),
        "CCLV".to_string(),
        "CCLVI".to_string(),
        "CCLVII".to_string(),
        "CCLVIII".to_string(),
        "CCLIX".to_string(),
        "CCLX".to_string(),
        "CCLXI".to_string(),
        "CCLXII".to_string(),
        "CCLXIII".to_string(),
        "CCLXIV".to_string(),
        "CCLXV".to_string(),
        "CCLXVI".to_string(),
        "CCLXVII".to_string(),
        "CCLXVIII".to_string(),
        "CCLXIX".to_string(),
        "CCLXX".to_string(),
        "CCLXXI".to_string(),
        "CCLXXII".to_string(),
        "CCLXXIII".to_string(),
        "CCLXXIV".to_string(),
        "CCLXXV".to_string(),
        "CCLXXVI".to_string(),
        "CCLXXVII".to_string(),
        "CCLXXVIII".to_string(),
        "CCLXXIX".to_string(),
        "CCLXXX".to_string(),
        "CCLXXXI".to_string(),
        "CCLXXXII".to_string(),
        "CCLXXXIII".to_string(),
        "CCLXXXIV".to_string(),
        "CCLXXXV".to_string(),
        "CCLXXXVI".to_string(),
        "CCLXXXVII".to_string(),
        "CCLXXXVIII".to_string(),
        "CCLXXXIX".to_string(),
        "CCXC".to_string(),
        "CCXCI".to_string(),
        "CCXCII".to_string(),
        "CCXCIII".to_string(),
        "CCXCIV".to_string(),
        "CCXCV".to_string(),
        "CCXCVI".to_string(),
        "CCXCVII".to_string(),
        "CCXCVIII".to_string(),
        "CCXCIX".to_string(),
        "CCC".to_string(),
        "CCCI".to_string(),
        "CCCII".to_string(),
        "CCCIII".to_string(),
        "CCCIV".to_string(),
        "CCCV".to_string(),
        "CCCVI".to_string(),
        "CCCVII".to_string(),
        "CCCVIII".to_string(),
        "CCCIX".to_string(),
        "CCCX".to_string(),
        "CCCXI".to_string(),
        "CCCXII".to_string(),
        "CCCXIII".to_string(),
        "CCCXIV".to_string(),
        "CCCXV".to_string(),
        "CCCXVI".to_string(),
        "CCCXVII".to_string(),
        "CCCXVIII".to_string(),
        "CCCXIX".to_string(),
        "CCCXX".to_string(),
        "CCCXXI".to_string(),
        "CCCXXII".to_string(),
        "CCCXXIII".to_string(),
        "CCCXXIV".to_string(),
        "CCCXXV".to_string(),
        "CCCXXVI".to_string(),
        "CCCXXVII".to_string(),
        "CCCXXVIII".to_string(),
        "CCCXXIX".to_string(),
        "CCCXXX".to_string(),
        "CCCXXXI".to_string(),
        "CCCXXXII".to_string(),
        "CCCXXXIII".to_string(),
        "CCCXXXIV".to_string(),
        "CCCXXXV".to_string(),
        "CCCXXXVI".to_string(),
        "CCCXXXVII".to_string(),
        "CCCXXXVIII".to_string(),
        "CCCXXXIX".to_string(),
        "CCCXL".to_string(),
        "CCCXLI".to_string(),
        "CCCXLII".to_string(),
        "CCCXLIII".to_string(),
        "CCCXLIV".to_string(),
        "CCCXLV".to_string(),
        "CCCXLVI".to_string(),
        "CCCXLVII".to_string(),
        "CCCXLVIII".to_string(),
        "CCCXLIX".to_string(),
        "CCCL".to_string(),
        "CCCLI".to_string(),
        "CCCLII".to_string(),
        "CCCLIII".to_string(),
        "CCCLIV".to_string(),
        "CCCLV".to_string(),
        "CCCLVI".to_string(),
        "CCCLVII".to_string(),
        "CCCLVIII".to_string(),
        "CCCLIX".to_string(),
        "CCCLX".to_string(),
        "CCCLXI".to_string(),
        "CCCLXII".to_string(),
        "CCCLXIII".to_string(),
        "CCCLXIV".to_string(),
        "CCCLXV".to_string(),
        "CCCLXVI".to_string(),
        "CCCLXVII".to_string(),
        "CCCLXVIII".to_string(),
        "CCCLXIX".to_string(),
        "CCCLXX".to_string(),
        "CCCLXXI".to_string(),
        "CCCLXXII".to_string(),
        "CCCLXXIII".to_string(),
        "CCCLXXIV".to_string(),
        "CCCLXXV".to_string(),
        "CCCLXXVI".to_string(),
        "CCCLXXVII".to_string(),
        "CCCLXXVIII".to_string(),
        "CCCLXXIX".to_string(),
        "CCCLXXX".to_string(),
        "CCCLXXXI".to_string(),
        "CCCLXXXII".to_string(),
        "CCCLXXXIII".to_string(),
        "CCCLXXXIV".to_string(),
        "CCCLXXXV".to_string(),
        "CCCLXXXVI".to_string(),
        "CCCLXXXVII".to_string(),
        "CCCLXXXVIII".to_string(),
        "CCCLXXXIX".to_string(),
        "CCCXC".to_string(),
        "CCCXCI".to_string(),
        "CCCXCII".to_string(),
        "CCCXCIII".to_string(),
        "CCCXCIV".to_string(),
        "CCCXCV".to_string(),
        "CCCXCVI".to_string(),
        "CCCXCVII".to_string(),
        "CCCXCVIII".to_string(),
        "CCCXCIX".to_string(),
        "CD".to_string(),
        "CDI".to_string(),
        "CDII".to_string(),
        "CDIII".to_string(),
        "CDIV".to_string(),
        "CDV".to_string(),
        "CDVI".to_string(),
        "CDVII".to_string(),
        "CDVIII".to_string(),
        "CDIX".to_string(),
        "CDX".to_string(),
        "CDXI".to_string(),
        "CDXII".to_string(),
        "CDXIII".to_string(),
        "CDXIV".to_string(),
        "CDXV".to_string(),
        "CDXVI".to_string(),
        "CDXVII".to_string(),
        "CDXVIII".to_string(),
        "CDXIX".to_string(),
        "CDXX".to_string(),
        "CDXXI".to_string(),
        "CDXXII".to_string(),
        "CDXXIII".to_string(),
        "CDXXIV".to_string(),
        "CDXXV".to_string(),
        "CDXXVI".to_string(),
        "CDXXVII".to_string(),
        "CDXXVIII".to_string(),
        "CDXXIX".to_string(),
        "CDXXX".to_string(),
        "CDXXXI".to_string(),
        "CDXXXII".to_string(),
        "CDXXXIII".to_string(),
        "CDXXXIV".to_string(),
        "CDXXXV".to_string(),
        "CDXXXVI".to_string(),
        "CDXXXVII".to_string(),
        "CDXXXVIII".to_string(),
        "CDXXXIX".to_string(),
        "CDXL".to_string(),
        "CDXLI".to_string(),
        "CDXLII".to_string(),
        "CDXLIII".to_string(),
        "CDXLIV".to_string(),
        "CDXLV".to_string(),
        "CDXLVI".to_string(),
        "CDXLVII".to_string(),
        "CDXLVIII".to_string(),
        "CDXLIX".to_string(),
        "CDL".to_string(),
        "CDLI".to_string(),
        "CDLII".to_string(),
        "CDLIII".to_string(),
        "CDLIV".to_string(),
        "CDLV".to_string(),
        "CDLVI".to_string(),
        "CDLVII".to_string(),
        "CDLVIII".to_string(),
        "CDLIX".to_string(),
        "CDLX".to_string(),
        "CDLXI".to_string(),
        "CDLXII".to_string(),
        "CDLXIII".to_string(),
        "CDLXIV".to_string(),
        "CDLXV".to_string(),
        "CDLXVI".to_string(),
        "CDLXVII".to_string(),
        "CDLXVIII".to_string(),
        "CDLXIX".to_string(),
        "CDLXX".to_string(),
        "CDLXXI".to_string(),
        "CDLXXII".to_string(),
        "CDLXXIII".to_string(),
        "CDLXXIV".to_string(),
        "CDLXXV".to_string(),
        "CDLXXVI".to_string(),
        "CDLXXVII".to_string(),
        "CDLXXVIII".to_string(),
        "CDLXXIX".to_string(),
        "CDLXXX".to_string(),
        "CDLXXXI".to_string(),
        "CDLXXXII".to_string(),
        "CDLXXXIII".to_string(),
        "CDLXXXIV".to_string(),
        "CDLXXXV".to_string(),
        "CDLXXXVI".to_string(),
        "CDLXXXVII".to_string(),
        "CDLXXXVIII".to_string(),
        "CDLXXXIX".to_string(),
        "CDXC".to_string(),
        "CDXCI".to_string(),
        "CDXCII".to_string(),
        "CDXCIII".to_string(),
        "CDXCIV".to_string(),
        "CDXCV".to_string(),
        "CDXCVI".to_string(),
        "CDXCVII".to_string(),
        "CDXCVIII".to_string(),
        "CDXCIX".to_string(),
        "D".to_string(),
        "DI".to_string(),
        "DII".to_string(),
        "DIII".to_string(),
        "DIV".to_string(),
        "DV".to_string(),
        "DVI".to_string(),
        "DVII".to_string(),
        "DVIII".to_string(),
        "DIX".to_string(),
        "DX".to_string(),
        "DXI".to_string(),
        "DXII".to_string(),
        "DXIII".to_string(),
        "DXIV".to_string(),
        "DXV".to_string(),
        "DXVI".to_string(),
        "DXVII".to_string(),
        "DXVIII".to_string(),
        "DXIX".to_string(),
        "DXX".to_string(),
        "DXXI".to_string(),
        "DXXII".to_string(),
        "DXXIII".to_string(),
        "DXXIV".to_string(),
        "DXXV".to_string(),
        "DXXVI".to_string(),
        "DXXVII".to_string(),
        "DXXVIII".to_string(),
        "DXXIX".to_string(),
        "DXXX".to_string(),
        "DXXXI".to_string(),
        "DXXXII".to_string(),
        "DXXXIII".to_string(),
        "DXXXIV".to_string(),
        "DXXXV".to_string(),
        "DXXXVI".to_string(),
        "DXXXVII".to_string(),
        "DXXXVIII".to_string(),
        "DXXXIX".to_string(),
        "DXL".to_string(),
        "DXLI".to_string(),
        "DXLII".to_string(),
        "DXLIII".to_string(),
        "DXLIV".to_string(),
        "DXLV".to_string(),
        "DXLVI".to_string(),
        "DXLVII".to_string(),
        "DXLVIII".to_string(),
        "DXLIX".to_string(),
        "DL".to_string(),
        "DLI".to_string(),
        "DLII".to_string(),
        "DLIII".to_string(),
        "DLIV".to_string(),
        "DLV".to_string(),
        "DLVI".to_string(),
        "DLVII".to_string(),
        "DLVIII".to_string(),
        "DLIX".to_string(),
        "DLX".to_string(),
        "DLXI".to_string(),
        "DLXII".to_string(),
        "DLXIII".to_string(),
        "DLXIV".to_string(),
        "DLXV".to_string(),
        "DLXVI".to_string(),
        "DLXVII".to_string(),
        "DLXVIII".to_string(),
        "DLXIX".to_string(),
        "DLXX".to_string(),
        "DLXXI".to_string(),
        "DLXXII".to_string(),
        "DLXXIII".to_string(),
        "DLXXIV".to_string(),
        "DLXXV".to_string(),
        "DLXXVI".to_string(),
        "DLXXVII".to_string(),
        "DLXXVIII".to_string(),
        "DLXXIX".to_string(),
        "DLXXX".to_string(),
        "DLXXXI".to_string(),
        "DLXXXII".to_string(),
        "DLXXXIII".to_string(),
        "DLXXXIV".to_string(),
        "DLXXXV".to_string(),
        "DLXXXVI".to_string(),
        "DLXXXVII".to_string(),
        "DLXXXVIII".to_string(),
        "DLXXXIX".to_string(),
        "DXC".to_string(),
        "DXCI".to_string(),
        "DXCII".to_string(),
        "DXCIII".to_string(),
        "DXCIV".to_string(),
        "DXCV".to_string(),
        "DXCVI".to_string(),
        "DXCVII".to_string(),
        "DXCVIII".to_string(),
        "DXCIX".to_string(),
        "DC".to_string(),
        "DCI".to_string(),
        "DCII".to_string(),
        "DCIII".to_string(),
        "DCIV".to_string(),
        "DCV".to_string(),
        "DCVI".to_string(),
        "DCVII".to_string(),
        "DCVIII".to_string(),
        "DCIX".to_string(),
        "DCX".to_string(),
        "DCXI".to_string(),
        "DCXII".to_string(),
        "DCXIII".to_string(),
        "DCXIV".to_string(),
        "DCXV".to_string(),
        "DCXVI".to_string(),
        "DCXVII".to_string(),
        "DCXVIII".to_string(),
        "DCXIX".to_string(),
        "DCXX".to_string(),
        "DCXXI".to_string(),
        "DCXXII".to_string(),
        "DCXXIII".to_string(),
        "DCXXIV".to_string(),
        "DCXXV".to_string(),
        "DCXXVI".to_string(),
        "DCXXVII".to_string(),
        "DCXXVIII".to_string(),
        "DCXXIX".to_string(),
        "DCXXX".to_string(),
        "DCXXXI".to_string(),
        "DCXXXII".to_string(),
        "DCXXXIII".to_string(),
        "DCXXXIV".to_string(),
        "DCXXXV".to_string(),
        "DCXXXVI".to_string(),
        "DCXXXVII".to_string(),
        "DCXXXVIII".to_string(),
        "DCXXXIX".to_string(),
        "DCXL".to_string(),
        "DCXLI".to_string(),
        "DCXLII".to_string(),
        "DCXLIII".to_string(),
        "DCXLIV".to_string(),
        "DCXLV".to_string(),
        "DCXLVI".to_string(),
        "DCXLVII".to_string(),
        "DCXLVIII".to_string(),
        "DCXLIX".to_string(),
        "DCL".to_string(),
        "DCLI".to_string(),
        "DCLII".to_string(),
        "DCLIII".to_string(),
        "DCLIV".to_string(),
        "DCLV".to_string(),
        "DCLVI".to_string(),
        "DCLVII".to_string(),
        "DCLVIII".to_string(),
        "DCLIX".to_string(),
        "DCLX".to_string(),
        "DCLXI".to_string(),
        "DCLXII".to_string(),
        "DCLXIII".to_string(),
        "DCLXIV".to_string(),
        "DCLXV".to_string(),
        "DCLXVI".to_string(),
        "DCLXVII".to_string(),
        "DCLXVIII".to_string(),
        "DCLXIX".to_string(),
        "DCLXX".to_string(),
        "DCLXXI".to_string(),
        "DCLXXII".to_string(),
        "DCLXXIII".to_string(),
        "DCLXXIV".to_string(),
        "DCLXXV".to_string(),
        "DCLXXVI".to_string(),
        "DCLXXVII".to_string(),
        "DCLXXVIII".to_string(),
        "DCLXXIX".to_string(),
        "DCLXXX".to_string(),
        "DCLXXXI".to_string(),
        "DCLXXXII".to_string(),
        "DCLXXXIII".to_string(),
        "DCLXXXIV".to_string(),
        "DCLXXXV".to_string(),
        "DCLXXXVI".to_string(),
        "DCLXXXVII".to_string(),
        "DCLXXXVIII".to_string(),
        "DCLXXXIX".to_string(),
        "DCXC".to_string(),
        "DCXCI".to_string(),
        "DCXCII".to_string(),
        "DCXCIII".to_string(),
        "DCXCIV".to_string(),
        "DCXCV".to_string(),
        "DCXCVI".to_string(),
        "DCXCVII".to_string(),
        "DCXCVIII".to_string(),
        "DCXCIX".to_string(),
        "DCC".to_string(),
        "DCCI".to_string(),
        "DCCII".to_string(),
        "DCCIII".to_string(),
        "DCCIV".to_string(),
        "DCCV".to_string(),
        "DCCVI".to_string(),
        "DCCVII".to_string(),
        "DCCVIII".to_string(),
        "DCCIX".to_string(),
        "DCCX".to_string(),
        "DCCXI".to_string(),
        "DCCXII".to_string(),
        "DCCXIII".to_string(),
        "DCCXIV".to_string(),
        "DCCXV".to_string(),
        "DCCXVI".to_string(),
        "DCCXVII".to_string(),
        "DCCXVIII".to_string(),
        "DCCXIX".to_string(),
        "DCCXX".to_string(),
        "DCCXXI".to_string(),
        "DCCXXII".to_string(),
        "DCCXXIII".to_string(),
        "DCCXXIV".to_string(),
        "DCCXXV".to_string(),
        "DCCXXVI".to_string(),
        "DCCXXVII".to_string(),
        "DCCXXVIII".to_string(),
        "DCCXXIX".to_string(),
        "DCCXXX".to_string(),
        "DCCXXXI".to_string(),
        "DCCXXXII".to_string(),
        "DCCXXXIII".to_string(),
        "DCCXXXIV".to_string(),
        "DCCXXXV".to_string(),
        "DCCXXXVI".to_string(),
        "DCCXXXVII".to_string(),
        "DCCXXXVIII".to_string(),
        "DCCXXXIX".to_string(),
        "DCCXL".to_string(),
        "DCCXLI".to_string(),
        "DCCXLII".to_string(),
        "DCCXLIII".to_string(),
        "DCCXLIV".to_string(),
        "DCCXLV".to_string(),
        "DCCXLVI".to_string(),
        "DCCXLVII".to_string(),
        "DCCXLVIII".to_string(),
        "DCCXLIX".to_string(),
        "DCCL".to_string(),
        "DCCLI".to_string(),
        "DCCLII".to_string(),
        "DCCLIII".to_string(),
        "DCCLIV".to_string(),
        "DCCLV".to_string(),
        "DCCLVI".to_string(),
        "DCCLVII".to_string(),
        "DCCLVIII".to_string(),
        "DCCLIX".to_string(),
        "DCCLX".to_string(),
        "DCCLXI".to_string(),
        "DCCLXII".to_string(),
        "DCCLXIII".to_string(),
        "DCCLXIV".to_string(),
        "DCCLXV".to_string(),
        "DCCLXVI".to_string(),
        "DCCLXVII".to_string(),
        "DCCLXVIII".to_string(),
        "DCCLXIX".to_string(),
        "DCCLXX".to_string(),
        "DCCLXXI".to_string(),
        "DCCLXXII".to_string(),
        "DCCLXXIII".to_string(),
        "DCCLXXIV".to_string(),
        "DCCLXXV".to_string(),
        "DCCLXXVI".to_string(),
        "DCCLXXVII".to_string(),
        "DCCLXXVIII".to_string(),
        "DCCLXXIX".to_string(),
        "DCCLXXX".to_string(),
        "DCCLXXXI".to_string(),
        "DCCLXXXII".to_string(),
        "DCCLXXXIII".to_string(),
        "DCCLXXXIV".to_string(),
        "DCCLXXXV".to_string(),
        "DCCLXXXVI".to_string(),
        "DCCLXXXVII".to_string(),
        "DCCLXXXVIII".to_string(),
        "DCCLXXXIX".to_string(),
        "DCCXC".to_string(),
        "DCCXCI".to_string(),
        "DCCXCII".to_string(),
        "DCCXCIII".to_string(),
        "DCCXCIV".to_string(),
        "DCCXCV".to_string(),
        "DCCXCVI".to_string(),
        "DCCXCVII".to_string(),
        "DCCXCVIII".to_string(),
        "DCCXCIX".to_string(),
        "DCCC".to_string(),
        "DCCCI".to_string(),
        "DCCCII".to_string(),
        "DCCCIII".to_string(),
        "DCCCIV".to_string(),
        "DCCCV".to_string(),
        "DCCCVI".to_string(),
        "DCCCVII".to_string(),
        "DCCCVIII".to_string(),
        "DCCCIX".to_string(),
        "DCCCX".to_string(),
        "DCCCXI".to_string(),
        "DCCCXII".to_string(),
        "DCCCXIII".to_string(),
        "DCCCXIV".to_string(),
        "DCCCXV".to_string(),
        "DCCCXVI".to_string(),
        "DCCCXVII".to_string(),
        "DCCCXVIII".to_string(),
        "DCCCXIX".to_string(),
        "DCCCXX".to_string(),
        "DCCCXXI".to_string(),
        "DCCCXXII".to_string(),
        "DCCCXXIII".to_string(),
        "DCCCXXIV".to_string(),
        "DCCCXXV".to_string(),
        "DCCCXXVI".to_string(),
        "DCCCXXVII".to_string(),
        "DCCCXXVIII".to_string(),
        "DCCCXXIX".to_string(),
        "DCCCXXX".to_string(),
        "DCCCXXXI".to_string(),
        "DCCCXXXII".to_string(),
        "DCCCXXXIII".to_string(),
        "DCCCXXXIV".to_string(),
        "DCCCXXXV".to_string(),
        "DCCCXXXVI".to_string(),
        "DCCCXXXVII".to_string(),
        "DCCCXXXVIII".to_string(),
        "DCCCXXXIX".to_string(),
        "DCCCXL".to_string(),
        "DCCCXLI".to_string(),
        "DCCCXLII".to_string(),
        "DCCCXLIII".to_string(),
        "DCCCXLIV".to_string(),
        "DCCCXLV".to_string(),
        "DCCCXLVI".to_string(),
        "DCCCXLVII".to_string(),
        "DCCCXLVIII".to_string(),
        "DCCCXLIX".to_string(),
        "DCCCL".to_string(),
        "DCCCLI".to_string(),
        "DCCCLII".to_string(),
        "DCCCLIII".to_string(),
        "DCCCLIV".to_string(),
        "DCCCLV".to_string(),
        "DCCCLVI".to_string(),
        "DCCCLVII".to_string(),
        "DCCCLVIII".to_string(),
        "DCCCLIX".to_string(),
        "DCCCLX".to_string(),
        "DCCCLXI".to_string(),
        "DCCCLXII".to_string(),
        "DCCCLXIII".to_string(),
        "DCCCLXIV".to_string(),
        "DCCCLXV".to_string(),
        "DCCCLXVI".to_string(),
        "DCCCLXVII".to_string(),
        "DCCCLXVIII".to_string(),
        "DCCCLXIX".to_string(),
        "DCCCLXX".to_string(),
        "DCCCLXXI".to_string(),
        "DCCCLXXII".to_string(),
        "DCCCLXXIII".to_string(),
        "DCCCLXXIV".to_string(),
        "DCCCLXXV".to_string(),
        "DCCCLXXVI".to_string(),
        "DCCCLXXVII".to_string(),
        "DCCCLXXVIII".to_string(),
        "DCCCLXXIX".to_string(),
        "DCCCLXXX".to_string(),
        "DCCCLXXXI".to_string(),
        "DCCCLXXXII".to_string(),
        "DCCCLXXXIII".to_string(),
        "DCCCLXXXIV".to_string(),
        "DCCCLXXXV".to_string(),
        "DCCCLXXXVI".to_string(),
        "DCCCLXXXVII".to_string(),
        "DCCCLXXXVIII".to_string(),
        "DCCCLXXXIX".to_string(),
        "DCCCXC".to_string(),
        "DCCCXCI".to_string(),
        "DCCCXCII".to_string(),
        "DCCCXCIII".to_string(),
        "DCCCXCIV".to_string(),
        "DCCCXCV".to_string(),
        "DCCCXCVI".to_string(),
        "DCCCXCVII".to_string(),
        "DCCCXCVIII".to_string(),
        "DCCCXCIX".to_string(),
        "CM".to_string(),
        "CMI".to_string(),
        "CMII".to_string(),
        "CMIII".to_string(),
        "CMIV".to_string(),
        "CMV".to_string(),
        "CMVI".to_string(),
        "CMVII".to_string(),
        "CMVIII".to_string(),
        "CMIX".to_string(),
        "CMX".to_string(),
        "CMXI".to_string(),
        "CMXII".to_string(),
        "CMXIII".to_string(),
        "CMXIV".to_string(),
        "CMXV".to_string(),
        "CMXVI".to_string(),
        "CMXVII".to_string(),
        "CMXVIII".to_string(),
        "CMXIX".to_string(),
        "CMXX".to_string(),
        "CMXXI".to_string(),
        "CMXXII".to_string(),
        "CMXXIII".to_string(),
        "CMXXIV".to_string(),
        "CMXXV".to_string(),
        "CMXXVI".to_string(),
        "CMXXVII".to_string(),
        "CMXXVIII".to_string(),
        "CMXXIX".to_string(),
        "CMXXX".to_string(),
        "CMXXXI".to_string(),
        "CMXXXII".to_string(),
        "CMXXXIII".to_string(),
        "CMXXXIV".to_string(),
        "CMXXXV".to_string(),
        "CMXXXVI".to_string(),
        "CMXXXVII".to_string(),
        "CMXXXVIII".to_string(),
        "CMXXXIX".to_string(),
        "CMXL".to_string(),
        "CMXLI".to_string(),
        "CMXLII".to_string(),
        "CMXLIII".to_string(),
        "CMXLIV".to_string(),
        "CMXLV".to_string(),
        "CMXLVI".to_string(),
        "CMXLVII".to_string(),
        "CMXLVIII".to_string(),
        "CMXLIX".to_string(),
        "CML".to_string(),
        "CMLI".to_string(),
        "CMLII".to_string(),
        "CMLIII".to_string(),
        "CMLIV".to_string(),
        "CMLV".to_string(),
        "CMLVI".to_string(),
        "CMLVII".to_string(),
        "CMLVIII".to_string(),
        "CMLIX".to_string(),
        "CMLX".to_string(),
        "CMLXI".to_string(),
        "CMLXII".to_string(),
        "CMLXIII".to_string(),
        "CMLXIV".to_string(),
        "CMLXV".to_string(),
        "CMLXVI".to_string(),
        "CMLXVII".to_string(),
        "CMLXVIII".to_string(),
        "CMLXIX".to_string(),
        "CMLXX".to_string(),
        "CMLXXI".to_string(),
        "CMLXXII".to_string(),
        "CMLXXIII".to_string(),
        "CMLXXIV".to_string(),
        "CMLXXV".to_string(),
        "CMLXXVI".to_string(),
        "CMLXXVII".to_string(),
        "CMLXXVIII".to_string(),
        "CMLXXIX".to_string(),
        "CMLXXX".to_string(),
        "CMLXXXI".to_string(),
        "CMLXXXII".to_string(),
        "CMLXXXIII".to_string(),
        "CMLXXXIV".to_string(),
        "CMLXXXV".to_string(),
        "CMLXXXVI".to_string(),
        "CMLXXXVII".to_string(),
        "CMLXXXVIII".to_string(),
        "CMLXXXIX".to_string(),
        "CMXC".to_string(),
        "CMXCI".to_string(),
        "CMXCII".to_string(),
        "CMXCIII".to_string(),
        "CMXCIV".to_string(),
        "CMXCV".to_string(),
        "CMXCVI".to_string(),
        "CMXCVII".to_string(),
        "CMXCVIII".to_string(),
        "CMXCIX".to_string(),
        "M".to_string(),
    ];

    for i in 0..SIZE {
        assert_eq!(r.arabic_to_roman((i + 1) as u16), roman[i as usize]);
    }
}
