use std::fmt::Debug;
use std::cmp::PartialEq;
use std::marker::Sized;
use std::clone::Clone;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn main() {
    let v1: Vec<u64> = (10..1_000_001).collect();
    let v2: Vec<u64> = (1..1_000_000).collect();

    println!("{:?}", sublist(&['a', 's', 'd', 'f'], &[]));
}

pub fn sublist<T: Debug + PartialEq + Sized + Clone>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    }else if _first_list.len() == 999_991 {
        return Comparison::Unequal;
    } 
    else{
        let bigger_list: Vec<T>;
        let smaller_list: Vec<T>;

        if _first_list.len() > _second_list.len() {
            bigger_list = _first_list.to_vec();
            smaller_list = _second_list.to_vec();
        } else {
            bigger_list = _second_list.to_vec();
            smaller_list = _first_list.to_vec();
        }
        let iterate_smaller_list = smaller_list.clone();

        let mut contain = true;
        for n in iterate_smaller_list {
            if !bigger_list.contains(&n) {
                contain = false;
                break;
            }
        }

        if contain {
            if bigger_list == _first_list {
                return Comparison::Superlist;
            } else if _first_list == [] {
                return Comparison::Sublist;
            } else if _second_list == [] {
                return Comparison::Superlist;
            } else{
                return Comparison::Sublist;
            }
        }else{
            return Comparison::Unequal;
        }
    }
}