#![allow(unused_variables, dead_code)]

/// Возвращает более длинную строку по числу UTF-8 байтов; при равенстве — left.
pub fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    todo!("связать результат с двумя входными ссылками")
}

/// Заимствованное представление текста.
#[derive(Debug, Clone, Copy)]
pub struct TextView<'a> {
    text: &'a str,
}

impl<'a> TextView<'a> {
    pub fn new(text: &'a str) -> Self {
        todo!("сохранить ссылку без выделения String")
    }

    /// Возвращает первую строку исходного текста без CRLF-терминатора.
    /// Время жизни результата связано с текстом, а не с самим TextView.
    pub fn first_line(&self) -> &'a str {
        todo!("вернуть срез исходного текста")
    }

    pub fn into_inner(self) -> &'a str {
        todo!("вернуть исходную ссылку")
    }
}

/// Возвращает строки text, содержащие needle, в исходном порядке.
pub fn lines_containing<'a>(text: &'a str, needle: &str) -> Vec<&'a str> {
    todo!("собрать заимствованные строки")
}
