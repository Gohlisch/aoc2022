pub fn char_to_priority_score(c: char) -> Option<u8> {
    match c {
        'a'..='z' => Some((c as u8) - 96),
        'A'..='Z' => Some((c as u8) - 38),
        _ => None
    }
}

pub fn next_three_as_tuple<I, T>(iterator:&mut I) -> Result<(T, T, T), i32> 
where
    I: Iterator<Item = T>,
{
    if let Some(a) = iterator.next()  {
        if let Some(b) = iterator.next() {
            if let Some(c) = iterator.next() {
                return Ok((a,b,c));
            }
            return Err(2)
        }
        return Err(1);
    }
    return Err(0);          
}