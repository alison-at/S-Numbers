package main

import (
	"fmt"
	"sync"
	"time"
	"math"
)

type OffsetInfo struct {
	offset int
	num int
}

var (
	mu sync.Mutex
	sFactors []int
	SPerfect []int
	iterating int
	counter int
	currentBar int
	allOffset []OffsetInfo
)

func printFormat() {
	var nums [15][]int

	for idx := range allOffset {
		currentOff := allOffset[idx].offset
		nums[currentOff+7] = append(nums[currentOff+7], allOffset[idx].num)
	}

	fmt.Println("\nDeficient (In S)")
	for x := 0; x <7; x++ {
		fmt.Println(x-7," -->", nums[x])
	}

	fmt.Println("\nPerfect (in S)\n0 --> ", SPerfect)

	fmt.Println("\nAbundant (In S)")
	for y := 8; y < 15; y++ {
		fmt.Println(y-7," -->", nums[y])
	}
}

func factoring(x int) []int{
	var facts []int
	for i := 1; float64(i) <= math.Sqrt(float64(x)); i++ {
		if (x%i == 0) {
			facts = append(facts, i)
			if (x/i != i && i != 1){
				y := x/i;
				facts = append(facts, y)
			}
		}
	}
	return facts
}

func sum(nums []int) int {
	total := 0

	for _,num := range nums {
		total += num
	}
	fmt.Println(total)
	return total
}

func contains(s []int, e int) bool {
    for _,a := range s {
        if a == e {
            return true
        }
    }
    return false
}

func updateSPerfect(newS []int, newSPerfect []int, iteratNum int, newOff []OffsetInfo) {
	defer mu.Unlock()
	mu.Lock()
	sFactors = append(sFactors, newS...)
	SPerfect = append(SPerfect, newSPerfect...)
	allOffset = append(allOffset, newOff...)
	iterating += iteratNum
	counter--
}
//change what you append to!!
func iteration(sFactors []int, start int, end int, max int, wg *sync.WaitGroup)  {
	defer wg.Done()

	var newSNums []int
	var newSPer []int
	var newOff []OffsetInfo

	iteratNum := 0
	for i := start; i <= end && i <= max; i++ {
		var currentFa = factoring(i)
		currentSum :=0

		for f := range currentFa{
			if contains(sFactors, currentFa[f]) {
				currentSum += currentFa[f]
			}
		}
		
		if 	currentSum <= i {
			newSNums = append(newSNums, i)
		}

		if (math.Abs(float64( currentSum - i) ) <=7) {
			o := OffsetInfo{offset: currentSum - i, num: i}
			newOff = append(newOff, o)
		}

		if currentSum == i {
			newSPer = append(newSPer, i)
		}
	
	 	iteratNum++;
		//fmt.Println("i", i, "start", start, "end", end, "sum", currentSum)
	}
	updateSPerfect(newSNums, newSPer, iteratNum, newOff)
}

func main() {
	//very weird: will only sometimes include 114688
	//also start and end and i do not go to max
	iterating = 2;
	counter = 0;
	sFactors = append(sFactors, 1, 2)
	sFactors = append(sFactors, 2)
	var wg sync.WaitGroup
	max := 1500000

	//generate Sfactors and Sperfect up to 12000
	for i := 3; i <= 12000 && i <= max; i++ {
		var currentFa = factoring(i)
		currentSum :=0
		for j := range currentFa{
			if contains(sFactors, currentFa[j]) {
				currentSum += currentFa[j]
			}
		}
		if 	currentSum <= i {
			sFactors = append(sFactors, i)
		}

		if (math.Abs(float64( currentSum - i) ) <=7) {
			o := OffsetInfo{offset: currentSum - i, num: i}
			allOffset = append(allOffset, o)
		}

		if currentSum == i {
			SPerfect = append(SPerfect, i)
		}

		iterating++
	}

	if iterating == max {
		iterating ++
	}

	currentBar = iterating + 2
	
	//start building by increments of 120
	//use waitloop
	for currentBar < max{
		time.Sleep(1*time.Millisecond)//catch up on counting
		
		for counter < 8 && currentBar < max{
			wg.Add(1)
			counter++
			
			if currentBar+2000 > max {
				currentBar += (max - currentBar)
				go iteration(sFactors, currentBar-1999, max, max, &wg)
				wg.Wait()
				break;
			} else {
				currentBar +=2000
				go iteration(sFactors, currentBar-1999, currentBar, max, &wg)
			}
			fmt.Println(counter, "counter", currentBar, "maxVal")
		}
	}
	wg.Wait()
	//fmt.Println(sFactors)
	fmt.Println(SPerfect)
	printFormat()
}

/* hard code 0 to 120,000 with main thread
	I could get end to == max if it is more than max and then get rid of iterator
	
	//lock and unlock sset when sset is changed, not for reading. Keep copy with thread, 
	//disapears with thread. should n be in s? reads sset copy, use function to put local into global
	wg.Add(1)
		counter++
		go iteration(sFactors, currentBar+1, currentBar+120, max, &wg)
		currentBar += 120
*/