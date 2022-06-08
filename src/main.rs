use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;



fn reuse<T: Eq + std::hash::Hash + Copy + Clone>(input : Vec<T>) -> HashMap<Option<usize>, u32>{
    // hap is mapping of element to set of elements seen by that element
    let mut hap : HashMap<T, HashSet<T>> = HashMap::new();
    // result is mapping of reuse distances to occurences, None means infinity
    let mut result : HashMap<Option<usize>, u32> = HashMap::new(); 
    
    let n = input.len();  
    let mut i = 0;
    
    let mut element : T; 
    let now = Instant::now();

    result.insert(None, 0u32); //initialize occurences of infinity  to zero
    
    while i < n {
        

        element = input[i].clone();
        if !hap.contains_key(&element) { // Element hasn't been seen yet
            result.insert(None, result.get(&None).unwrap() + 1);
            hap.insert(element, HashSet::new());
            for(_, val) in hap.iter_mut() {
                val.insert(element);
            }
            
        } else {  // Element has been seen
            let count = hap.get(&element).unwrap().len();
            if !result.contains_key(&Some(count)) {
                result.insert(Some(count), 0u32);
            }
            result.insert(Some(count), result.get(&Some(count)).unwrap() + 1); 
            
            hap.get_mut(&element).unwrap().clear();
            for(_, val) in hap.iter_mut() {
                val.insert(element);
            }
        }
        i = i + 1;
       
    }
    let b = i as f64;
    println!("{:?} reuses per second", b / now.elapsed().as_secs_f64());

    result
}


fn main() {


    let m = 10000u32;
    let up : Vec<u32> = (0..m).take(m as usize).collect();
    let down : Vec<u32> = up.iter().map(|&x| m-x).collect();
    let inp = [up, down].concat().repeat(50);

    let now = Instant::now();

    let result = reuse(inp);
    println!("{} total seconds", now.elapsed().as_secs_f64());

    //let input : Vec<u32> = vec![4, 3, 4, 5, 5, 5, 6, 4];
    //println!("{:?}", reuse(input));
}


#[cfg(test)]

mod tests{
    use super::*;
    #[test] 
    fn example(){
        let input : Vec<u32> = vec![4, 3, 4, 5, 5, 5, 6, 4];
    }
}

