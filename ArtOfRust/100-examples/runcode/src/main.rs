

mod closures;
mod lifetimes;
mod reference_counters;
mod code;
mod iterators;

fn main() {
    iterators::method_that_produce_other_iterators();
}

#[cfg(test)]
mod closure_that_capture_envionment {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            iterators::Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            iterators::Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            iterators::Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = iterators::shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                iterators::Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                iterators::Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}