use crate::util::char_to_priority_score;

#[derive(Copy, Clone)]
pub struct Group<'a>(&'a str, &'a str, &'a str);

impl Group<'_> {
    pub fn from_tuple<'a> (tuple: (&'a str, &'a str, &'a str)) -> Group<'a>  {
        Group(tuple.0, tuple.1, tuple.2)
    }

    pub fn get_badge_priority(self: &Self) -> Option<u8> {
        self.find_badge_char().map(|c| char_to_priority_score(c).unwrap())
    }

    fn find_badge_char(self) -> Option<char> {
        for c0 in self.0.chars() {
            if self.1.chars().find(|&c1| c1 == c0).is_some() && self.2.chars().find(|&c2| c2 == c0).is_some() {
                return Some(c0);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::Group;

    #[test]
    fn get_badge_priorities_1() {
        let group = Group("vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg");

        let badge_priority = group.get_badge_priority();

        assert_eq!(badge_priority.unwrap(), 18)
    } 

    #[test]
    fn get_badge_priorities_2() {
        let group = Group("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "ttgJtRGJQctTZtZT", "CrZsJsPPZsGzwwsLwLmpwMDw");

        let badge_priority = group.get_badge_priority();

        assert_eq!(badge_priority.unwrap(), 52)
    } 
}