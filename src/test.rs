use super::*;
use helpers::{Coordinates, ShipDirection, LEN};

#[cfg(test)]
mod test {

    use super::{Coordinates, GameField, ShipDirection, LEN};

    #[test]
    fn create_field() {
        let GameField { field, .. } = super::GameField::new();
        assert_eq!(field, [[0; 12]; 12]);
    }
    #[test]
    fn get_ships() {
        let GameField { ships, .. } = super::GameField::new();
        let arr = [1, 2, 3, 4];
        let length = arr.len();
        println!("Ships {:?}", ships);
        for (i, &elem) in arr.iter().enumerate() {
            let index = length - (1 + i);
            let to_be = &arr[index];
            assert_eq!(ships.get(&elem).unwrap(), to_be);
        }
    }
    #[test]
    fn reduce_ships() {
        let mut field = super::GameField::new();
        for index in 1..5 {
            field.reduce_ships(&index);
        }
        assert_eq!(*field.ships.get(&1).unwrap(), 3);
        assert_eq!(*field.ships.get(&2).unwrap(), 2);
        assert_eq!(*field.ships.get(&3).unwrap(), 1);
        assert_eq!(*field.ships.get(&4).unwrap(), 0);
    }
    #[test]
    fn fixedcheck_permission1() {
        let mut field = super::GameField::new();
        let permission = field.check_permission(&4);
        assert_eq!(permission, true);

    }
    #[test]
    fn fixedcheck_permission2() {
        let mut field = super::GameField::new();
        field.reduce_ships(&4);
        let permission = field.check_permission(&4);
        assert_eq!(permission, false);
    }
    #[test]
    fn random_coordinates() {
        let field = super::GameField::new();
        let size = 4;
        let Coordinates {
            will_change,
            fixed,
            range,
        } = field.random_coordinates(&size);
        let expect_will_change = will_change >= 1 && will_change <= 7;
        let expect_fixed = fixed >= 1 && fixed <= 10;

        assert_eq!(expect_will_change, true);
        assert_eq!(expect_fixed, true);
        assert_eq!(range, will_change..will_change + size);
    }

    #[test]
    fn create_ship_1() {
        let mut field = super::GameField::new();
        let size = 2;
        field.create_ship(size, ShipDirection::Vertical);

        let expect: Vec<&[u8; LEN as usize]> = field
            .field
            .iter()
            .filter(|el| el.into_iter().any(|v| *v == 1))
            .collect();

        assert_eq!(expect.len(), size as usize);
    }
    #[test]
    fn create_ship_2() {
        let mut field = super::GameField::new();
        let size = 2;
        field.create_ship(size, ShipDirection::Vertical);

        let expect: Vec<&[u8; LEN as usize]> = field
            .field
            .iter()
            .filter(|el| el.into_iter().any(|v| *v == 2))
            .collect();
        println!("Expect {:?}", expect);
        let rows_contain_bounds = 4;

        assert_eq!(expect.len(), rows_contain_bounds);
    }
    #[test]
    fn create_ship_3() {
        let mut field = super::GameField::new();
        let size = 4;
        field.create_ship(size, ShipDirection::Vertical);

        match field.create_ship(size, ShipDirection::Vertical) {
            Some(_) => assert_eq!(true, false),
            None => assert_eq!(true, true),
        }

    }
}