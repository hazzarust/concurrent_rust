mod threads;

use threads::one;
use crate::threads::leak;
use crate::threads::data_race;
fn main(){
    let a = 2;
    let mut b = 2;
    data_race(&a, &mut b);
}