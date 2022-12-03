use crate::util::char_to_priority_score;

#[derive(Copy, Clone)]
pub struct Compartments<'a>(&'a str, &'a str);

impl Compartments<'_> {
    #[allow(dead_code)] // used in tests
    pub fn from_string<'a> (string: &'a String) -> Compartments<'a>  {
        assert!(string.len() % 2 == 0);
        let halfes = string.split_at(string.len()/2);
        Compartments(halfes.0, halfes.1)
    }

    pub fn from_str<'a> (str: &'a str) -> Compartments<'a>  {
        assert!(str.len() % 2 == 0);
        let halfes = str.split_at(str.len()/2);
        Compartments(halfes.0, halfes.1)
    }

    pub fn get_priority(self: &Self) -> Option<u8> {
        self.find_duplicated_char().map(|c| char_to_priority_score(c).unwrap())
    }

    fn find_duplicated_char(self) -> Option<char> {
        for c1 in self.0.chars() {
            if let Some(found) = self.1.chars().find(|&c2| c2 == c1) {
                return Some(found);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::Compartments;

    #[test]
    fn splits_string_in_two_equally_big_compartments() {
        let string: String = String::from("vJrwpWtwJgWrhcsFMMfFFhFp");

        let compartments = Compartments::from_string(&string);

        assert_eq!(compartments.0, "vJrwpWtwJgWr");
        assert_eq!(compartments.1, "hcsFMMfFFhFp");
    }

    #[test]
    fn p_has_priority_16() {
        let string: String = String::from("vJrwpWtwJgWrhcsFMMfFFhFp"); // p is a duplicate in both compartments!
        let compartments = Compartments::from_string(&string);

        let priority = compartments.get_priority();

        assert_eq!(priority.unwrap(), 16);
    }

    #[test]
    fn uppercase_l_has_priority_38() {
        let string: String = String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"); // L is a duplicate in both compartments!
        let compartments = Compartments::from_string(&string);

        let priority = compartments.get_priority();

        assert_eq!(priority.unwrap(), 38);
    }
    
    #[test]
    fn uppercase_p_has_priority_42() {
        let string: String = String::from("PmmdzqPrVvPwwTWBwg"); // P is a duplicate in both compartments!
        let compartments = Compartments::from_string(&string);

        let priority = compartments.get_priority();

        assert_eq!(priority.unwrap(), 42);
    }
    
    #[test]
    fn v_has_priority_22() {
        let string: String = String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"); // v is a duplicate in both compartments!
        let compartments = Compartments::from_string(&string);

        let priority = compartments.get_priority();

        assert_eq!(priority.unwrap(), 22);
    }
    
    #[test]
    fn t_has_priority_20() {
        let string: String = String::from("ttgJtRGJQctTZtZT"); // t is a duplicate in both compartments!
        let compartments = Compartments::from_string(&string);

        let priority = compartments.get_priority();

        assert_eq!(priority.unwrap(), 20);
    }
    
    #[test]
    fn s_has_priority_20() {
        let string: String = String::from("CrZsJsPPZsGzwwsLwLmpwMDw"); // s is a duplicate in both compartments!
        let compartments = Compartments::from_string(&string);

        let priority = compartments.get_priority();

        assert_eq!(priority.unwrap(), 19);
    }
    
    #[test]
    fn a_has_priority_1() {
        let string: String = String::from("aa");
        let compartments = Compartments::from_string(&string);

        let priority = compartments.get_priority();

        assert_eq!(priority.unwrap(), 1);
    }
    
    #[test]
    fn uppercase_a_has_priority_27() {
        let string: String = String::from("AA");
        let compartments = Compartments::from_string(&string);

        let priority = compartments.get_priority();

        assert_eq!(priority.unwrap(), 27);
    }
    
    #[test]
    fn z_has_priority_26() {
        let string: String = String::from("zz");
        let compartments = Compartments::from_string(&string);

        let priority = compartments.get_priority();

        assert_eq!(priority.unwrap(), 26);
    }
    
    #[test]
    fn uppercase_z_has_priority_52() {
        let string: String = String::from("ZZ");
        let compartments = Compartments::from_string(&string);

        let priority = compartments.get_priority();

        assert_eq!(priority.unwrap(), 52);
    }
}