//use std::fmt;
//use std::num::Float;
//so inefficient it is a crime
//so ugly it is sad
//does update
use std::env;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct OffsetInfo {
    offset: i32,
    num: u32,
} //offset info struct

/*Job of main: 
have vector of sperfect 
have vector of offset vectors
have vector of all s factors
updte this based on input from thread messages
*/
fn main() {
    /*the issue: second while loop is only exited when current val is over max, that is when messages would go to rx
    */
    let mut sperfects: Vec<u32> = Vec::new();   
    let mut all_sfacts: Vec<u32> = vec![1, 2];
    let mut all_offsets: Vec<OffsetInfo> = Vec::new();
    let mut threadCount: u32 = 0;
    let args: Vec<String> = env::args().collect(); // a slice / vector
    let val:u32 = args[1].trim().parse().unwrap();
    let max = val;
    all_sfacts.push(1);
    all_sfacts.push(2);

    for n in 3..20001 {
        let factors = find_factors(n);
        let mut sSum: u32 = 0;
        for i in 0..factors.len() {
            if all_sfacts.contains(&factors[i]) {
                sSum += factors[i]
            }
        }

        if sSum == n  {
            sperfects.push(n);
            all_sfacts.push(n);
        }
        else if sSum < n {
            all_sfacts.push(n);
        }

        if ((sSum as i32) - (n as i32) <=7) && ((sSum as i32) - (n as i32)  >= -7 ) { //OFFSET
            let o = OffsetInfo{offset: (sSum as i32) - (n as i32), num: n};
            all_offsets.push(o);
        }
    }

    let mut currentMaxVal: u32 = 20000;
    let (tx, rx) = mpsc::channel();
    
    while currentMaxVal < max {
        thread::sleep(Duration::from_millis(10));
        while threadCount < 8 && currentMaxVal < max {
            println!("threadcount {}", threadCount);
            let tx1 = tx.clone();
            threadCount +=1;
            let point_all_sfacts = all_sfacts.clone();
            let point_sperfects = sperfects.clone();

            if currentMaxVal+3000 >= max {
                let former = currentMaxVal;
                currentMaxVal = max;
                println!("maxVal {}", currentMaxVal);
                (thread::spawn(move || {
                    tx1.send(get_s_through(former, max, &point_all_sfacts, &point_sperfects));
                }));
            } else {
                currentMaxVal += 3000;
                println!("currmaxVal {}", currentMaxVal);
                (thread::spawn(move || {
                    tx1.send(get_s_through(currentMaxVal-2999, currentMaxVal, &point_all_sfacts, &point_sperfects));
                }));   
            }   
        }

        for recived in &rx {
            threadCount -= 1;
            println!("threadcount!!{}", threadCount);
            let mut new_offsets: Vec<OffsetInfo> = Vec::new();
            let mut new_s_perfects: Vec<u32> = Vec::new();
            let mut new_s_facts: Vec<u32> = Vec::new();
            (new_s_perfects, new_s_facts, new_offsets) = recived;
            all_offsets.append(&mut new_offsets);
            all_sfacts.append(&mut new_s_facts);
            sperfects.append(&mut new_s_perfects);
            
            if threadCount == 0 {
                break;
            }
        }


        /*if currentMaxVal >= max {
            println!("finally");
            break;
        }*/
    }
    /*
    for recived in rx {
        threadCount -= 1;
        println!("threadcount!!{}", threadCount);
        let mut newOffsets: Vec<OffsetInfo> = Vec::new();
        let mut newSPerfects: Vec<u32> = Vec::new();
        let mut newSFacts: Vec<u32> = Vec::new();
        (newSPerfects, newSFacts, newOffsets) = recived;
        all_offsets.append(&mut newOffsets);
        all_sfacts.append(&mut newSFacts);
        sperfects.append(&mut newSPerfects);
        
        if threadCount == 0 {
            break;
        }
    }*/
    
    //println!("{:?}\n{:?}\n\n{:?}",  all_sfacts, all_offsets, sperfects);
    
    println!("{:?}", sperfects);
    
    //working offset here!!
    let mut placeholder: Vec<i32> = Vec::new();
    let mut Offset_vec: Vec<Vec<i32>> = vec![placeholder; 15];
    for i in all_offsets.iter() {

        Offset_vec[(i.offset + 7) as usize].push((i.num) as i32);
        //println!("vec: {:?}",  Offset_vec);
    }

    println!("\nDeficient (In S)");
    for j in 0..7 {
        println!("{} -> {:?}", (j as i32)-7, Offset_vec[j]);
    }

    println!("\nPerfect (In S)");
    println!("{} -> {:?}", 0, Offset_vec[7]);

    println!("\nAbundant (In S)");
    for j in 8..15 {
        println!("{} -> {:?}", (j as i32)-7, Offset_vec[j]);
    }


    
}

//Job of get_s_through: return tuple off offset (iteration?) 
fn get_s_through(min: u32, max:u32, all_sfacts: &Vec<u32>, sperfects: &Vec<u32>) -> (Vec<u32>, Vec<u32>, Vec<OffsetInfo>){ //return tuple
    let mut sperfects: Vec<u32> = sperfects.to_vec();
    let mut all_sfacts: Vec<u32> = all_sfacts.to_vec();

    let mut newSPerfects: Vec<u32> = Vec::new();
    let mut newSFacts: Vec<u32> = Vec::new();
    let mut offset_vec: Vec<OffsetInfo> = Vec::new(); //vec of OffsetInfo

    for n in min..max+1 {
        //println!(" iter {} min {} max {}",n, min, max);
        let mut sSum: u32 = 0;
        let factors = find_factors(n); //factors of n

        for fact in factors.iter() { //goes through factors of n
            if all_sfacts.contains(&fact) { //borrow
                sSum += fact; //don't understand why star here...
            }
        }
        if ((sSum as i32) - (n as i32) <=7) && ((sSum as i32) - (n as i32)  >= -7 ) { //OFFSET
            let o = OffsetInfo{offset: (sSum as i32) - (n as i32), num: n};
            offset_vec.push(o);
            //OFFSET: offset = sum - n
        }

        if sSum == n  {
            //println!("{}",n);
            newSPerfects.push(n);
            newSFacts.push(n);
            //println!("{:?}\n{:?}\n", sperfects, all_sfacts);
        }

        if sSum  < n  {  //this and above are if is Negative...
            newSFacts.push(n);
        }
    }
    //println!("Done with range \n\n");
    return (newSPerfects, newSFacts,  offset_vec); //tuple, just to see/ test

}
 
//finds factors: ((x as f64).sqrt() as u32 to square root
fn find_factors(x: u32)-> Vec<u32> { //helper func... not called by main
    let mut factors: Vec<u32> = Vec::new();
    //take the sqare root, add one in case sqaree root is a decimal and gets rounded down when turned to u32
    for num in 1..(x as f32).sqrt() as u32 +1 {
        if x % num == 0  {
            let pair = x/num;

            if pair == num || num ==1 {
                factors.push(num);
            } else {
                factors.push(num);
                factors.push(pair);
            }
        }
    }
    return factors;
}

fn sum(nums: &Vec<u32>) -> u32 {
    let mut sum: u32 = 0;
    for i in 0..nums.len() {
        sum += nums[i]
    }

    return sum
}

fn contains(vec: &Vec<u32>,x: u32 ) -> bool {
    for n in 0..vec.len() {
        if x == vec[n] {
            return true
        }
    }
    return false
}
 
