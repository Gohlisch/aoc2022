use std::ops::Add;

pub fn highest_value<T>(value_vector: &mut Vec<T>) -> Option<&T> 
where
    T: Add<Output = T> + Ord
{
    value_vector.sort_unstable_by(|a, b| b.cmp(a));
    value_vector.get(0)
}

pub fn sum_of_top_3<T>(value_vector: &mut Vec<T>) -> Option<T>
where
    T: Add<Output = T> + Ord + Copy
{
    if value_vector.len() < 3 {
        return None;
    }
    value_vector.sort_unstable_by(|a, b| b.cmp(a));
    
    Option::from(value_vector[0] + value_vector[1] + value_vector[2])
}



#[cfg(test)]
mod tests {
    use crate::{*, util::{highest_value, sum_of_top_3}};

    #[test]
    fn highest_value_gets_highest_value() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        let mut calories_vector = get_calories_vector(input.to_string());

        let result = highest_value(&mut calories_vector);

        assert_eq!(result.unwrap(), 24000);
    }

    #[test]
    fn highest_value_returns_none_if_vector_is_empty() {
        let mut calories_vector: Vec<i32> = vec![];

        let result = highest_value(&mut calories_vector);

        assert!(result.is_none());
    }

    #[test]
    fn sum_of_top_3_works() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        let mut calories_vector = get_calories_vector(input.to_string());

        let result = sum_of_top_3(&mut calories_vector);

        assert_eq!(result.unwrap(), 45000);
    }


    #[test]
    fn sum_of_top_3_returns_none_if_vector_is_bellow_len_3() {
        let mut calories_vector = vec![42, 1337];

        let result = sum_of_top_3(&mut calories_vector);

        assert!(result.is_none());
    }
}