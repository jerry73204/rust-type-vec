use type_vec::{Dyn, Vect};
use typenum::consts::*;

#[test]
fn static_vec_test() {
    let vec: Vect<usize, U0> = Vect::new();

    // remove or pop on empty vec, none of these compile
    /*
        let vec = vec.pop();
        let vec = vec.remove(1);
        let vec = vec.remove(U1::new());
    */

    // push
    let vec: Vect<usize, U1> = vec.push(3);
    let vec: Vect<usize, U2> = vec.push(1);
    let vec: Vect<usize, U3> = vec.push(4);

    // insert by static index
    let vec: Vect<usize, U4> = vec.insert(U0::new(), 7);
    let vec: Vect<usize, U5> = vec.insert(U4::new(), 5);
    /* this should not compile
        let vec: Vect<usize, U6> = vec.insert(U6::new(), 1);
    */

    // insert by dynamic index
    let vec: Vect<usize, U6> = vec.insert(5, 6);
    /* this causes runtime panic
       let vec: Vect<usize, U7> = vec.insert(10, 6);
    */

    // remove by static index
    let vec: Vect<usize, U5> = vec.remove(U5::new());
    /* this should not compile
        let vec: Vect<usize, U4> = vec.remove(U5::new());
    */

    // remove by runtime index
    let vec: Vect<usize, U4> = vec.remove(4);

    // get by static index
    let elem: &usize = vec.get(U0::new());
    assert_eq!(elem, &7);
    let elem: &usize = vec.get(U1::new());
    assert_eq!(elem, &3);
    let elem: &usize = vec.get(U2::new());
    assert_eq!(elem, &1);
    let elem: &usize = vec.get(U3::new());
    assert_eq!(elem, &4);
    /* this should not compile
        let elem: &usize = vec.get(U4::new());
    */

    // get by dynamic index
    let elem: Option<&usize> = vec.get(0);
    assert_eq!(elem, Some(&7));
    let elem: Option<&usize> = vec.get(1);
    assert_eq!(elem, Some(&3));
    let elem: Option<&usize> = vec.get(2);
    assert_eq!(elem, Some(&1));
    let elem: Option<&usize> = vec.get(3);
    assert_eq!(elem, Some(&4));
    let elem: Option<&usize> = vec.get(4);
    assert_eq!(elem, None);

    // pop
    let (vec, elem): (Vect<usize, U3>, usize) = vec.pop();
    assert_eq!(elem, 4);
    let (vec, elem): (Vect<usize, U2>, usize) = vec.pop();
    assert_eq!(elem, 1);

    //into_dyn
    let vec: Vect<usize, Dyn> = vec.into_dyn();
    let vec: Vect<usize, Dyn> = vec.push(2);
    let vec: Vect<usize, Dyn> = vec.push(5);

    // get by static index on Vect<T, Dyn>
    let elem: Option<&usize> = vec.get(U0::new());
    assert_eq!(elem, Some(&7));
    let elem: Option<&usize> = vec.get(U1::new());
    assert_eq!(elem, Some(&3));
    let elem: Option<&usize> = vec.get(U2::new());
    assert_eq!(elem, Some(&2));
    let elem: Option<&usize> = vec.get(U3::new());
    assert_eq!(elem, Some(&5));
    let elem: Option<&usize> = vec.get(U4::new());
    assert_eq!(elem, None);

    // get by dynamic index on Vect<T, Dyn>
    let elem: Option<&usize> = vec.get(0);
    assert_eq!(elem, Some(&7));
    let elem: Option<&usize> = vec.get(1);
    assert_eq!(elem, Some(&3));
    let elem: Option<&usize> = vec.get(2);
    assert_eq!(elem, Some(&2));
    let elem: Option<&usize> = vec.get(3);
    assert_eq!(elem, Some(&5));
    let elem: Option<&usize> = vec.get(4);
    assert_eq!(elem, None);
}
