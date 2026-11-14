#![allow(unused_variables)]

/// Потребляет строку, возвращает её без пробелов по краям в верхнем регистре.
pub fn normalize_title(title: String) -> String {
    todo!("преобразовать owned String без повторного параметра")
}

/// Возвращает исходную строку и её длину в байтах.
pub fn take_and_return(text: String) -> (String, usize) {
    todo!("вернуть владение строкой вместе с длиной")
}

/// Создаёт две независимые копии заголовка.
pub fn duplicate_title(title: String) -> (String, String) {
    todo!("явно клонировать данные там, где нужны два владельца")
}
