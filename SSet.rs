//use std::fmt;
//use std::num::Float;
//so inefficient it is a crime
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
    let mut sperfects: Vec<u32> = Vec::new();   
    let mut all_sfacts: Vec<u32> = vec![1, 2];
    let mut all_offsets: Vec<OffsetInfo> = Vec::new();
    let mut threadCount: u32 = 0;
    let max = 15000;
    all_sfacts.push(1);
    all_sfacts.push(2);

    for n in 3..12000 {
        //let mut sfacts_of_n: Vec<u32> = vec![]; //create specialized vector
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
            //OFFSET: offset = sum - n
        }
    }

    let mut currentMaxVal: u32 = 12000;

    let (tx, rx) = mpsc::channel();

    while currentMaxVal < max {
        thread::sleep(Duration::from_millis(1));
        while threadCount < 8 && currentMaxVal < max {
            println!("threadcount {}", threadCount);
            if currentMaxVal+2000 > max {
                let tx1 = tx.clone();
                threadCount +=1;
                let point_all_sfacts = all_sfacts.clone();
                let point_sperfects = sperfects.clone();
            
                thread::spawn(move || {
                    tx1.send(get_s_through(currentMaxVal, max, &point_all_sfacts, &point_sperfects));
                    
                });
                thread::sleep(Duration::from_millis(1));
                currentMaxVal += ((max as i32) - (currentMaxVal as i32)).abs() as u32;
                println!("max {}", currentMaxVal);
                break;
            } else {
                let tx1 = tx.clone();
                threadCount +=1;
                let point_all_sfacts = all_sfacts.clone();
                let point_sperfects = sperfects.clone();
                
                thread::spawn(move || {
                    tx1.send(get_s_through(currentMaxVal, currentMaxVal+2000, &point_all_sfacts, &point_sperfects));
                });
                currentMaxVal+=2000;
            }
        }
    }

    for recived in rx {
        let mut newOffsets: Vec<OffsetInfo> = Vec::new();
        let mut newSPerfects: Vec<u32> = Vec::new();
        let mut newSFacts: Vec<u32> = Vec::new();
        (newSPerfects, newSFacts, newOffsets) = recived;
        all_offsets.append(&mut newOffsets);
        all_sfacts.append(&mut newSFacts);
        sperfects.append(&mut newSPerfects);
        threadCount -= 1;
        println!("threadcount{}", threadCount);

        if threadCount == 0 {
            break;
        }
    }

    println!("{:?}\n{:?}\n\n{:?}",  all_sfacts, all_offsets, sperfects);

        //Test1 : test find_factors method
    /*let factors = find_factors(12);
    println!("{:?}",factors)*/

    //Test2: tests the initial populating of s_factors
    /*println!("{:?}\n{:?}\n", sperfects, all_sfacts);*/

     //test3: check get_s_through
    /*let mut newOffsets: Vec<OffsetInfo> = Vec::new();
    let mut newSPerfects: Vec<u32> = Vec::new();
    (sperfects, all_sfacts, newOffsets) = get_s_through(121, 200, &all_sfacts, &sperfects);
    all_offsets.append(&mut newOffsets);
    println!("{:?}\n{:?}\n{:?}", sperfects, all_sfacts, all_offsets);*/
}


fn printFormat() {
    //let mut Offset_vec_of_vecs = Offset_vec_of_vecs;
    let mut Offset_vec_of_vecs: Vec<Vec<OffsetInfo>> = vec![];
}


//Job of get_s_through: return tuple off offset (iteration?) 
fn get_s_through(min: u32, max:u32, all_sfacts: &Vec<u32>, sperfects: &Vec<u32>) -> (Vec<u32>, Vec<u32>, Vec<OffsetInfo>){ //return tuple
    let mut sperfects: Vec<u32> = sperfects.to_vec();
    let mut all_sfacts: Vec<u32> = all_sfacts.to_vec();

    let mut newSPerfects: Vec<u32> = Vec::new();
    let mut newSFacts: Vec<u32> = Vec::new();
    //println!("{offs}");
    //local variables for local use... corresponding to the vars in main
    //will be populated once offset vec is done populating
    let mut offset_vec: Vec<OffsetInfo> = Vec::new(); //vec of OffsetInfo

    for n in min..max+1 {
        //let mut sfacts_of_n: Vec<u32> = vec![]; //create specialized vector
        let mut sSum: u32 = 0;
        let factors = find_factors(n); //factors of n

        for fact in factors.iter() { //goes through factors of n
            //let val: u32 = factors[*fact as usi];
            if all_sfacts.contains(&fact) { //borrow
                sSum += fact; //don't understand why star here...
            }
        }
        if ((sSum as i32) - (n as i32) <=7) && ((sSum as i32) - (n as i32)  >= -7 ) { //OFFSET
            let o = OffsetInfo{offset: (sSum as i32) - (n as i32), num: n};
            offset_vec.push(o);
            //OFFSET: offset = sum - n
        }

        //let mut sum: i32 = 0;
        /*for sfact in sfacts_of_n {
            sum += sfact as i32;
        } //find sum of sfacts_of_n*/

        //okay... offset different...
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

    //println!("{:#?}", offset_vec);
    //so now I want to go through and organize the items of struct

    for item in &offset_vec {

    } //this should populate vec_of_vecs
    
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
                //println!("{}", num);
                factors.push(num);
            } else {
                factors.push(num);
                factors.push(pair);
                //println!("{}", num);
                //println!("{}", pair);
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
 
