//use std::fmt;
//use std::num::Float;
//UNCOMPLETE

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
    let mut Offset_vec_of_vecs: Vec<Vec<OffsetInfo>> = vec![];
    let max = 100;

    all_sfacts.push(1);
    all_sfacts.push(2);


    for n in 3..120 {
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
        
    }

    //Test1 : test find_factors method
    /*let factors = find_factors(12);
    println!("{:?}",factors)*/

    //Test2: tests the initial populating of s_factors
    /*println!("{:?}\n{:?}\n", sperfects, all_sfacts)*/
}

fn printFormat(Offset_vec_of_vecs: Vec<Vec<OffsetInfo>>) {
    let mut Offset_vec_of_vecs = Offset_vec_of_vecs;
}

/*
//Job of get_s_through: return tuple off offset (iteration?)
fn get_s_through(min: u32, max:u32)-> (Vec<u32>, Vec<OffsetInfo>) { //return tuple

    //println!("{offs}");
    //local variables for local use... corresponding to the vars in main


    //will be populated once offset vec is done populating

    let mut offset_vec: Vec<OffsetInfo> = Vec::new(); //vec of OffsetInfo


    for n in min..max {
        let mut sfacts_of_n: Vec<u32> = vec![]; //create specialized vector
        let factors = find_factors(n); //factors of n

        for fact in facts.iter() { //goes through factors of n

            if all_sfacts.contains(&fact) { //borrow
                sfacts_of_n.push(*fact); //don't understand why star here...
            } //else...
        }

        let mut sum: i32 = 0;

        for sfact in sfacts_of_n {

            sum += sfact as i32;

        } //find sum of sfacts_of_n

        //okay... offset different...
        if sum - n as i32 == 0  {

            sperfects.push(n);

            all_sfacts.push(n);
        }

        //for offset

        if (sum - n as i32 <=7) && (sum - n as i32 >= -7) { //OFFSET -
            let o = OffsetInfo{offset: sum - n as i32, num: n};

            //println!("{:?}", o); //must print first...

            offset_vec.push(o);

            //OFFSET: offset = sum - n
        }

        if sum < n as i32 {  //this and above are if is Negative...

            all_sfacts.push(n);
        }

    }

    println!("{:#?}", offset_vec);
    //so now I want to go through and organize the items of struct

    for item in offset_vec {

        //append num to vec corresponding to offset

        //vec[0]

        //offset 

        //recieving vec: vec_of_vecs[item.offset]

        //being pushed: item.num

        vec_of_vecs[item.offset].push(item.num);

        //push onto vec corresponding with offset

    } //this should populate vec_of_vecs

    return (sperfects, offset_vec); //tuple, just to see/ test

}*/
 
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
 
