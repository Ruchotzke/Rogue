
#[cfg(test)]
mod tests{

    #[test]
    fn should_work(){
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn should_not_work(){
        let result = 2 + 3;
        assert_eq!(result, 4);
    }

}